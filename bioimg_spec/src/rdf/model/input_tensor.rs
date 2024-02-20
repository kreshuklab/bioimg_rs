use serde::{Deserialize, Serialize};

use crate::rdf::{bounded_string::BoundedString, file_reference::FileReference, non_empty_list::NonEmptyList};

use super::{InputAxis, TensorDescription, TensorId};

#[derive(Serialize, Deserialize, Debug)]
pub struct InputTensorDescr {
    pub id: TensorId,
    #[serde(default)]
    pub description: TensorDescription,
    pub axes: NonEmptyList<InputAxis>,
    pub test_tensor: FileReference,
    #[serde(default)]
    pub sample_tensor: Option<FileReference>,
}
