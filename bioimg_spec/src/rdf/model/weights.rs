use crate::rdf::{author::Author2, file_description::{FileDescription, Sha256}, FileReference, Identifier, Version};

#[derive(thiserror::Error, Debug)]
pub enum ModelWeightsParsingError{
    #[error("Bad or unsupported Onnx opset version: {0}. Must be >= 7")]
    BadOnnxOpsetVersion(usize),
    #[error("No model weights found")]
    NoWeightsFound,
    #[error("Dependencies must be a .yml or .yaml file")]
    DependenciesNotYaml{path: String}
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct MaybeSomeWeightsDescr{
    #[serde(default)]
    keras_hdf5: Option<KerasHdf5WeightsDescr>,
    #[serde(default)]
    onnx: Option<OnnxWeightsDescr>,
    #[serde(default)]
    pytorch_state_dict: Option<PytorchStateDictWeightsDescr>,
    #[serde(default)]
    tensorflow_js: Option<TensorflowJsWeightsDescr>,
    #[serde(default)]
    tensorflow_saved_model_bundle: Option<TensorflowSavedModelBundleWeightsDescr>,
    #[serde(default)]
    torchscript: Option<TorchscriptWeightsDescr>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(try_from = "MaybeSomeWeightsDescr")]
pub struct WeightsDescr(MaybeSomeWeightsDescr);

impl TryFrom<MaybeSomeWeightsDescr> for WeightsDescr{
    type Error = ModelWeightsParsingError;
    fn try_from(value: MaybeSomeWeightsDescr) -> Result<Self, Self::Error> {
        if value.keras_hdf5.is_none()
        && value.onnx.is_none()
        && value.pytorch_state_dict.is_none()
        && value.tensorflow_js.is_none()
        && value.tensorflow_saved_model_bundle.is_none()
        && value.torchscript.is_none(){
            return Err(ModelWeightsParsingError::NoWeightsFound)
        }
        Ok(Self(value))
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum WeightsFormat{
    #[serde(rename = "keras_hdf5")]
    KerasHdf5,
    #[serde(rename="onnx")]
    Onnx,
    #[serde(rename="pytorch_state_dict")]
    PytorchStateDict,
    #[serde(rename="tensorflow_js")]
    TensorflowJs,
    #[serde(rename="tensorflow_saved_model_bundle")]
    TensorflowSavedModelBundle,
    #[serde(rename="torchscript")]
    Torchscript,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum ModelWeightsEnum{
    #[serde(rename = "keras_hdf5")]
    KerasHdf5WeightsDescr(KerasHdf5WeightsDescr),
    #[serde(rename="onnx")]
    OnnxWeightsDescr(OnnxWeightsDescr),
    #[serde(rename="pytorch_state_dict")]
    PytorchStateDictWeightsDescr(PytorchStateDictWeightsDescr),
    #[serde(rename="tensorflow_js")]
    TensorflowJsWeightsDescr(TensorflowJsWeightsDescr),
    #[serde(rename="tensorflow_saved_model_bundle")]
    TensorflowSavedModelBundleWeightsDescr(TensorflowSavedModelBundleWeightsDescr),
    #[serde(rename="torchscript")]
    TorchscriptWeightsDescr(TorchscriptWeightsDescr),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
struct WeightsDescrBase{
    source: FileReference,
    #[serde(default)]
    sha256: Option<Sha256>,
    #[serde(default)]
    authors: Option<Vec<Author2>>,
    parent: Option<WeightsFormat>,
}


#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KerasHdf5WeightsDescr{
    #[serde(flatten)]
    base: WeightsDescrBase,
    tensorflow_version: Version,
}



#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct OnnxOpsetVersion(usize);
impl TryFrom<usize> for OnnxOpsetVersion{
    type Error = ModelWeightsParsingError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 7{
            Err(ModelWeightsParsingError::BadOnnxOpsetVersion(value))
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct OnnxWeightsDescr{
    #[serde(flatten)]
    base: WeightsDescrBase,
    opset_version: OnnxOpsetVersion,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PyTorchArchitectureFromFileDescr{
    /// Identifier of the callable that returns a torch.nn.Module instance."""
    /// examples: "MyNetworkClass", "get_my_model"
    callable: Identifier<String>,
    /// key word arguments for the `callable`
    kwargs: serde_json::Map<String, serde_json::Value>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PyTorchArchitectureFromLibraryDescr{
    /// Identifier of the callable that returns a torch.nn.Module instance.
    /// examples: "MyNetworkClass", "get_my_model"
    callable: Identifier<String>,
    /// key word arguments for the `callable`
    kwargs: serde_json::Map<String, serde_json::Value>,
    /// Where to import the callable from, i.e. `from <import_from> import <callable>`
    import_from: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum ArchitectureDescr{
    FromLibraryDescr(PyTorchArchitectureFromLibraryDescr),
    FromFileDescr(PyTorchArchitectureFromFileDescr),
}


#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct PytorchStateDictWeightsDescr{
    #[serde(flatten)]
    base: WeightsDescrBase,
    architecture: ArchitectureDescr,
    /// Version of the PyTorch library used.
    /// If `architecture.depencencies` is specified it has to include pytorch and any version pinning has to be compatible.
    pytorch_version: Version,
    ///Custom depencies beyond pytorch.
    ///
    ///The conda environment file should include pytorch and any version pinning has to be compatible with
    ///
    ///`pytorch_version`.
    #[serde(default)]
    dependencies: Option<EnvironmentFileDescr>,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct EnvironmentFileDescr(FileDescription);

impl TryFrom<FileDescription> for EnvironmentFileDescr{
    type Error = ModelWeightsParsingError;
    fn try_from(value: FileDescription) -> Result<Self, Self::Error> {
        let raw: String = match &value.source{
            FileReference::Path(path) => path.clone().into(),
            FileReference::Url(url) => url.clone().into(),
        };
        if raw.to_lowercase().ends_with(".yml") || raw.ends_with(".yaml"){
            Ok(Self(value))
        }else{
            Err(ModelWeightsParsingError::DependenciesNotYaml { path: raw })
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct TensorflowJsWeightsDescr{
    #[serde(flatten)]
    base: WeightsDescrBase,
    /// Version of the TensorFlow library used
    tensorflow_version: Version,
    // The multi-file weights.
    // All required files/folders should be a zip archive."""
    source: FileReference,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct TensorflowSavedModelBundleWeightsDescr{
    #[serde(flatten)]
    base: WeightsDescrBase,

    /// Version of the TensorFlow library used
    tensorflow_version: Version,
    /// Custom dependencies beyond tensorflow.
    /// Should include tensorflow and any version pinning has to be compatible with `tensorflow_version
    #[serde(default)]
    dependencies: Option<EnvironmentFileDescr>,

    /// The multi-file weights.
    /// All required files/folders should be a zip archive
    source: FileReference,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct TorchscriptWeightsDescr{
    #[serde(flatten)]
    base: WeightsDescrBase,
    /// Version of the PyTorch library used
    pytorch_version: Version
}