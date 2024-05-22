use std::{io::Cursor, path::Path, sync::Arc};

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
    pub file_source_widget: FileSourceWidgetPopupButton,
    loading_state: LoadingState,
}

impl ImageWidget2{
    pub fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.vertical(|ui|{
            self.file_source_widget.draw_and_parse(ui, id.with("file source".as_ptr()));
            let file_source_res = self.file_source_widget.state();
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
}
