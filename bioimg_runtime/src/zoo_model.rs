use std::{
    io::{Seek, Write},
    path::PathBuf,
};

use bioimg_spec::rdf::{
    author::Author2,
    cite_entry::CiteEntry2,
    maintainer::Maintainer,
    model::{
        ModelRdf, RdfTypeModel
    },
    non_empty_list::NonEmptyList,
    version::Version_0_5_0,
    FileReference, FsPath, HttpUrl, LicenseId, ResourceName, ResourceTextDescription, Version
};
use image::ImageError;

use crate::{
    file_reference::FileExt, model_weights::ModelWeights, npy_array::ArcNpyArray, zip_writer_ext::ModelZipWriter, CoverImage, Icon, ModelInterface
};

#[derive(thiserror::Error, Debug)]
pub enum ModelPackingError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    ImageError(#[from] ImageError),
    #[error("{0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("File {0} already exists")]
    AlreadyExists(PathBuf),
    #[error("{0}")]
    WriteNpyError(#[from] ndarray_npy::WriteNpyError),
    #[error("{0}")]
    RdfSerializationError(#[from] serde_json::Error),
    #[error("Could not write yaml file to zip: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),
}

pub struct ZooModel<'a> {
    description: ResourceTextDescription,
    covers: Vec<CoverImage>,
    attachments: Vec<std::fs::File>,
    cite: NonEmptyList<CiteEntry2>,
    // config: serde_json::Map<String, serde_json::Value>,
    git_repo: Option<HttpUrl>,
    icon: Option<Icon>,
    links: Vec<String>,
    maintainers: Vec<Maintainer>,
    tags: Vec<String>,
    version: Option<Version>,
    authors: NonEmptyList<Author2>,
    documentation: &'a str,
    license: LicenseId,
    name: ResourceName,
    // training_data: DatasetDescrEnum, //FIXME
    weights: ModelWeights,
    interface: ModelInterface<ArcNpyArray>,
}

impl<'a> ZooModel<'a> {
    pub fn pack_into<Sink: Write + Seek>(mut self, sink: Sink) -> Result<(), ModelPackingError> {
        let mut writer = ModelZipWriter::new(sink);

        let (inputs, outputs) = self.interface.dump(&mut writer)?;
        let covers = self.covers.iter().map(|cov| {
            cov.dump(&mut writer)
        }).collect::<Result<Vec<_>, _>>()?;
        let attachments = self.attachments.iter_mut().map(|file|{
            file.rdf_dump(&mut writer)
        }).collect::<Result<Vec<_>, _>>()?;
        let icon = match &self.icon{
            Some(icon) => Some(icon.dump(&mut writer)?),
            None => None,
        };
        let attachments = self.attachments.iter_mut().map(|rt_att|{ // Should this be internal mutability
            rt_att.rdf_dump(&mut writer)
        }).collect::<Result<Vec<_>, _>>()?;
        let documentation: FileReference = {
            let documentation_path = FsPath::unique_suffixed(".md");
            let documentation_path_string: String = documentation_path.clone().into(); //FIXME
            writer.write_file(&documentation_path_string, |writer| -> Result<FileReference, std::io::Error> {
                writer.write(self.documentation.as_bytes())?;
                Ok(FileReference::Path(documentation_path))
            })?
        };
        let weights = self.weights.rdf_dump(&mut writer)?;

        let model_rdf = ModelRdf {
            description: self.description,
            covers,
            id: None,
            attachments,
            cite: self.cite,
            config: serde_json::Map::new(),
            git_repo: self.git_repo,
            icon,
            links: self.links,
            maintainers: self.maintainers,
            tags: self.tags,
            version: self.version,
            format_version: Version_0_5_0::new(),
            rdf_type: RdfTypeModel,
            authors: self.authors,
            documentation,
            inputs,
            license: self.license,
            name: self.name,
            outputs,
            run_mode: None,
            timestamp: iso8601_timestamp::Timestamp::now_utc(),
            training_data: None, //FIXME
            weights,
        };

        writer.write_file("/rdf.yaml", |writer| serde_yaml::to_writer(writer, &model_rdf))?;

        writer.finish()?;
        Ok(())
    }
}
