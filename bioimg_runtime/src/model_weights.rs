use std::{io::{Read, Seek, Write}, path::Path};

use bioimg_spec::rdf;
use bioimg_spec::rdf::model as modelrdf;
use zip::ZipArchive;

use crate::{conda_env::CondaEnvLoadingError, zip_archive_ext::RdfFileReferenceReadError};
use crate::{conda_env::CondaEnv, file_source::FileSourceError, zip_writer_ext::ModelZipWriter, zoo_model::ModelPackingError, FileSource};

#[derive(thiserror::Error, Debug)]
pub enum ModelWeightsError{
    #[error("No model weights provided")]
    NoModels,
}

#[derive(Clone)]
pub struct ModelWeights{
    keras_hdf5: Option<KerasHdf5Weights>,
    onnx: Option<OnnxWeights>,
    pytorch_state_dict: Option<PytorchStateDictWeights>,
    tensorflow_js: Option<TensorflowJsWeights>,
    tensorflow_saved_model_bundle: Option<TensorflowSavedModelBundleWeights>,
    torchscript: Option<TorchscriptWeights>,
}

impl ModelWeights{
    pub fn keras_hdf5(&self) -> Option<&KerasHdf5Weights>{
        self.keras_hdf5.as_ref()
    }
    pub fn onnx(&self) -> Option<&OnnxWeights>{
        self.onnx.as_ref()
    }
    pub fn pytorch_state_dict(&self) -> Option<&PytorchStateDictWeights>{
        self.pytorch_state_dict.as_ref()
    }
    pub fn tensorflow_js(&self) -> Option<&TensorflowJsWeights>{
        self.tensorflow_js.as_ref()
    }
    pub fn tensorflow_saved_model_bundle(&self) -> Option<&TensorflowSavedModelBundleWeights>{
        self.tensorflow_saved_model_bundle.as_ref()
    }
    pub fn torchscript(&self) -> Option<&TorchscriptWeights>{
        self.torchscript.as_ref()
    }
}


impl ModelWeights{
    pub fn new(
        keras_hdf5: Option<KerasHdf5Weights>,
        onnx: Option<OnnxWeights>,
        pytorch_state_dict: Option<PytorchStateDictWeights>,
        tensorflow_js: Option<TensorflowJsWeights>,
        tensorflow_saved_model_bundle: Option<TensorflowSavedModelBundleWeights>,
        torchscript: Option<TorchscriptWeights>,
    ) -> Result<Self, ModelWeightsError>{
        if keras_hdf5.is_none()
        && onnx.is_none()
        && pytorch_state_dict.is_none()
        && tensorflow_js.is_none()
        && tensorflow_saved_model_bundle.is_none()
        && torchscript.is_none() {
            return Err(ModelWeightsError::NoModels)
        }
        Ok(Self{
            keras_hdf5,
            onnx,
            pytorch_state_dict,
            tensorflow_js,
            tensorflow_saved_model_bundle,
            torchscript,
        })
    }

    pub fn rdf_dump(
        &self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::WeightsDescr, ModelPackingError> {
        let keras_hdf5 = self.keras_hdf5.as_ref().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let onnx = self.onnx.as_ref().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let pytorch_state_dict = self.pytorch_state_dict.as_ref().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let tensorflow_js = self.tensorflow_js.as_ref().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let tensorflow_saved_model_bundle = self.tensorflow_saved_model_bundle.as_ref().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let torchscript = self.torchscript.as_ref().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        Ok(modelrdf::WeightsDescr::try_from(modelrdf::MaybeSomeWeightsDescr{
            keras_hdf5,
            onnx,
            pytorch_state_dict,
            tensorflow_js,
            tensorflow_saved_model_bundle,
            torchscript,
        }).unwrap())
    }

    pub fn try_from_rdf<R: Read + Seek>(
        weights_rdf: modelrdf::WeightsDescr,
        zip_file_path: &Path,
        zip_archive: &mut ZipArchive<R>,
    ) -> Result<Self, ModelWeightsLoadingError>{
        let weights = weights_rdf.into_inner();
        Ok(Self{
            keras_hdf5: weights.keras_hdf5
                .map(|rdf| KerasHdf5Weights::try_from_rdf(rdf, zip_file_path))
                .transpose()?,
            onnx: weights.onnx
                .map(|rdf| OnnxWeights::try_from_rdf(rdf, zip_file_path))
                .transpose()?,
            pytorch_state_dict: weights.pytorch_state_dict
                .map(|rdf| PytorchStateDictWeights::try_from_rdf(rdf, zip_file_path, zip_archive))
                .transpose()?,
            tensorflow_js: weights.tensorflow_js
                .map(|rdf| TensorflowJsWeights::try_from_rdf(rdf, zip_file_path))
                .transpose()?,
            tensorflow_saved_model_bundle: weights.tensorflow_saved_model_bundle
                .map(|rdf| TensorflowSavedModelBundleWeights::try_from_rdf(rdf, zip_file_path, zip_archive))
                .transpose()?,
            torchscript: weights.torchscript
                .map(|rdf| TorchscriptWeights::try_from_rdf(rdf, zip_file_path))
                .transpose()?,
        })
    }
}
#[derive(Clone)]
pub struct WeightsBase{
    pub source: FileSource,
    pub authors: Option<Vec<rdf::Author2>>,
}

#[derive(thiserror::Error, Debug)]
pub enum ModelWeightsLoadingError{
    #[error("Could not load weights: {0}")]
    FileSourceError(#[from] FileSourceError),
    #[error("Could not retrieve file from zip: {0}")]
    RdfFileReferenceReadError(#[from] RdfFileReferenceReadError),
    #[error("Could not parse conda env: {0}")]
    CondaEnvLoadingError(#[from] CondaEnvLoadingError)
}

impl WeightsBase{
    fn rdf_dump(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<modelrdf::WeightsDescrBase, ModelPackingError> {
        Ok(modelrdf::WeightsDescrBase{
            source: self.source.rdf_dump_as_file_reference(zip_file)?,
            authors: self.authors.clone(),
            parent: None, //FIXME
            sha256: None, //FIXME
        })
    }

    fn try_from_rdf(
        rdf_weights_base: modelrdf::WeightsDescrBase,
        zip_file_path: &Path,
    ) -> Result<Self, ModelWeightsLoadingError>{
        Ok(Self{
            authors: rdf_weights_base.authors,
            source: FileSource::from_rdf_file_reference(zip_file_path, &rdf_weights_base.source)?
        })
    }
}

#[derive(Clone)]
pub struct KerasHdf5Weights{
    pub weights: WeightsBase,
    pub tensorflow_version: rdf::Version,
}
impl KerasHdf5Weights{
    fn rdf_dump(
        &self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::KerasHdf5WeightsDescr, ModelPackingError> {
        let weights = self.weights.rdf_dump(zip_file)?;
        Ok(modelrdf::KerasHdf5WeightsDescr{
            base: weights,
            tensorflow_version: self.tensorflow_version.clone(),
        })
    }

    pub fn try_from_rdf(
        rdf: modelrdf::KerasHdf5WeightsDescr, zip_file_path: &Path
    ) -> Result<Self, ModelWeightsLoadingError>{
        let weights = WeightsBase::try_from_rdf(rdf.base, zip_file_path)?;
        Ok(Self{
            weights,
            tensorflow_version: rdf.tensorflow_version,
        })
    }
}

#[derive(Clone)]
pub struct OnnxWeights{
    pub weights: WeightsBase,
    pub opset_version: modelrdf::OnnxOpsetVersion,
}

impl OnnxWeights{
    fn rdf_dump(
        &self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::OnnxWeightsDescr, ModelPackingError> {
        let weights = self.weights.rdf_dump(zip_file)?;
        Ok(modelrdf::OnnxWeightsDescr{
            base: weights,
            opset_version: self.opset_version.clone(),
        })
    }

    pub fn try_from_rdf(
        rdf: modelrdf::OnnxWeightsDescr, zip_file_path: &Path
    ) -> Result<Self, ModelWeightsLoadingError>{
        let weights = WeightsBase::try_from_rdf(rdf.base, zip_file_path)?;
        Ok(Self{
            weights,
            opset_version: rdf.opset_version,
        })
    }
}

#[derive(Clone)]
pub enum PytorchArch{
    FromLib(modelrdf::weights::PyTorchArchitectureFromLibraryDescr),
    FromFile{
        file_source: FileSource,
        callable: rdf::Identifier,
        kwargs: serde_json::Map<String, serde_json::Value>,
    }
}

impl PytorchArch{
    fn rdf_dump(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<modelrdf::PytorchArchitectureDescr, ModelPackingError> {
        match self{
            Self::FromFile { file_source: file_descr, callable, kwargs } => {
                let file_descr = file_descr.dump_as_file_description(zip_file)?;
                Ok(modelrdf::PytorchArchitectureDescr::FromFileDescr(
                    modelrdf::weights::PyTorchArchitectureFromFileDescr{
                        file_descr,
                        callable: callable.clone(),
                        kwargs: kwargs.clone()
                    }
                ))
            },
            Self::FromLib(arch) => {
                Ok(arch.clone().into())
            },
        }
    }

    pub fn try_from_rdf(zip_path: &Path, rdf: modelrdf::PytorchArchitectureDescr) -> Result<Self, ModelWeightsLoadingError>{
        match rdf{
            modelrdf::PytorchArchitectureDescr::FromFileDescr(from_file) => {
                Ok(Self::FromFile {
                    file_source: FileSource::from_rdf_file_descr(zip_path, &from_file.file_descr)?,
                    callable: from_file.callable,
                    kwargs: from_file.kwargs,
                })
            },
            modelrdf::PytorchArchitectureDescr::FromLibraryDescr(from_lib) => {
                Ok(Self::FromLib(from_lib))
            }
        }
    }
}

#[derive(Clone)]
pub struct PytorchStateDictWeights{
    pub weights: WeightsBase,
    pub architecture: PytorchArch,
    pub pytorch_version: rdf::Version,
    pub dependencies: Option<CondaEnv>,
}

impl PytorchStateDictWeights{
    fn rdf_dump(
        &self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::PytorchStateDictWeightsDescr, ModelPackingError> {
        Ok(modelrdf::PytorchStateDictWeightsDescr{
            base: self.weights.rdf_dump(zip_file)?,
            architecture: self.architecture.rdf_dump(zip_file)?,
            pytorch_version: self.pytorch_version.clone(),
            dependencies: self.dependencies.as_ref().map(|env|{
                env.rdf_dump(zip_file)
            }).transpose()?,
        })
    }

    pub fn try_from_rdf<R: Read + Seek>(
        rdf: modelrdf::PytorchStateDictWeightsDescr,
        zip_file_path: &Path,
        zip_archive: &mut ZipArchive<R>,
    ) -> Result<Self, ModelWeightsLoadingError>{
        let weights = WeightsBase::try_from_rdf(rdf.base, zip_file_path)?;
        Ok(Self{
            weights,
            architecture: PytorchArch::try_from_rdf(zip_file_path, rdf.architecture)?,
            pytorch_version: rdf.pytorch_version,
            dependencies: rdf.dependencies
                .map(|value| CondaEnv::try_load_rdf(value, zip_archive))
                .transpose()?
        })
    }
}

#[derive(Clone)]
pub struct TensorflowJsWeights{
    // FIXME: source must be a zip?
    // FIXME: double check what "wo_special_file_name" is supposed to mean
    pub weights: WeightsBase,
    /// Version of the TensorFlow library used
    pub tensorflow_version: rdf::Version,
}
impl TensorflowJsWeights{
    fn rdf_dump(
        &self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::TensorflowJsWeightsDescr, ModelPackingError> {
        Ok(modelrdf::TensorflowJsWeightsDescr{
            base: self.weights.rdf_dump(zip_file)?,
            tensorflow_version: self.tensorflow_version.clone(),
        })
    }

    pub fn try_from_rdf(
        rdf: modelrdf::TensorflowJsWeightsDescr, zip_file_path: &Path
    ) -> Result<Self, ModelWeightsLoadingError>{
        let weights = WeightsBase::try_from_rdf(rdf.base, zip_file_path)?;
        Ok(Self{
            weights,
            tensorflow_version: rdf.tensorflow_version,
        })
    }
}

#[derive(Clone)]
pub struct TensorflowSavedModelBundleWeights{
    //FIXME: file should be a zip
    pub weights: WeightsBase,
    pub tensorflow_version: rdf::Version,
    pub dependencies: Option<CondaEnv>,
}

impl TensorflowSavedModelBundleWeights{
    fn rdf_dump(
        &self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::TensorflowSavedModelBundleWeightsDescr, ModelPackingError> {
        Ok(modelrdf::TensorflowSavedModelBundleWeightsDescr{
            base: self.weights.rdf_dump(zip_file)?,
            tensorflow_version: self.tensorflow_version.clone(),
            dependencies: self.dependencies.as_ref().map(|env|{
                env.rdf_dump(zip_file)
            }).transpose()?,
        })
    }

    pub fn try_from_rdf<R: Read + Seek>(
        rdf: modelrdf::TensorflowSavedModelBundleWeightsDescr,
        zip_file_path: &Path,
        zip_archive: &mut ZipArchive<R>,
    ) -> Result<Self, ModelWeightsLoadingError>{
        let weights = WeightsBase::try_from_rdf(rdf.base, zip_file_path)?;
        Ok(Self{
            weights,
            tensorflow_version: rdf.tensorflow_version,
            dependencies: rdf.dependencies
                .map(|value| CondaEnv::try_load_rdf(value, zip_archive))
                .transpose()?
        })
    }
}

#[derive(Clone)]
pub struct TorchscriptWeights {
    pub weights: WeightsBase,
    pub pytorch_version: rdf::Version,
}

impl TorchscriptWeights {
    fn rdf_dump(
        &self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::TorchscriptWeightsDescr, ModelPackingError> {
        Ok(modelrdf::TorchscriptWeightsDescr{
            base: self.weights.rdf_dump(zip_file)?,
            pytorch_version: self.pytorch_version.clone(),
        })
    }

    pub fn try_from_rdf(
        rdf: modelrdf::TorchscriptWeightsDescr,
        zip_file_path: &Path,
    ) -> Result<Self, ModelWeightsLoadingError>{
        let weights = WeightsBase::try_from_rdf(rdf.base, zip_file_path)?;
        Ok(Self{
            weights,
            pytorch_version: rdf.pytorch_version,
        })
    }
}
