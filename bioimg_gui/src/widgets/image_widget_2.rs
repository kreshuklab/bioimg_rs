use std::borrow::Borrow;
use std::sync::Arc;
use std::io::Cursor;
use std::error::Error;

use image::GenericImageView;
use parking_lot as pl;
use bioimg_runtime::{self as rt, FileSource};

use crate::{project_data::{ImageWidget2LoadingStateRawData, ImageWidget2RawData, SpecialImageWidgetRawData}, result::{GuiError, Result}};
use super::{Restore, StatefulWidget, ValueWidget};
use super::error_display::show_error;
use super::file_source_widget::FileSourceWidget;
use super::util::DynamicImageExt;

pub type ArcDynImg = Arc<image::DynamicImage>;
pub type Generation = i64;

pub struct Texture{
    name: String,
    context: egui::Context,
    handle: egui::TextureHandle,
}

impl Texture{
    pub fn load(img: &image::DynamicImage, context: egui::Context) -> Self{
        let texture_name: String = uuid::Uuid::new_v4().to_string();
        let texture_handle = img.to_egui_texture_handle(&texture_name, &context);
        Self{
            name: texture_name,
            context,
            handle: texture_handle,
        }
    }
    pub fn show(&self, ui: &mut egui::Ui, display_size: egui::Vec2){
        let ui_img = egui::Image::new(
            egui::ImageSource::Texture(
                egui::load::SizedTexture {
                    id: self.handle.id(),
                    size: display_size,
                }
            )
        );
        ui.add(ui_img);
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        self.context.forget_image(&self.name);
    }
}

#[derive(Default)]
enum LoadingState{
    #[default]
    Empty,
    Loading{source: rt::FileSource},
    Ready{source: rt::FileSource, img: ArcDynImg, texture: Option<Texture>},
    Forced{img: ArcDynImg, texture: Option<Texture>},
    Failed{source: rt::FileSource, err: GuiError},
}

impl LoadingState{
    pub fn file_source(&self) -> Option<&FileSource>{
        match self {
            Self::Empty => None,
            Self::Loading { source } => Some(source),
            Self::Ready { source, ..} => Some(source),
            Self::Forced { .. } => None,
            Self::Failed { source, .. } => Some(source)
        }
    }
    pub fn is_loading(&self) -> bool{
        matches!(self, Self::Loading{..})
    }
}

pub struct ImageWidget2{
    file_source_widget: FileSourceWidget,
    loading_state: Arc<pl::Mutex<(Generation, LoadingState)>>,
}

impl Default for ImageWidget2{
    fn default() -> Self {
        Self{
            file_source_widget: Default::default(),
            loading_state: Arc::new(pl::Mutex::new((0, Default::default())))
        }
    }
}

impl Restore for ImageWidget2{
    type RawData = ImageWidget2RawData;
    fn dump(&self) -> Self::RawData {
        let loading_state_guard = self.loading_state.lock();
        let loading_state: &(_, LoadingState) = &*loading_state_guard;
        ImageWidget2RawData{
            file_source_widget: self.file_source_widget.dump(),
            loading_state: match &loading_state.1{
                LoadingState::Forced{img, ..} => {
                    let mut raw_out = Vec::<u8>::new();
                    if let Err(err) = img.write_to(&mut Cursor::new(&mut raw_out), image::ImageFormat::Png){
                        eprintln!("[WARNING] Could not save pathless image: {err}");
                    }
                    ImageWidget2LoadingStateRawData::Forced { img_bytes: raw_out }
                },
                _ => ImageWidget2LoadingStateRawData::Empty,
            }
        }
    }
    fn restore(&mut self, raw: Self::RawData) {
        self.file_source_widget.restore(raw.file_source_widget);
        let loading_state = match raw.loading_state{
            ImageWidget2LoadingStateRawData::Empty => LoadingState::Empty,
            ImageWidget2LoadingStateRawData::Forced { img_bytes } => 'forced: {
                let Ok(reader) = image::io::Reader::new(Cursor::new(img_bytes)).with_guessed_format() else {
                    eprintln!("[WARNING] Could not guess format of saved image");
                    break 'forced LoadingState::Empty;
                };
                let Ok(image) = reader.decode() else {
                    eprintln!("[WARNING] Could not decoded saved image");
                    break 'forced LoadingState::Empty;
                };
                LoadingState::Forced { img: Arc::new(image), texture: None }
            }
        };
        self.loading_state = Arc::new(pl::Mutex::new((0, loading_state)));
    }
}

impl ValueWidget for ImageWidget2{
    type Value<'v> = (Option<rt::FileSource>, Option<ArcDynImg>);

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        match value{
            (None, Some(img)) => {
                self.file_source_widget = Default::default();
                self.loading_state = Arc::new(pl::Mutex::new((0, LoadingState::Forced { img, texture: None})));
            },
            (None, None) => {
                self.file_source_widget = Default::default();
                self.loading_state = Arc::new(pl::Mutex::new((0, LoadingState::Empty)));
            },
            (Some(file_source), _) => {
                self.file_source_widget.set_value(file_source);
                self.loading_state = Arc::new(pl::Mutex::new((0, LoadingState::Empty)));
            }
        }
        // self.update(); //FIXME: call once set_value takes a context
    }
}

impl ImageWidget2{
    pub fn update(&self, ctx: &egui::Context){
        if self.loading_state.lock().1.is_loading(){ //FIXME: only on loading?
            ctx.request_repaint();
        }
    }
    fn spawn_load_image_task(
        generation: Generation,
        file_source: FileSource,
        loading_state: Arc<pl::Mutex<(Generation, LoadingState)>>,
    ){
        std::thread::spawn(move ||{
            let res = || -> Result<ArcDynImg>{
                let mut img_data = Vec::<u8>::new();
                file_source.read_to_end(&mut img_data)?;
                let img = image::io::Reader::new(Cursor::new(img_data)).with_guessed_format()?.decode()?;
                Ok(Arc::new(img))
            }();
            let mut guard = loading_state.lock();
            if guard.0 > generation {
                eprintln!("Dropping stale image: {file_source:?}");
                return
            }
            guard.1 = match res{
                Err(e) => LoadingState::Failed { source: file_source, err: e },
                Ok(img) => LoadingState::Ready { source: file_source, img, texture: None },
            };
        });
    }
}

impl StatefulWidget for ImageWidget2{
    type Value<'p> = Result<Arc<image::DynamicImage>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        let mut loading_state_guard = self.loading_state.lock();
        let gen_state: &mut (Generation, LoadingState) = &mut *loading_state_guard;
        let generation: &mut Generation = &mut gen_state.0;
        let loading_state: &mut LoadingState = &mut gen_state.1;

        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                self.file_source_widget.draw_and_parse(ui, id.with("file source".as_ptr()));

                if let Ok(file_source) = self.file_source_widget.state() { 'needs_reload: {
                    if let Some(fs) = loading_state.file_source() {
                        if *fs == file_source{
                            eprintln!("Reload dismissed!!");
                            break 'needs_reload
                        }
                    }
                    //FIXME: this logic should also trigger on restore, but we'd need an egui::Context
                    eprintln!("Reload REQUIRED, dispatching thread!!");
                    *generation += 1;
                    *loading_state = LoadingState::Loading{ source: file_source.clone() };
                    Self::spawn_load_image_task(
                        *generation,
                        file_source,
                        Arc::clone(&self.loading_state)
                    );
                }}

                match loading_state {
                    LoadingState::Empty => (),
                    LoadingState::Loading{ .. } => {
                        ui.ctx().request_repaint();
                        ui.weak("Loading...");
                    },
                    LoadingState::Ready{ img, texture, ..} => {
                        let tex = texture.get_or_insert_with(|| Texture::load(img, ui.ctx().clone()));
                        let (width, height) = img.dimensions();
                        let ratio =  height as f64 / width as f64;
                        tex.show(ui, egui::Vec2 { x: 50.0, y: 50.0 * ratio as f32 }); //FIXME: can we not hardcode this?
                    },
                    LoadingState::Forced { img, texture } => {
                        let tex = texture.get_or_insert_with(|| Texture::load(img, ui.ctx().clone()));
                        let (width, height) = img.dimensions();
                        let ratio =  height as f64 / width as f64;
                        tex.show(ui, egui::Vec2 { x: 50.0, y: 50.0 * ratio as f32 }); //FIXME: can we not hardcode this?
                    },
                    LoadingState::Failed { .. } => (), //will render later so erro shows under the button
                };
            });

            if let LoadingState::Failed{ err, .. } = loading_state {
                show_error(ui, err);
            }
        });

        self.update(&ui.ctx()); //FIXME: maybe don't call update on draw?
    }

    fn state(&self) -> Result<ArcDynImg>{
        let loading_state_guard = self.loading_state.lock();
        let loading_state: &(_, LoadingState) = &*loading_state_guard;
        match &loading_state.1{
            LoadingState::Empty | LoadingState::Loading { .. } => Err(GuiError::new("Empty".to_owned())),
            LoadingState::Failed { err, .. } => Err(err.clone()),
            LoadingState::Ready { img, .. } => Ok(img.clone()),
            LoadingState::Forced { img, .. } => Ok(img.clone()),
        }
    }
}

pub struct SpecialImageWidget<I>{
    image_widget: ImageWidget2,
    parsed: Result<I>,
}

impl<I> ValueWidget for SpecialImageWidget<I>
where
    I: Borrow<ArcDynImg>
{
    type Value<'v> = (Option<rt::FileSource>, Option<I>);
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.image_widget.set_value(
            (value.0, value.1.map(|val| val.borrow().clone()))
        )
    }
}

impl<I> Restore for SpecialImageWidget<I>{
    type RawData = SpecialImageWidgetRawData;
    fn restore(&mut self, value: Self::RawData){
        self.image_widget.restore(value.image_widget);
    }
    fn dump(&self) -> Self::RawData {
        SpecialImageWidgetRawData{image_widget: self.image_widget.dump()}
    }
}

impl<I> Default for SpecialImageWidget<I>{
    fn default() -> Self {
        Self{
            image_widget: Default::default(),
            parsed: Err(GuiError::new("empty".to_owned())),
        }
    }
}

impl<I> StatefulWidget for SpecialImageWidget<I>
where
    I : TryFrom<Arc<image::DynamicImage>>,
    <I as TryFrom<Arc<image::DynamicImage>>>::Error: Error,
{
    type Value<'p> = Result<&'p I> where I: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.horizontal(|ui|{
            self.image_widget.draw_and_parse(ui, id.with("img widget".as_ptr()));
            let Ok(gui_img) = self.image_widget.state() else {
                return;
            };
            //FIXME: is it always ok to do this every frame?
            self.parsed = I::try_from(gui_img).map_err(|err| GuiError::from(err))
        });
    }

    fn state<'p>(&'p self) -> Result<&'p I>{
        self.parsed.as_ref().map_err(|err| err.clone())
    } 
}
