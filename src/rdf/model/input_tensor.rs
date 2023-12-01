use serde::{Deserialize, Serialize};

use crate::util::ConfigString;

use super::{axes::AxisSequence, data_range::DataRange, data_type::DataType, preprocessing::Preprocessing};

pub struct InputTensor {
    pub axes: AxisSequence,
    pub data_type: DataType,
    pub name: ConfigString,
    pub shape: Vec<usize>,
    pub data_range: DataRange,
    pub preprocessing: Vec<Preprocessing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputTensor2 {
    pub axes: AxisSequence,
    pub data_type: DataType,
    #[serde(default = "_default_input_name")]
    pub name: ConfigString,
    #[serde(default)]
    pub description: String,
    pub shape: Vec<usize>,
    pub data_range: DataRange,
    pub preprocessing: Vec<Preprocessing>,
}

fn _default_input_name() -> ConfigString {
    ConfigString::try_from("input").unwrap()
}
