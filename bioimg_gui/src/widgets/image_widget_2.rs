use std::{error::Error, io::Cursor, sync::Arc};

use bioimg_runtime as rt;

use crate::result::{GuiError, Result};
use super::{error_display::show_error, file_source_widget::FileSourceWidgetPopupButton, util::DynamicImageExt, StatefulWidget};

pub struct GuiImage{
    source: Option<rt::FileSource>,
    texture_name: String,
    image: Arc<image::DynamicImage>,
    context: egui::Context,
    texture_handle: egui::TextureHandle,
}

impl GuiImage{
    pub fn image(&self) -> Arc<image::DynamicImage>{
        Arc::clone(&self.image)
    }
    pub fn load(source: rt::FileSource, context: egui::Context) -> Result<Self>{
        let mut img_data = Vec::<u8>::new();
        source.read_to_end(&mut img_data)?;
        
        let img = image::io::Reader::new(Cursor::new(img_data)).with_guessed_format()?.decode()?;
        let texture_name: String = source.to_string();
        let texture_handle = img.to_egui_texture_handle(&texture_name, &context);
        Ok(Self{
            source: Some(source),
            texture_name,
            image: Arc::new(img),
            context,
            texture_handle,
        })
    }
    pub fn show(&self, ui: &mut egui::Ui, display_size: egui::Vec2){
        let ui_img = egui::Image::new(
            egui::ImageSource::Texture(
                egui::load::SizedTexture {
                    id: self.texture_handle.id(),
                    size: display_size,
                }
            )
        );
        ui.add(ui_img);
    }
}
impl Drop for GuiImage {
    fn drop(&mut self) {
        self.context.forget_image(&self.texture_name);
    }
}

#[derive(Default)]
enum LoadingState{
    #[default]
    Empty,
    Loading{source: rt::FileSource, promise: poll_promise::Promise<Result<GuiImage>>},
    Ready(GuiImage),
    Failed(GuiError),
}

#[derive(Default)]
pub struct ImageWidget2{
    pub picker_button: FileSourceWidgetPopupButton,
    loading_state: LoadingState,
}

impl StatefulWidget for ImageWidget2{
    type Value<'p> = Result<Arc<image::DynamicImage>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.horizontal(|ui|{
            self.picker_button.draw_and_parse(ui, id.with("file source".as_ptr()));
            let file_source_res = self.picker_button.state();
            let Ok(file_source) = file_source_res else {
                self.loading_state = LoadingState::Empty;
                return;
            };
            self.loading_state = match std::mem::take(&mut self.loading_state){
                LoadingState::Empty => {
                    ui.ctx().request_repaint();
                    LoadingState::Loading {
                        promise: {
                            let file_source = file_source.clone();
                            let context = ui.ctx().clone();
                            poll_promise::Promise::spawn_thread(file_source.to_string(), move ||{
                                GuiImage::load(file_source, context)
                            })
                        },
                        source: file_source,
                    }
                },
                LoadingState::Loading { source, promise } => {
                    ui.ctx().request_repaint();
                    match promise.try_take(){
                        Err(promise) => LoadingState::Loading { source, promise },
                        Ok(Err(err)) => LoadingState::Failed(err),
                        Ok(Ok(gui_img)) => LoadingState::Ready(gui_img),
                    }
                },
                LoadingState::Failed(err) => {
                    show_error(ui, &err);
                    LoadingState::Failed(err)
                },
                LoadingState::Ready(gui_img) => {
                    gui_img.show(ui, egui::Vec2 { x: 50.0, y: 50.0 }); //FIXME
                    LoadingState::Ready(gui_img)
                },
            }
        });
    }

    fn state(&self) -> Result<Arc<image::DynamicImage>>{
        match &self.loading_state{
            LoadingState::Empty | LoadingState::Loading { .. } => Err(GuiError::new("Empty".to_owned())),
            LoadingState::Failed(err) => Err(err.clone()),
            LoadingState::Ready(gui_img) => Ok(gui_img.image())
        }
    }
}

pub struct SpecialImageWidget<I>{
    image_widget: ImageWidget2,
    parsed: Result<I>,
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
