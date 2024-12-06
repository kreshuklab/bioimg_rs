use std::{fmt::Display, io::{Seek, Write}, str::FromStr};


use bioimg_spec::rdf;

use crate::zip_archive_ext::{RdfFileReferenceReadError, SharedZipArchive};
use crate::zoo_model::ModelPackingError;
use crate::zip_writer_ext::ModelZipWriter;

#[derive(thiserror::Error, Debug)]
pub enum CondaEnvParsingError{
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("Could not parse yaml contents: {0}")]
    YamlParsingError(#[from] serde_yaml::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum CondaEnvLoadingError{
    #[error("Could not load conda env from zip archive: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error(transparent)]
    ParsingError(#[from] CondaEnvParsingError),
    #[error(transparent)]
    RdfFileReferenceReadError(#[from] RdfFileReferenceReadError),
    #[error("Url file reference not supported yet")]
    UrlFileReferenceNotSupportedYet,
}

#[derive(Clone)]
pub struct CondaEnv{
    pub raw: serde_yaml::Mapping,
}

impl FromStr for CondaEnv{
    type Err = CondaEnvParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw: serde_yaml::Mapping = serde_yaml::from_str(s)?;
        Ok(Self{ raw })
    }
}

impl Display for CondaEnv{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_yaml::to_string(&self.raw).unwrap())
    }
}

impl CondaEnv{
    pub fn try_load(reader: impl std::io::Read) -> Result<Self, CondaEnvParsingError>{
        Ok(Self{
            raw: serde_yaml::from_reader(reader)?
        })
    }

    pub fn try_load_rdf(
        descr: rdf::FileDescription<rdf::EnvironmentFile>, zip_archive: &SharedZipArchive
    ) -> Result<Self, CondaEnvLoadingError>{
        let file_ref: &rdf::FileReference = &descr.source;
        let inner_path: String = match file_ref{
            rdf::FileReference::Url(_) => return Err(CondaEnvLoadingError::UrlFileReferenceNotSupportedYet),
            rdf::FileReference::Path(path) => path.into(),
        };
        let conda_env = zip_archive.with_entry(&inner_path, |entry|{
            CondaEnv::try_load(entry)
        })??;
        Ok(conda_env)
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
