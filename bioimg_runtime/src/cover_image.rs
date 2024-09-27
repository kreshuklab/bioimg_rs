use std::{borrow::Borrow, io::{Cursor, Read, Seek, Write}, ops::Deref, sync::Arc};

use bioimg_spec::rdf;
use image::codecs::png::PngEncoder;
use zip::ZipArchive;

use crate::zip_archive_ext::RdfFileReferenceExt;
use crate::{zip_archive_ext::RdfFileReferenceReadError, zip_writer_ext::ModelZipWriter, zoo_model::ModelPackingError};

#[derive(Clone)]
pub struct CoverImage(Arc<image::DynamicImage>);

impl CoverImage {
    pub const ALLOWED_WIDTH_TO_HEIGHT_RATIOS: [f32; 2] = [1.0, 2.0];
    pub const MAX_SIZE_IN_BYTES: usize = 500 * 1024;

    fn is_valid_ratio(_ratio: f32) -> bool {
        return true
        // return Self::ALLOWED_WIDTH_TO_HEIGHT_RATIOS
        //     .into_iter()
        //     .find(|v| *v == ratio)
        //     .is_some();
    }
    pub fn dump(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result< rdf::CoverImageSource, ModelPackingError> {
        let test_tensor_zip_path = rdf::FsPath::unique_suffixed("_cover_image.png");
        zip_file.write_file(&test_tensor_zip_path, |writer| -> Result<(), ModelPackingError> {
            let encoder = PngEncoder::new(writer);
            Ok(self.0.write_with_encoder(encoder)?)
        })?;
        Ok(rdf::CoverImageSource::try_from(rdf::FileReference::Path(test_tensor_zip_path)).unwrap())
    }
}

impl Borrow<Arc<image::DynamicImage>> for CoverImage{
    fn borrow(&self) -> &Arc<image::DynamicImage> {
        &self.0
    }
}

impl Deref for CoverImage {
    type Target = image::DynamicImage;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CoverImageParsingError {
    #[error("Image is too big ({size} bytes), must be up to 500KB")]
    TooBig { size: usize },
    #[error("Bad aspect ratio (width / height): {ratio}, expected 2:1 or 1:1")]
    BadAspectRatio { ratio: f32 },
    #[error("{0}")]
    BadImageData(#[from] image::ImageError),
}

impl TryFrom<Arc<image::DynamicImage>> for CoverImage{
    type Error = CoverImageParsingError;
    fn try_from(img: Arc<image::DynamicImage>) -> Result<Self, Self::Error> {
        let ratio = (img.width() as f32) / (img.height() as f32);
        if !Self::is_valid_ratio(ratio) {
            return Err(CoverImageParsingError::BadAspectRatio { ratio });
        }
        return Ok(Self(img));
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CoverImageLoadingError{
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("Could not parse cover image: {0}")]
    ParsingError(#[from] CoverImageParsingError),
    #[error("Could not parse image: {0}")]
    ImageParsingError(#[from] image::ImageError),
    #[error(transparent)]
    RdfFileReferenceReadError(#[from] RdfFileReferenceReadError)
}

impl CoverImage{
    pub fn try_load<R: Read + Seek>(
        rdf_cover: rdf::CoverImageSource,
        archive: &mut ZipArchive<R>
    ) -> Result<Self, CoverImageLoadingError>{
        let mut image_bytes = Vec::<u8>::new();
        rdf_cover.try_get_reader(archive)?.read_to_end(&mut image_bytes)?;
        let image = image::io::Reader::new(Cursor::new(image_bytes)).with_guessed_format()?.decode()?;
        Ok(CoverImage::try_from(Arc::new(image))?)
    }
}


