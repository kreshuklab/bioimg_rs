use crate::rdf;
use image::DynamicImage;

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

impl TryFrom<String> for Icon {
    type Error = IconParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::Text(rdf::EmojiIcon::try_from(value)?))
    }
}
