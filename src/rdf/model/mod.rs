use super::Rdf;

pub mod axes;
pub mod axes2;
pub mod channel_name;
pub mod data_range;
pub mod data_type;
pub mod input_tensor;
pub mod preprocessing;
pub mod shape;
pub mod size;
pub mod tensor_id;
pub mod time_unit;

pub struct ModelRdf {
    pub base: Rdf,
    // inputs: u32
}

pub struct ModelRdfV05 {}
