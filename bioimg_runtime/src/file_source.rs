

use std::{io::{Seek, Write}, path::PathBuf};

use bioimg_spec::rdf;

use crate::{zip_writer_ext::ModelZipWriter, zoo_model::ModelPackingError};

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