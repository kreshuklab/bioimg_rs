use serde::{Deserialize, Serialize};

use crate::rdf::file_reference::FileReference;

use super::{axes::output_axes::OutputAxisGroup, TensorId, TensorTextDescription};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputTensorDescr {
    pub id: TensorId,
    #[serde(default)]
    pub description: TensorTextDescription,
    pub axes: OutputAxisGroup,
    pub test_tensor: FileReference,
    #[serde(default)]
    pub sample_tensor: Option<FileReference>,
}
