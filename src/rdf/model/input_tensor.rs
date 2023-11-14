use crate::util::ConfigString;

use super::{
    axes::AxisSequence, data_range::DataRange, data_type::DataType, preprocessing::Preprocessing,
};

pub struct InputTensor {
    pub axes: AxisSequence,
    pub data_type: DataType,
    pub name: ConfigString,
    pub shape: Vec<usize>,
    pub data_range: DataRange,
    pub preprocessing: Vec<Preprocessing>,
}
