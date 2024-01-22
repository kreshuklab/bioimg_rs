use std::path::PathBuf;

use egui::{load::SizedTexture, ImageSource};
use image::{io::Reader as ImageReader, DynamicImage};

use super::{file_widget::{FilePickerError, FileWidget}, DrawAndParse};


#[derive(thiserror::Error, Debug, Clone)]
pub  enum CoverImageError{
    #[error("{0}")]
    FileError(#[from] FilePickerError),
    #[error("Image is too big ({size} bytes), must be up to 500KB")]
    TooBig{size: usize},
    #[error("{reason}")]
    ImageError{path: PathBuf, reason: String},
    #[error("Bad aspect ratio (width / height): {ratio}, expected 2:1 or 1:1")]
    BadAspectRation{ratio: f32},
}

#[derive(Clone)]
pub struct LoadedImage{
    path: PathBuf,
    contents: DynamicImage,
    texture_handle: egui::TextureHandle,
    size: egui::Vec2,
}

impl LoadedImage{
    fn as_image_source(&self) -> egui::ImageSource<'_>{
        ImageSource::Texture(
            SizedTexture{
                id: self.texture_handle.id(),
                size: egui::Vec2{x: 50.0, y: 50.0},
            }
        )
    }
}

pub struct CoverImageWidget{
    file_widget: FileWidget,
    contents: Result<LoadedImage, CoverImageError>,
}

impl Default for CoverImageWidget{
    fn default() -> Self {
        Self{
            file_widget: FileWidget::default(),
            contents: Err(CoverImageError::FileError(FilePickerError::Empty))
        }
    }
}

impl DrawAndParse for CoverImageWidget{
    type Parsed<'p> = &'p LoadedImage;
    type Error = CoverImageError;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error> {
        let loaded_file = match self.file_widget.draw_and_parse(ui, id){
            Err(err) => {
                self.contents = Err(err.clone().into());
                return Err(err.into())
            },
            Ok(loaded_file) => loaded_file
        };

        'parse_image: {
            if let Ok(current_image) = &self.contents{
                if &current_image.path == loaded_file.path(){
                    break 'parse_image
                }
            }
            if let Err(CoverImageError::ImageError{path, ..}) = &self.contents{
                if path == loaded_file.path(){
                    break 'parse_image
                }
            }
            let data_size = loaded_file.contents().len();
            if data_size > 500 * 1024{
                self.contents = Err(CoverImageError::TooBig { size: data_size });
                break 'parse_image
            }
            let cursor = std::io::Cursor::new(loaded_file.contents());
            let img = match ImageReader::new(cursor).with_guessed_format().unwrap().decode(){
                Ok(image) => image,
                Err(err) => {
                    self.contents = Err(CoverImageError::ImageError {
                        path: loaded_file.path().clone(),
                        reason: err.to_string(),
                    });
                    break 'parse_image
                }
            };

            let ratio = (img.width() as f32) / (img.height() as f32);
            if ratio != 2.0 && ratio != 1.0{
                self.contents = Err(CoverImageError::BadAspectRation{ratio});
                break 'parse_image
            }

            let size = [img.width() as _, img.height() as _];
            let rgb_image = img.to_rgb8();
            let pixels = rgb_image.as_flat_samples();

            let texture_image = egui::ColorImage::from_rgb(
                size,
                pixels.as_slice(),
            );

            if let Ok(ref last_img) = self.contents{
                ui.ctx().forget_image(&last_img.path.to_string_lossy());
            }

            let texture_handle = ui.ctx().load_texture(
                loaded_file.path().to_string_lossy(),
                texture_image,
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Linear,
                    minification: egui::TextureFilter::Nearest,
                }
            );
            self.contents = Ok(LoadedImage{
                path: loaded_file.path().clone(),
                contents: img,
                texture_handle: texture_handle.clone(),
                size: egui::Vec2{x: size[0] as f32, y: size[1] as f32},
            });
        }

        if let Ok(ref img) = self.contents{
            let ui_img = egui::Image::new(img.as_image_source());
            ui.add(ui_img);
        }

        self.contents.as_ref().map_err(|err| err.clone())
    }
}