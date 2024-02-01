use std::path::PathBuf;

use bioimg_spec::runtime as rt;
use egui::{load::SizedTexture, ImageSource};

use super::{error_display::show_error, file_widget::{ParsedFile, FileWidget}};

pub struct GuiCoverImage{
    path: PathBuf,
    contents: rt::CoverImage,
    context: egui::Context,
    texture_handle: egui::TextureHandle,
}

impl Drop for GuiCoverImage{
    fn drop(&mut self) {
        self.context.forget_image(&self.path.to_string_lossy());
    }
}

impl ParsedFile for anyhow::Result<GuiCoverImage>{ //FIXME: specific error?
    fn parse(path: PathBuf, ctx: egui::Context) -> Self {
        let contents = std::fs::read(&path)?;
        let cover_image = rt::CoverImage::try_from(contents.as_slice())?;

        let size = [cover_image.width() as _, cover_image.height() as _];
        let rgb_image = cover_image.to_rgb8();
        let pixels = rgb_image.as_flat_samples();
        let texture_image = egui::ColorImage::from_rgb(size, pixels.as_slice());

        let texture_handle = ctx.load_texture(
            path.to_string_lossy(),
            texture_image,
            egui::TextureOptions {
                magnification: egui::TextureFilter::Linear,
                minification: egui::TextureFilter::Nearest,
            },
        );
        Ok(GuiCoverImage {
            path: path.clone(),
            contents: cover_image,
            context: ctx,
            texture_handle: texture_handle.clone(),
        })
    }

    fn render(&self, ui: &mut egui::Ui, id: egui::Id){
        match self{
            Ok(loaded_cover_image) => {
                let image_source = ImageSource::Texture(SizedTexture {
                    id: loaded_cover_image.texture_handle.id(),
                    size: egui::Vec2 { x: 50.0, y: 50.0 },
                });
                let ui_img = egui::Image::new(image_source);
                ui.add(ui_img);
            },
            Err(err) => {
                show_error(ui, err.to_string())
            }
        }
    }
}

pub type CoverImageWidget = FileWidget<anyhow::Result<GuiCoverImage>>;
