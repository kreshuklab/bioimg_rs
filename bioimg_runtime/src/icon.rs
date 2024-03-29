use std::io::{Seek, Write};

use bioimg_spec::rdf;
use image::codecs::png::PngEncoder;
use image::DynamicImage;

use crate::zip_writer_ext::ModelZipWriter;
use crate::zoo_model::ModelPackingError;

#[derive(thiserror::Error, Debug, Clone)]
pub enum IconParsingError {
    #[error("Image is not square")]
    ImageNotSquare(DynamicImage),
    #[error("0")]
    RdfError(#[from] rdf::IconParsingError),
}

pub struct IconImage(DynamicImage);

impl TryFrom<DynamicImage> for IconImage {
    type Error = IconParsingError;

    fn try_from(value: DynamicImage) -> Result<Self, Self::Error> {
        if value.width() != value.height() {
            Err(IconParsingError::ImageNotSquare(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<DynamicImage> for Icon {
    type Error = IconParsingError;
    fn try_from(value: DynamicImage) -> Result<Self, Self::Error> {
        Ok(Self::Image(IconImage::try_from(value)?))
    }
}

pub enum Icon {
    Image(IconImage),
    Text(rdf::icon::EmojiIcon),
}

impl From<rdf::icon::EmojiIcon> for Icon{
    fn from(value: rdf::icon::EmojiIcon) -> Self {
        Self::Text(value)
    }
}

impl Icon{
    pub fn dump(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result< rdf::Icon, ModelPackingError> {
        let icon_img = match self{
            Self::Text(emoji) => return Ok(rdf::Icon::Emoji(emoji.clone())),
            Self::Image(icon_img) => icon_img,
        };
        let test_tensor_zip_path = rdf::FsPath::unique();
        let test_tensor_zip_path_str: String = test_tensor_zip_path.clone().into();
        zip_file.write_file(&test_tensor_zip_path_str, |writer| -> Result<(), ModelPackingError> {
            let encoder = PngEncoder::new(writer);
            Ok(icon_img.0.write_with_encoder(encoder)?)
        })?;
        Ok(rdf::Icon::FileRef(rdf::FileReference::Path(test_tensor_zip_path)))
    }
}

impl TryFrom<String> for Icon {
    type Error = IconParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::Text(rdf::EmojiIcon::try_from(value)?))
    }
}
