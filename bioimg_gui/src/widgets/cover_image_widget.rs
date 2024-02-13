use std::path::PathBuf;

use crate::result::Result;
use bioimg_spec::runtime as rt;
use egui::{load::SizedTexture, ImageSource};

use super::{
    error_display::show_error,
    file_widget::{FileWidget, ParsedFile},
    util::DynamicImageExt,
};

pub struct GuiCoverImage {
    path: PathBuf,
    contents: rt::CoverImage,
    context: egui::Context,
    texture_handle: egui::TextureHandle,
}

impl Drop for GuiCoverImage {
    fn drop(&mut self) {
        self.context.forget_image(&self.path.to_string_lossy());
    }
}

impl ParsedFile for Result<GuiCoverImage> {
    //FIXME: specific error?
    fn parse(path: PathBuf, ctx: egui::Context) -> Self {
        let contents = std::fs::read(&path)?;
        let cover_image = rt::CoverImage::try_from(contents.as_slice())?;
        let texture_handle = cover_image.to_egui_texture_handle(path.to_string_lossy(), &ctx);
        Ok(GuiCoverImage {
            path: path.clone(),
            contents: cover_image,
            context: ctx,
            texture_handle: texture_handle.clone(),
        })
    }

    fn render(&self, ui: &mut egui::Ui, id: egui::Id) {
        match self {
            Ok(loaded_cover_image) => {
                let image_source = ImageSource::Texture(SizedTexture {
                    id: loaded_cover_image.texture_handle.id(),
                    size: egui::Vec2 { x: 50.0, y: 50.0 },
                });
                let ui_img = egui::Image::new(image_source);
                ui.add(ui_img);
            }
            Err(err) => show_error(ui, err.to_string()),
        }
    }
}

pub type CoverImageWidget = FileWidget<Result<GuiCoverImage>>;
