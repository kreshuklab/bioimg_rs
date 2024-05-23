use std::{borrow::Borrow, fmt::Display, io::{Read, Seek, Write}, path::Path, sync::Arc};

use bioimg_spec::rdf::{self, FileReference, HttpUrl};

use crate::{zip_writer_ext::ModelZipWriter, zoo_model::ModelPackingError};

#[derive(thiserror::Error, Debug)]
pub enum FileSourceError{
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error("Error downloading file: {0}")]
    HttpError(#[from] ureq::Error)
}

#[derive(Clone, PartialEq, Eq)]
pub enum FileSource{
    LocalFile{path: Arc<Path>},
    FileInZipArchive{outer_path: Arc<Path>, inner_path: Arc<str>},
    HttpUrl(Arc<HttpUrl>),
}

impl Display for FileSource{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::LocalFile { path } => write!(f, "{}", path.to_string_lossy()),
            Self::FileInZipArchive { outer_path, inner_path } => write!(f, "{}/{}", outer_path.to_string_lossy(), inner_path),
            Self::HttpUrl(http_url) => write!(f, "{}", http_url.as_str()),
        }
    }
}

impl FileSource{
    //FIXME: add some cancellation token?
    fn rdf_dump(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<rdf::FsPath, ModelPackingError> {
        let output_inner_path = rdf::FsPath::unique();
        zip_file.write_file(&output_inner_path, |writer| -> Result<u64, ModelPackingError>{
            let copied_bytes: u64 = match self{
                Self::LocalFile { path } => {
                    std::io::copy(&mut std::fs::File::open(path)?, writer)?
                },
                Self::FileInZipArchive { outer_path, inner_path } => {
                    let archive_file = std::fs::File::open(outer_path)?;
                    let mut archive = zip::ZipArchive::new(archive_file)?;
                    let mut archived_file = archive.by_name(inner_path.as_ref())?;
                    std::io::copy(&mut archived_file, writer)?
                },
                Self::HttpUrl(http_url) => {
                    let response = ureq::get(http_url.as_str()).call()?;
                    eprintln!("Requesting {http_url} returned result {}", response.status());
                    if response.status() / 100 != 2{
                        return Err(ModelPackingError::UnexpectedHttpStatus {
                            status: response.status(),
                            url: http_url.as_ref().clone(),
                        })
                    }
                    let mut response_reader = response.into_reader();
                    std::io::copy(&mut response_reader, writer)? //FIXME!! limit size or whatever
                }
            };
            Ok(copied_bytes)
        })?;
        Ok(output_inner_path)
    }

    pub fn rdf_dump_as_file_reference(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<rdf::FileReference, ModelPackingError> {
        let output_inner_path = self.rdf_dump(zip_file)?;
        Ok(rdf::FileReference::Path(output_inner_path))
    }

    pub fn dump_as_file_description(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<rdf::FileDescription, ModelPackingError> {
        let file_reference = self.rdf_dump_as_file_reference(zip_file)?;
        Ok(rdf::FileDescription{source: file_reference, sha256: None})
    }
}

impl FileSource{
    pub fn from_rdf_file_descr<T: Borrow<FileReference>>(
        zip_path: &Path, file_reference: &rdf::FileDescription<T>
    ) -> Result<Self, FileSourceError>{
        Self::from_rdf_file_reference(zip_path, file_reference.source.borrow())
    }


    pub fn from_rdf_file_reference(
        zip_path: &Path, file_reference: &rdf::FileReference
    ) -> Result<Self, FileSourceError>{
        Ok(match file_reference{
            rdf::FileReference::Url(url) => Self::HttpUrl(Arc::new(url.clone())),
            rdf::FileReference::Path(path) => Self::FileInZipArchive {
                outer_path: Arc::from(zip_path),
                inner_path:  Arc::from(String::from(path).as_str())
            }
        })
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> Result<usize, FileSourceError>{
        match self{
            Self::LocalFile { path } => Ok(std::fs::File::open(path)?.read_to_end(buf)?),
            Self::FileInZipArchive { outer_path, inner_path } => {
                let mut archive = zip::ZipArchive::new(std::fs::File::open(outer_path)?)?;
                let bytes_read = archive.by_name(&inner_path)?.read_to_end(buf)?;
                Ok(bytes_read)
            },
            Self::HttpUrl(http_url) => {
                let mut response_reader = ureq::get(http_url.as_str())
                .call()?
                .into_reader();
                Ok(response_reader.read_to_end(buf)?)
            }
        }
    }
}
