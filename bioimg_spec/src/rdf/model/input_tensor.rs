use serde::{Deserialize, Serialize};

use crate::rdf::file_reference::FileReference;

use super::{axes::InputAxisGroup, TensorTextDescription, TensorId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputTensorDescr {
    pub id: TensorId,
    #[serde(default)]
    pub description: TensorTextDescription,
    pub axes: InputAxisGroup,
    pub test_tensor: FileReference,
    #[serde(default)]
    pub sample_tensor: Option<FileReference>,
}
