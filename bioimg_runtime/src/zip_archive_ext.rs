use std::io::{Read, Seek};

use bioimg_spec::rdf;
use zip::read::ZipFile;

#[derive(thiserror::Error, Debug)]
pub enum RdfFileReferenceReadError{
    #[error("{0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Url file reference not supported yet")]
    UrlFileReferenceNotSupportedYet,
}

pub trait RdfFileReferenceExt{
    fn try_get_reader<'z, R: Read + Seek>(
        &self, archive: &'z mut zip::ZipArchive<R>
    ) -> Result<ZipFile<'z>, RdfFileReferenceReadError>;
}
impl RdfFileReferenceExt for rdf::FileReference{
    fn try_get_reader<'z, R: Read + Seek>(
        &self, archive: &'z mut zip::ZipArchive<R>
    ) -> Result<ZipFile<'z>, RdfFileReferenceReadError>{
        let inner_path: String = match self{
            rdf::FileReference::Url(_) => return Err(RdfFileReferenceReadError::UrlFileReferenceNotSupportedYet),
            rdf::FileReference::Path(path) => path.into(),
        };
        Ok(archive.by_name(&inner_path)?)
    }
}