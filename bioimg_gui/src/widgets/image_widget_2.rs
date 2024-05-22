use std::{error::Error, io::Cursor, sync::Arc};

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
    Ready{img: ArcDynImg, texture: Texture},
    Failed(GuiError),
}

#[derive(Default)]
pub struct ImageWidget2{
    pub picker_button: FileSourceWidgetPopupButton,
    loading_state: LoadingState,
}

impl ValueWidget for ImageWidget2{
    type Value<'v> = rt::FileSource;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.picker_button.set_value(value);
    }
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
                },
                LoadingState::Loading { source, promise } => {
                    ui.ctx().request_repaint();
                    match promise.try_take(){
                        Err(promise) => LoadingState::Loading { source, promise },
                        Ok(Err(err)) => LoadingState::Failed(err),
                        Ok(Ok((img, texture))) => LoadingState::Ready{img, texture},
                    }
                },
                LoadingState::Failed(err) => {
                    show_error(ui, &err);
                    LoadingState::Failed(err)
                },
                LoadingState::Ready{img, texture} => {
                    texture.show(ui, egui::Vec2 { x: 50.0, y: 50.0 }); //FIXME
                    LoadingState::Ready{img, texture}
                },
            }
        });
    }

    fn state(&self) -> Result<ArcDynImg>{
        match &self.loading_state{
            LoadingState::Empty | LoadingState::Loading { .. } => Err(GuiError::new("Empty".to_owned())),
            LoadingState::Failed(err) => Err(err.clone()),
            LoadingState::Ready{img, ..} => Ok(img.clone())
        }
    }
}

pub struct SpecialImageWidget<I>{
    image_widget: ImageWidget2,
    parsed: Result<I>,
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
