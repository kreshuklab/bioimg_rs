use super::Rdf;

pub mod axes;
pub mod data_range;
pub mod data_type;
pub mod input_tensor;
pub mod preprocessing;
pub mod shape;

pub struct ModelRdf {
    pub base: Rdf,
    // inputs: u32
}
