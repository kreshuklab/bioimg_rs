use crate::util::ConfigString;

use super::{
    axes::{AxisLabel, AxisSequence},
    data_range::DataRange,
};

pub struct InputTensor {
    pub axes: AxisSequence,
    // data_type: DataType,
    pub name: ConfigString,
    pub shape: Vec<usize>,
    pub data_range: DataRange,
    // preprocessing: c
}
