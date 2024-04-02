use std::{path::PathBuf, sync::Arc};

use crate::result::{GuiError, Result};

use super::{error_display::show_error, util::DynamicImageExt, StatefulWidget};

pub struct LoadedImage{
    path: PathBuf,
    texture_name: String,
    image: Arc<image::DynamicImage>,
    context: egui::Context,
    texture_handle: egui::TextureHandle,
}
impl LoadedImage{
    pub fn image(&self) -> Arc<image::DynamicImage>{
        Arc::clone(&self.image)
    }
    pub fn load(path: PathBuf, context: egui::Context) -> Result<Self>{
        let img = image::io::Reader::open(&path)?.decode()?;
        let texture_name: String = path.to_string_lossy().into();
        let texture_handle = img.to_egui_texture_handle(&texture_name, &context);
        Ok(Self{
            path,
            texture_name,
            image: Arc::new(img),
            context,
            texture_handle,
        })
    }
    pub fn show(&self, ui: &mut egui::Ui){
        let ui_img = egui::Image::new(
            egui::ImageSource::Texture(
                egui::load::SizedTexture {
                    id: self.texture_handle.id(),
                    size: egui::Vec2 { x: 50.0, y: 50.0 },
                }
            )
        );
        ui.add(ui_img);
    }
}
impl Drop for LoadedImage {
    fn drop(&mut self) {
        self.context.forget_image(&self.texture_name);
    }
}

#[derive(Default)]
pub enum ImageWidget{
    #[default]
    Empty,
    AboutToLoad{path: PathBuf}, //useful for setting widget state without a egui::Context
    Loading{
        path: PathBuf,
        promise: poll_promise::Promise<Result<LoadedImage>>,
    },
    Ready(LoadedImage),
    Failed{path: PathBuf, message: String}
}


impl ImageWidget{
    pub fn set_path(&mut self, path: PathBuf){
        *self = ImageWidget::AboutToLoad { path };
    }
}

impl StatefulWidget for ImageWidget{
    type Value<'p> = Result<Arc<image::DynamicImage>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        if ui.button("Open...").clicked(){
            if let Some(path) = rfd::FileDialog::new().pick_file(){
                self.set_path(path);
                return;
            }
        }
        *self = match std::mem::replace(self, ImageWidget::Empty){
            ImageWidget::AboutToLoad { path } => {
                ui.ctx().request_repaint();
                let texture_name: String = path.to_string_lossy().into();
                ui.label(format!("Loading {} ...", texture_name));

                let ctx = ui.ctx().clone();
                ImageWidget::Loading {
                    path: path.clone(),
                    promise: poll_promise::Promise::spawn_thread(
                        "loading image",
                        move || {
                            LoadedImage::load(path, ctx)
                        }
                    )
                }
            },
            ImageWidget::Loading { path, promise } => {
                ui.ctx().request_repaint();
                ui.label(format!("Loading {} ...", path.to_string_lossy()));
                match promise.try_take() {
                    Err(promise) => ImageWidget::Loading { path, promise },
                    Ok(Err(error)) => ImageWidget::Failed { path, message: format!("Could not open image: {error}") },
                    Ok(Ok(loaded_image)) => ImageWidget::Ready(loaded_image),
                }
            },
            ImageWidget::Ready(loaded_image) => {
                ui.weak(loaded_image.path.to_string_lossy());
                loaded_image.show(ui);
                ImageWidget::Ready(loaded_image)
            },
            ImageWidget::Failed { path, message } => {
                show_error(ui, &message);
                ImageWidget::Failed { path, message }
            }
            ImageWidget::Empty => ImageWidget::Empty,
        };
    }

    //FIXME: less string allocs?
    fn state<'p>(&'p self) -> Self::Value<'p> {
        match self{
            Self::Ready(loaded_image) => Ok(loaded_image.image()),
            Self::Empty => Err(GuiError::new("No image selected".into())),
            Self::Failed { message, .. } => Err(GuiError::new(message.clone())),
            Self::AboutToLoad { path } | Self::Loading { path, .. } => Err(
                GuiError::new(format!("Still loading {}", path.to_string_lossy()))
            )
        }
    }
}