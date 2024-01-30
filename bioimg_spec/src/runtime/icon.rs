use image::DynamicImage;
use crate::rdf;

#[derive(thiserror::Error, Debug)]
pub enum IconImageParsingError{
    #[error("Image is not square")]
    ImageNotSquare(DynamicImage)
}

pub struct IconImage(DynamicImage);

impl TryFrom<DynamicImage> for IconImage{
    type Error = IconImageParsingError;

    fn try_from(value: DynamicImage) -> Result<Self, Self::Error> {
        if value.width() != value.height(){
            Err(IconImageParsingError::ImageNotSquare(value))
        }else{
            Ok(Self(value))
        }
    }
}

pub enum Icon{
    Image(IconImage),
    Emoji(rdf::icon::EmojiIcon),
}

