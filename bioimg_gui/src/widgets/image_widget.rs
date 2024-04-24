use std::{error::Error, path::PathBuf, sync::Arc};

use crate::result::{GuiError, Result};

use super::{error_display::{show_error, show_if_error}, util::DynamicImageExt, StatefulWidget, ValueWidget};

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
impl Drop for LoadedImage {
    fn drop(&mut self) {
        self.context.forget_image(&self.texture_name);
    }
}

#[derive(Default)]
pub enum ImageWidgetState{
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

pub struct ImageWidget<T>{
    pub state: ImageWidgetState,
    pub parsed: Result<T>,
    pub display_size: egui::Vec2,
}

impl<T> Default for ImageWidget<T>{
    fn default() -> Self {
        ImageWidget{
            state: ImageWidgetState::default(),
            parsed: Err(GuiError::new("Empty".into())),
            display_size: egui::Vec2 { x: 50.0, y: 50.0 },
        }
    }
}

impl<T> ValueWidget for ImageWidget<T>{
    type Value<'a> = PathBuf;
    fn set_value<'a>(&mut self, path: Self::Value<'a>) {
        self.state = ImageWidgetState::AboutToLoad { path };
    }
}

//FIXME: remove this, use ValueWidget
impl<T> ImageWidget<T>{
    pub fn set_path(&mut self, path: PathBuf){
        self.state = ImageWidgetState::AboutToLoad { path };
    }
}

impl<T> StatefulWidget for ImageWidget<T>
where
    T: TryFrom<Arc<image::DynamicImage>>,
    <T as TryFrom<Arc<image::DynamicImage>>>::Error: Error,
    T: Clone,
{
    type Value<'p> = Result<T> where T: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                if ui.button("Open...").clicked(){
                    if let Some(path) = rfd::FileDialog::new().pick_file(){
                        self.set_path(path);
                        return;
                    }
                }
                self.state = match std::mem::replace(&mut self.state, ImageWidgetState::Empty){
                    ImageWidgetState::AboutToLoad { path } => {
                        ui.ctx().request_repaint();
                        let texture_name: String = path.to_string_lossy().into();
                        ui.label(format!("Loading {} ...", texture_name));

                        let ctx = ui.ctx().clone();
                        ImageWidgetState::Loading {
                            path: path.clone(),
                            promise: poll_promise::Promise::spawn_thread(
                                "loading image",
                                move || {
                                    LoadedImage::load(path, ctx)
                                }
                            )
                        }
                    },
                    ImageWidgetState::Loading { path, promise } => {
                        ui.ctx().request_repaint();
                        ui.label(format!("Loading {} ...", path.to_string_lossy()));
                        match promise.try_take() {
                            Err(promise) => ImageWidgetState::Loading { path, promise },
                            Ok(Err(error)) => ImageWidgetState::Failed { path, message: format!("Could not open image: {error}") },
                            Ok(Ok(loaded_image)) => ImageWidgetState::Ready(loaded_image),
                        }
                    },
                    ImageWidgetState::Ready(loaded_image) => {
                        ui.weak(loaded_image.path.to_string_lossy());
                        loaded_image.show(ui, self.display_size);
                        ImageWidgetState::Ready(loaded_image)
                    },
                    ImageWidgetState::Failed { path, message } => {
                        show_error(ui, &message);
                        ImageWidgetState::Failed { path, message }
                    }
                    ImageWidgetState::Empty => ImageWidgetState::Empty,
                };
            });
            match &self.state{
                ImageWidgetState::Ready(loaded_image) => {
                    self.parsed = T::try_from(loaded_image.image()).map_err(|err| GuiError::from(err));
                    show_if_error(ui, &self.parsed);
                },
                _ => {
                    self.parsed = Err(GuiError::new("No image ready".into()));
                },
            };
        });
    }

    //FIXME: less string allocs?
    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}