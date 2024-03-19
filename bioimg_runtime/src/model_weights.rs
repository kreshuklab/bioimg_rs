use std::{io::{Seek, Write}, path::PathBuf};

use bioimg_spec::rdf;//::{self, author::Author2, model::weights::{ArchitectureDescr, OnnxOpsetVersion}, Version};
use bioimg_spec::rdf::model as modelrdf;

use crate::{conda_env::CondaEnv, file_reference::FileExt, zip_writer_ext::ModelZipWriter, zoo_model::ModelPackingError};

#[derive(thiserror::Error, Debug)]
pub enum ModelWeightsError{
    #[error("No model weights provided")]
    NoModels,
}

pub struct ModelWeights{
    keras_hdf5: Option<KerasHdf5Weights>,
    onnx: Option<OnnxWeights>,
    pytorch_state_dict: Option<PytorchStateDictWeights>,
    tensorflow_js: Option<TensorflowJsWeights>,
    tensorflow_saved_model_bundle: Option<TensorflowSavedModelBundleWeights>,
    torchscript: Option<TorchscriptWeights>,
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
        &mut self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::WeightsDescr, ModelPackingError> {
        let keras_hdf5 = self.keras_hdf5.as_mut().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let onnx = self.onnx.as_mut().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let pytorch_state_dict = self.pytorch_state_dict.as_mut().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let tensorflow_js = self.tensorflow_js.as_mut().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let tensorflow_saved_model_bundle = self.tensorflow_saved_model_bundle.as_mut().map(|weights|{
            weights.rdf_dump(zip_file)
        }).transpose()?;
        let torchscript = self.torchscript.as_mut().map(|weights|{
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
}


pub struct WeightsBase{
    source: PathBuf,
    authors: Option<Vec<rdf::Author2>>,
}

impl WeightsBase{
    fn rdf_dump_suffixed(
        &mut self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
        suffix: &str,
    ) -> Result<modelrdf::WeightsDescrBase, ModelPackingError> {
        Ok(modelrdf::WeightsDescrBase{
            source: self.source.rdf_dump_suffixed(zip_file, suffix)?,
            authors: self.authors.clone(),
            parent: None, //FIXME
            sha256: None, //FIXME
        })
    }
    fn rdf_dump(
        &mut self, zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<modelrdf::WeightsDescrBase, ModelPackingError> {
        Ok(modelrdf::WeightsDescrBase{
            source: self.source.rdf_dump(zip_file)?,
            authors: self.authors.clone(),
            parent: None, //FIXME
            sha256: None, //FIXME
        })
    }
}

pub struct KerasHdf5Weights{
    pub weights: WeightsBase,
    pub tensorflow_version: rdf::Version,
}
impl KerasHdf5Weights{
    fn rdf_dump(
        &mut self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::KerasHdf5WeightsDescr, ModelPackingError> {
        let weights = self.weights.rdf_dump(zip_file)?;
        Ok(modelrdf::KerasHdf5WeightsDescr{
            base: weights,
            tensorflow_version: self.tensorflow_version.clone(),
        })
    }
}


pub struct OnnxWeights{
    pub weights: WeightsBase,
    pub opset_version: modelrdf::OnnxOpsetVersion,
}

impl OnnxWeights{
    fn rdf_dump(
        &mut self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::OnnxWeightsDescr, ModelPackingError> {
        let weights = self.weights.rdf_dump(zip_file)?;
        Ok(modelrdf::OnnxWeightsDescr{
            base: weights,
            opset_version: self.opset_version.clone(),
        })
    }
}


pub struct PytorchStateDictWeights{
    pub weights: WeightsBase,
    pub architecture: modelrdf::PytorchArchitectureDescr,
    pub pytorch_version: rdf::Version,
    pub dependencies: Option<CondaEnv>,
}

impl PytorchStateDictWeights{
    fn rdf_dump(
        &mut self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::PytorchStateDictWeightsDescr, ModelPackingError> {
        Ok(modelrdf::PytorchStateDictWeightsDescr{
            base: self.weights.rdf_dump(zip_file)?,
            architecture: self.architecture.clone(),
            pytorch_version: self.pytorch_version.clone(),
            dependencies: self.dependencies.as_mut().map(|env|{
                env.rdf_dump(zip_file)
            }).transpose()?,
        })
    }
}


pub struct TensorflowJsWeights{
    // FIXME: source must be a zip?
    // FIXME: double check what "wo_special_file_name" is supposed to mean
    pub weights: WeightsBase,
    /// Version of the TensorFlow library used
    pub tensorflow_version: rdf::Version,
}
impl TensorflowJsWeights{
    fn rdf_dump(
        &mut self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::TensorflowJsWeightsDescr, ModelPackingError> {
        Ok(modelrdf::TensorflowJsWeightsDescr{
            base: self.weights.rdf_dump_suffixed(zip_file, ".zip")?,
            tensorflow_version: self.tensorflow_version.clone(),
        })
    }
}


pub struct TensorflowSavedModelBundleWeights{
    //FIXME: file should be a zip
    pub weights: WeightsBase,
    pub tensorflow_version: rdf::Version,
    pub dependencies: Option<CondaEnv>,
}

impl TensorflowSavedModelBundleWeights{
    fn rdf_dump(
        &mut self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::TensorflowSavedModelBundleWeightsDescr, ModelPackingError> {
        Ok(modelrdf::TensorflowSavedModelBundleWeightsDescr{
            base: self.weights.rdf_dump_suffixed(zip_file, ".zip")?,
            tensorflow_version: self.tensorflow_version.clone(),
            dependencies: self.dependencies.as_mut().map(|env|{
                env.rdf_dump(zip_file)
            }).transpose()?,
        })
    }
}


pub struct TorchscriptWeights {
    pub weights: WeightsBase,
    pub pytorch_version: rdf::Version,
}

impl TorchscriptWeights {
    fn rdf_dump(
        &mut self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<modelrdf::TorchscriptWeightsDescr, ModelPackingError> {
        Ok(modelrdf::TorchscriptWeightsDescr{
            base: self.weights.rdf_dump_suffixed(zip_file, ".zip")?,
            pytorch_version: self.pytorch_version.clone(),
        })
    }
}
