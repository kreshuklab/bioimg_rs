use std::{borrow::Borrow, io::{Read, Seek, Write}, path::PathBuf};

use bioimg_spec::rdf::{self, FileReference};

use crate::{zip_writer_ext::ModelZipWriter, zoo_model::ModelPackingError};

#[derive(thiserror::Error, Debug)]
pub enum FileSourceError{
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error("Url not supported yet")]
    UrlNotSupportedYet,
}

#[derive(Clone)]
pub enum FileSource{
    LocalFile{path: PathBuf},
    FileInZipArchive{outer_path: PathBuf, inner_path: String},
}

impl FileSource{
    fn rdf_dump(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<rdf::FsPath, ModelPackingError> {
        let output_inner_path = rdf::FsPath::unique();
        zip_file.write_file(&output_inner_path, |writer| {
            match self{
                Self::LocalFile { path } => {
                    std::io::copy(&mut std::fs::File::open(path)?, writer)
                },
                Self::FileInZipArchive { outer_path, inner_path } => {
                    let archive_file = std::fs::File::open(outer_path)?;
                    let mut archive = zip::ZipArchive::new(archive_file)?;
                    let mut archived_file = archive.by_name(inner_path.as_str())?;
                    std::io::copy(&mut archived_file, writer)
                }
            }
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
        zip_path: PathBuf, file_reference: &rdf::FileDescription<T>
    ) -> Result<Self, FileSourceError>{
        Self::from_rdf_file_reference(zip_path, file_reference.source.borrow())
    }


    pub fn from_rdf_file_reference(
        zip_path: PathBuf, file_reference: &rdf::FileReference
    ) -> Result<Self, FileSourceError>{
        Ok(Self::FileInZipArchive {
            outer_path: zip_path,
            inner_path: match file_reference{
                rdf::FileReference::Url(_) => return Err(FileSourceError::UrlNotSupportedYet),
                rdf::FileReference::Path(path) => path.into()
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
            }
        }
    }
}