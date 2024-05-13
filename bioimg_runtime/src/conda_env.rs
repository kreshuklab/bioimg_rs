use std::io::{Read, Seek, Write};

use bioimg_spec::rdf;
use zip::ZipArchive;

use crate::{zip_archive_ext::{RdfFileReferenceExt, RdfFileReferenceReadError}, zip_writer_ext::ModelZipWriter, zoo_model::ModelPackingError};

#[derive(thiserror::Error, Debug)]
pub enum CondaEnvParsingError{
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("Could not parse yaml contents: {0}")]
    YamlParsingError(#[from] serde_yaml::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum CondaEnvLoadingError{
    #[error(transparent)]
    ParsingError(#[from] CondaEnvParsingError),
    #[error(transparent)]
    RdfFileReferenceReadError(#[from] RdfFileReferenceReadError),
}

#[derive(Clone)]
pub struct CondaEnv{
    pub raw: serde_yaml::Mapping,
}

impl CondaEnv{
    pub fn try_load(reader: impl std::io::Read) -> Result<Self, CondaEnvParsingError>{
        Ok(Self{
            raw: serde_yaml::from_reader(reader)?
        })
    }

    pub fn try_load_rdf<R: Read + Seek>(
        rdf: rdf::FileDescription<rdf::EnvironmentFile>, zip_archive: &mut ZipArchive<R>
    ) -> Result<Self, CondaEnvLoadingError>{
        let reader = rdf.source.try_get_reader(zip_archive)?;
        Ok(CondaEnv::try_load(reader)?)
    }
}

impl CondaEnv{
    pub fn rdf_dump(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<rdf::EnvironmentFileDescr, ModelPackingError> {
        let zip_path = rdf::FsPath::unique_suffixed("_environment.yml");
        zip_file.write_file(&zip_path, |writer| {
            serde_yaml::to_writer(writer, &self.raw)
        })?;
        let file_ref = rdf::FileReference::Path(zip_path);
        Ok(rdf::FileDescription{
            source: file_ref.try_into().unwrap(),
            sha256: None //FIXME
        })
    }
}
