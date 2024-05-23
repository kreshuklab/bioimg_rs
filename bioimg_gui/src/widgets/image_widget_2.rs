use std::{borrow::Borrow, error::Error, io::Cursor, sync::Arc};

use poll_promise as pp;
use bioimg_runtime as rt;

use crate::result::{GuiError, Result};
use super::{error_display::show_error, file_source_widget::FileSourceWidgetPopupButton, util::DynamicImageExt, StatefulWidget, ValueWidget};

pub type ArcDynImg = Arc<image::DynamicImage>;

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
    Loading{source: rt::FileSource, promise: pp::Promise<Result<(ArcDynImg, Texture)>>},
    Ready{source: rt::FileSource, img: ArcDynImg, texture: Texture},
    Forced{img: ArcDynImg, texture: Option<Texture>},
    Failed{source: rt::FileSource, err: GuiError},
}

impl LoadingState{
    pub fn loading(file_source: rt::FileSource, context: egui::Context) -> Self{
        LoadingState::Loading {
            promise: {
                let file_source = file_source.clone();
                pp::Promise::spawn_thread(file_source.to_string(), move ||{
                    let mut img_data = Vec::<u8>::new();
                    file_source.read_to_end(&mut img_data)?;
                    let img = image::io::Reader::new(Cursor::new(img_data)).with_guessed_format()?.decode()?;
                    let texture = Texture::load(&img, context);
                    Ok((Arc::new(img), texture))
                })
            },
            source: file_source,
        }
    }
}

#[derive(Default)]
pub struct ImageWidget2{
    pub picker_button: FileSourceWidgetPopupButton,
    loading_state: LoadingState,
}

impl ValueWidget for ImageWidget2{
    type Value<'v> = (Option<rt::FileSource>, Option<ArcDynImg>);

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        match value{
            (None, Some(img)) => {
                self.picker_button = Default::default();
                self.loading_state = LoadingState::Forced { img, texture: None};
            },
            (None, None) => {
                self.picker_button = Default::default();
                self.loading_state = LoadingState::Empty;
            },
            (Some(file_source), _) => {
                self.picker_button.set_value(file_source);
                self.loading_state = LoadingState::Empty;
            }
        }
    }
}

impl StatefulWidget for ImageWidget2{
    type Value<'p> = Result<Arc<image::DynamicImage>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.horizontal(|ui|{
            self.picker_button.draw_and_parse(ui, id.with("file source".as_ptr()));
            let file_source_res = self.picker_button.state();
            self.loading_state = match (std::mem::take(&mut self.loading_state), file_source_res){
                (LoadingState::Empty, Err(_)) => LoadingState::Empty,
                (LoadingState::Empty, Ok(file_source)) => {
                    ui.ctx().request_repaint();
                    LoadingState::loading(file_source, ui.ctx().clone())
                },
                (LoadingState::Loading{..}, Err(_)) => LoadingState::Empty,
                (LoadingState::Loading { source, promise }, Ok(new_source)) => 'loading_ok: {
                    ui.ctx().request_repaint();
                    if source != new_source{
                        break 'loading_ok LoadingState::Empty;
                    }
                    match promise.try_take(){
                        Err(promise) => LoadingState::Loading { source, promise },
                        Ok(Err(err)) => LoadingState::Failed{source, err},
                        Ok(Ok((img, texture))) => LoadingState::Ready{source, img, texture},
                    }
                },
                (LoadingState::Failed{..}, Err(_)) => LoadingState::Empty,
                (LoadingState::Failed{source, err}, Ok(new_source)) => {
                    if source == new_source{
                        show_error(ui, &err);
                        LoadingState::Failed { source, err }
                    }else{
                        LoadingState::Empty
                    }
                },
                (LoadingState::Ready{..}, Err(_)) => LoadingState::Empty,
                (LoadingState::Ready{source, img, texture}, Ok(new_source)) => {
                    if new_source == source{
                        texture.show(ui, egui::Vec2 { x: 50.0, y: 50.0 }); //FIXME
                        LoadingState::Ready{source, img, texture}
                    }else{
                        LoadingState::Empty
                    }
                },
                (LoadingState::Forced { img, texture }, Err(_)) => {
                    let texture = texture.unwrap_or_else(|| Texture::load(&img, ui.ctx().clone()));
                    texture.show(ui, egui::Vec2 { x: 50.0, y: 50.0 }); //FIXME
                    LoadingState::Forced { img,  texture: Some(texture) }
                },
                (LoadingState::Forced{..}, Ok(_)) => LoadingState::Empty,
            }
        });
    }

    fn state(&self) -> Result<ArcDynImg>{
        match &self.loading_state{
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
