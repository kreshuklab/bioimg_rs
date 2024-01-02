use serde::{Deserialize, Serialize};

use crate::{
    rdf::{file_reference::FileReference, non_empty_list::NonEmptyList, pegged_string::PeggedString},
    util::SingleOrMultiple,
};

use super::{
    axes::AxisSequence, axes2::InputAxis, data_range::DataRange, data_type::DataType, preprocessing::Preprocessing,
    tensor_id::TensorId,
};

pub struct InputTensorDescr {
    pub axes: AxisSequence,
    pub data_type: DataType,
    pub name: PeggedString<1, 1023>,
    pub shape: Vec<usize>,
    pub data_range: DataRange,
    pub preprocessing: Vec<Preprocessing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputTensorDescr2 {
    pub id: TensorId,
    #[serde(default = "_default_description")]
    pub description: PeggedString<0, 128>,
    pub axes: NonEmptyList<InputAxis>,
    pub test_tensor: FileReference,
    #[serde(default)]
    pub sample_tensor: Option<FileReference>,
    // #[serde(default = "_default_data_description")]
    // pub data: SingleOrMultiple,
    // pub data_type: DataType,
    // #[serde(default = "_default_input_name")]
    // pub name: PeggedString<1, 1023>,
    // pub shape: Vec<usize>,
    // pub data_range: DataRange,
    // pub preprocessing: Vec<Preprocessing>,
}

fn _default_description() -> PeggedString<0, 128> {
    PeggedString::try_from(String::from("")).unwrap()
}
fn _default_input_name() -> PeggedString<1, 1023> {
    PeggedString::try_from("input").unwrap()
}
