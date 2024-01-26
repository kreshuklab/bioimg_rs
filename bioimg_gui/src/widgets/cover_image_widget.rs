use std::path::{Path, PathBuf};

use egui::{load::SizedTexture, ImageSource};
use image::{io::Reader as ImageReader, DynamicImage};

use super::{
    file_widget::{FilePickerError, FileWidget, FileWidgetState},
    StatefulWidget,
};

#[derive(thiserror::Error, Debug, Clone)]
pub enum CoverImageError {
    #[error("Image is too big ({size} bytes), must be up to 500KB")]
    TooBig { path: PathBuf, size: usize },
    #[error("Bad aspect ratio (width / height): {ratio}, expected 2:1 or 1:1")]
    BadAspectRation { path: PathBuf, ratio: f32 },
    #[error("{reason}")]
    ImageError { path: PathBuf, reason: String },
}
impl CoverImageError {
    pub fn path(&self) -> &Path {
        match self {
            Self::TooBig { path, .. } => path,
            Self::ImageError { path, .. } => path,
            Self::BadAspectRation { path, .. } => path,
        }
    }
}

#[derive(Clone)]
pub struct LoadedImage {
    path: PathBuf,
    contents: DynamicImage,
    texture_handle: egui::TextureHandle,
    size: egui::Vec2,
}

impl LoadedImage {
    fn as_image_source(&self) -> egui::ImageSource<'_> {
        ImageSource::Texture(SizedTexture {
            id: self.texture_handle.id(),
            size: egui::Vec2 { x: 50.0, y: 50.0 },
        })
    }
}

pub enum CoverImageState {
    Empty,
    FilePickerError(FilePickerError),
    CoverImageError(CoverImageError),
    Loaded(LoadedImage),
}

pub struct CoverImageWidget {
    file_widget: FileWidget,
    state: CoverImageState,
}

impl Default for CoverImageWidget {
    fn default() -> Self {
        Self {
            file_widget: FileWidget::default(),
            state: CoverImageState::Empty,
        }
    }
}

impl StatefulWidget for CoverImageWidget {
    type Value<'p> = &'p CoverImageState;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id){
        let (file_path, file_data) = 'get_file: {
            self.file_widget.draw_and_parse(ui, id);
            let new_state = match self.file_widget.state() {
                FileWidgetState::Loaded { path, data } => break 'get_file (path, data),
                FileWidgetState::Empty => CoverImageState::Empty,
                FileWidgetState::Loading { .. } => CoverImageState::Empty,
                FileWidgetState::Failed(err) => CoverImageState::FilePickerError(err.clone()),
            };
            if let CoverImageState::Loaded(last_img) = &self.state {
                ui.ctx().forget_image(&last_img.path.to_string_lossy());
            }
            self.state = new_state;
            return;
        };

        'parse_image: {
            if let CoverImageState::Loaded(loaded_image) = &self.state {
                if &loaded_image.path == file_path {
                    break 'parse_image;
                }
            }
            if let CoverImageState::CoverImageError(err) = &self.state {
                if err.path() == file_path {
                    break 'parse_image;
                }
            }

            let data_size = file_data.len();
            if data_size > 500 * 1024 {
                self.state = CoverImageState::CoverImageError(CoverImageError::TooBig {
                    path: file_path.clone(),
                    size: data_size,
                });
                break 'parse_image;
            }

            let cursor = std::io::Cursor::new(file_data);
            let img = match ImageReader::new(cursor).with_guessed_format().unwrap().decode() {
                Ok(image) => image,
                Err(err) => {
                    self.state = CoverImageState::CoverImageError(CoverImageError::ImageError {
                        path: file_path.clone(),
                        reason: err.to_string(),
                    });
                    break 'parse_image;
                }
            };

            let ratio = (img.width() as f32) / (img.height() as f32);
            if ratio != 2.0 && ratio != 1.0 {
                self.state = CoverImageState::CoverImageError(CoverImageError::BadAspectRation {
                    path: file_path.clone(),
                    ratio,
                });
                break 'parse_image;
            }

            let size = [img.width() as _, img.height() as _];
            let rgb_image = img.to_rgb8();
            let pixels = rgb_image.as_flat_samples();

            let texture_image = egui::ColorImage::from_rgb(size, pixels.as_slice());

            if let CoverImageState::Loaded(last_img) = &self.state {
                ui.ctx().forget_image(&last_img.path.to_string_lossy());
            }

            let texture_handle = ui.ctx().load_texture(
                file_path.to_string_lossy(),
                texture_image,
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Linear,
                    minification: egui::TextureFilter::Nearest,
                },
            );
            self.state = CoverImageState::Loaded(LoadedImage {
                path: file_path.clone(),
                contents: img,
                texture_handle: texture_handle.clone(),
                size: egui::Vec2 {
                    x: size[0] as f32,
                    y: size[1] as f32,
                },
            })
        };

        if let CoverImageState::Loaded(img) = &self.state {
            let ui_img = egui::Image::new(img.as_image_source());
            ui.add(ui_img);
        }
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.state
    }
}
