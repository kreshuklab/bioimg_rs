use std::path::PathBuf;

use bioimg_spec::runtime as rt;
use egui::{load::SizedTexture, ImageSource};

use super::{
    error_display::show_error, file_widget::{FilePickerError, FileWidget, FileWidgetState}, StatefulWidget
};

pub enum CoverImageState {
    Empty,
    FilePickerError(FilePickerError),
    CoverImageError{path: PathBuf, error: rt::CoverImageParsingError},
    Loaded{
        path: PathBuf,
        contents: rt::CoverImage,
        texture_handle: egui::TextureHandle,
        size: egui::Vec2,
    },
}

impl CoverImageState{
    fn as_image_source(&self) -> Option<egui::ImageSource<'_>> {
        let Self::Loaded{texture_handle, ..} = self else{
            return None
        };
        Some(ImageSource::Texture(SizedTexture {
            id: texture_handle.id(),
            size: egui::Vec2 { x: 50.0, y: 50.0 },
        }))
    }
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
            if let CoverImageState::Loaded{path, ..} = &self.state {
                ui.ctx().forget_image(&path.to_string_lossy());
            }
            self.state = new_state;
            return;
        };

        'parse_image: {
            if let CoverImageState::Loaded{path, ..} = &self.state {
                if path == file_path {
                    break 'parse_image;
                }
            }
            if let CoverImageState::CoverImageError{path, ..} = &self.state {
                if path == file_path {
                    break 'parse_image;
                }
            }

            let img = match rt::CoverImage::try_from(file_data.as_slice()){
                Ok(cover_img) => cover_img,
                Err(error) => {
                    self.state = CoverImageState::CoverImageError { path: file_path.clone(), error };
                    break 'parse_image;
                }
            };

            let size = [img.width() as _, img.height() as _];
            let rgb_image = img.to_rgb8();
            let pixels = rgb_image.as_flat_samples();

            let texture_image = egui::ColorImage::from_rgb(size, pixels.as_slice());

            if let CoverImageState::Loaded{path, ..} = &self.state {
                ui.ctx().forget_image(&path.to_string_lossy());
            }

            let texture_handle = ui.ctx().load_texture(
                file_path.to_string_lossy(),
                texture_image,
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Linear,
                    minification: egui::TextureFilter::Nearest,
                },
            );
            self.state = CoverImageState::Loaded {
                path: file_path.clone(),
                contents: img,
                texture_handle: texture_handle.clone(),
                size: egui::Vec2 {
                    x: size[0] as f32,
                    y: size[1] as f32,
                },
            };
        };

        if let Some(image_source) = self.state.as_image_source() {
            let ui_img = egui::Image::new(image_source);
            ui.add(ui_img);
        }
        if let CoverImageState::CoverImageError { error, .. } = &self.state{
            show_error(ui, error.to_string());
        }
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.state
    }
}
