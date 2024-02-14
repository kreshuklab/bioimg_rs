use super::Rdf;

pub mod axes;
pub mod axis_size;
pub mod channel_name;
pub mod data_range;
pub mod data_type;
pub mod input_tensor;
pub mod preprocessing;
pub mod space_unit;
pub mod tensor_data_descr;
pub mod tensor_id;
pub mod time_unit;

pub use axis_size::{AnyAxisSize, AxisSizeReference, FixedAxisSize, ParameterizedAxisSize};

pub struct ModelRdf {
    pub base: Rdf,
    // inputs: u32
}

pub struct ModelRdfV05 {}
