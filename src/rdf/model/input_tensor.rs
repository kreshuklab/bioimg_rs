use serde::{Deserialize, Serialize};

use crate::rdf::pegged_string::PeggedString;

use super::{axes::AxisSequence, data_range::DataRange, data_type::DataType, preprocessing::Preprocessing};

pub struct InputTensor {
    pub axes: AxisSequence,
    pub data_type: DataType,
    pub name: PeggedString<1, 1023>,
    pub shape: Vec<usize>,
    pub data_range: DataRange,
    pub preprocessing: Vec<Preprocessing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputTensor2 {
    pub axes: AxisSequence,
    pub data_type: DataType,
    #[serde(default = "_default_input_name")]
    pub name: PeggedString<1, 1023>,
    #[serde(default)]
    pub description: String,
    pub shape: Vec<usize>,
    pub data_range: DataRange,
    pub preprocessing: Vec<Preprocessing>,
}

fn _default_input_name() -> PeggedString<1, 1023> {
    PeggedString::try_from("input").unwrap()
}
