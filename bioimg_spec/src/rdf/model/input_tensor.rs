use serde::{Deserialize, Serialize};

use crate::rdf::{file_reference::FileReference, FileDescription};

use super::{axes::input_axes::InputAxisGroup, preprocessing::PreprocessingDescr, TensorId, TensorTextDescription};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputTensorDescr {
    /// Input tensor id. No duplicates are allowed across all inputs and outputs.
    pub id: TensorId,

    /// indicates that this tensor is optional when doing inference
    #[serde(default)]
    pub optional: bool,

    /// Description of how this input should be preprocessed.
    ///
    /// notes:
    /// - If preprocessing does not start with an 'ensure_dtype' entry, it is added
    ///   to ensure an input tensor's data type matches the input tensor's data description.
    /// - If preprocessing does not end with an 'ensure_dtype' or 'binarize' entry, an
    ///   'ensure_dtype' step is added to ensure preprocessing steps are not unintentionally
    ///   changing the data type.
    pub preprocessing: Vec<PreprocessingDescr>,

    #[serde(default)]
    pub description: TensorTextDescription,
    pub axes: InputAxisGroup,
    pub test_tensor: FileDescription,
    #[serde(default)]
    pub sample_tensor: Option<FileReference>,
}
