use std::{
    io::{Seek, Write},
    path::PathBuf, sync::Arc,
};

use bioimg_spec::rdf::{
    self, author::Author2, cite_entry::CiteEntry2, file_reference::FsPathComponent, maintainer::Maintainer, model::{
        ModelRdf, RdfTypeModel
    }, non_empty_list::NonEmptyList, version::Version_0_5_0, FileReference, FsPath, HttpUrl, LicenseId, ResourceName, ResourceTextDescription, Version
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

pub struct ZooModel {
    pub description: ResourceTextDescription,
    pub covers: Vec<Arc<CoverImage>>,
    pub attachments: Vec<PathBuf>,
    pub cite: NonEmptyList<CiteEntry2>,
    // config: serde_json::Map<String, serde_json::Value>,
    pub git_repo: Option<HttpUrl>,
    pub icon: Option<Arc<Icon>>,
    pub links: Vec<String>,
    pub maintainers: Vec<Maintainer>,
    pub tags: Vec<String>,
    pub version: Option<Version>,
    pub authors: NonEmptyList<Author2>,
    pub documentation: String,
    pub license: LicenseId,
    pub name: ResourceName,
    // training_data: DatasetDescrEnum, //FIXME
    pub weights: ModelWeights,
    pub interface: ModelInterface<ArcNpyArray>,
}

impl ZooModel {
    pub fn pack_into<Sink: Write + Seek>(self, sink: Sink) -> Result<(), ModelPackingError> {
        let mut writer = ModelZipWriter::new(sink);

        let (inputs, outputs) = self.interface.dump(&mut writer)?;
        let covers = self.covers.iter().map(|cov| {
            cov.dump(&mut writer)
        }).collect::<Result<Vec<_>, _>>()?;
        let attachments = self.attachments.iter().map(|file|{
            file.rdf_dump(&mut writer)
        }).collect::<Result<Vec<_>, _>>()?;
        let icon: Option<rdf::Icon> = match &self.icon{
            Some(icon) => Some(icon.dump(&mut writer)?),
            None => None,
        };
        let documentation: FileReference = {
            let documentation_path = FsPath::unique_suffixed("_README.md");
            writer.write_file(&documentation_path, |writer| -> Result<FileReference, std::io::Error> {
                writer.write(self.documentation.as_bytes())?;
                Ok(FileReference::Path(documentation_path.clone()))
            })?
        };
        let config = serde_yaml::Mapping::new();
        let timestamp = iso8601_timestamp::Timestamp::now_utc();
        let weights = self.weights.rdf_dump(&mut writer)?;

        let model_rdf = ModelRdf {
            description: self.description,
            covers: covers,
            id: None,
            attachments: attachments,
            cite: self.cite,
            config: config,
            git_repo: self.git_repo,
            icon: icon,
            links: self.links,
            maintainers: self.maintainers,
            tags: self.tags,
            version: self.version,
            format_version: Version_0_5_0::new(),
            rdf_type: RdfTypeModel,
            authors: self.authors,
            documentation: documentation,
            inputs: inputs,
            license: self.license,
            name: self.name,
            outputs: outputs,
            run_mode: None,
            timestamp: timestamp,
            training_data: None, //FIXME
            weights: weights,
        };
        let model_json_val = serde_json::to_value(&model_rdf).unwrap();

        let rdf_file_name = FsPathComponent::try_from("rdf.yaml".to_owned()).unwrap();
        let rdf_path = FsPath::from_components(vec![rdf_file_name]).unwrap();
        writer.write_file(&rdf_path, |writer| serde_yaml::to_writer(writer, &model_json_val))?;

        writer.finish()?;
        Ok(())
    }
}
