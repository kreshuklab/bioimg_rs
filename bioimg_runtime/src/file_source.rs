use std::{borrow::Borrow, fmt::Display, io::{Read, Seek, Write}, path::Path, sync::Arc};

use bioimg_spec::rdf::{self, FileReference, HttpUrl};

use crate::{zip_writer_ext::ModelZipWriter, zoo_model::ModelPackingError};

#[derive(thiserror::Error, Debug)]
pub enum FileSourceError{
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("IO error trying to read {path}: {inner}")]
    ZipError{inner: zip::result::ZipError, path: String},
    #[error("Error downloading file: {reason}")]
    HttpError{reason: String}
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
        let extension = match self{
            Self::LocalFile { path } => path.extension().map(|ex| ex.to_string_lossy().to_string()),
            Self::FileInZipArchive { inner_path, .. } => {
                inner_path.split(".").last().map(|s| s.to_owned())
            },
            Self::HttpUrl(url) => {
                url.path().split(".").last().map(|s| s.to_owned())
            }
        };
        let output_inner_path = match extension{
            Some(ext) => rdf::FsPath::unique_suffixed(&format!(".{ext}")),
            None => rdf::FsPath::unique(),
        };
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
                #[cfg(target_arch = "wasm32")]
                Self::HttpUrl(_http_url) => {
                    panic!("can't download in wasm yet. This'd need to be async")
                }
                #[cfg(not(target_arch = "wasm32"))]
                Self::HttpUrl(http_url) => {
                    let response = ureq::get(http_url.as_str()).call()
                        .map_err(|e| ModelPackingError::HttpErro { reason: e.to_string()})?;
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
                let mut archive = zip::ZipArchive::new(std::fs::File::open(outer_path)?)
                    .map_err(|inner| FileSourceError::ZipError{inner, path: outer_path.to_string_lossy().into()})?;
                let full_path = format!("{}/{}", outer_path.to_string_lossy(), inner_path);
                let bytes_read = archive.by_name(&inner_path)
                    .map_err(|inner| FileSourceError::ZipError { inner, path: full_path})?
                    .read_to_end(buf)?;
                Ok(bytes_read)
            },
            #[cfg(target_arch = "wasm32")]
            Self::HttpUrl(_http_url) => {
                panic!("Can't download on wasm yet. This'd need to be async")
            },
            #[cfg(not(target_arch = "wasm32"))]
            Self::HttpUrl(http_url) => {
                let mut response_reader = ureq::get(http_url.as_str())
                .call()
                .map_err(|e| FileSourceError::HttpError { reason: e.to_string()})?
                .into_reader();
                Ok(response_reader.read_to_end(buf)?)
            }
        }
    }
}
