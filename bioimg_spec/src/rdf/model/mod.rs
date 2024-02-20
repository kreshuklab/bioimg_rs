use super::{BoundedString, Rdf};

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

pub use axes::{AxisId, AxisScale, BatchAxis, ChannelAxis, IndexAxis, InputAxis, SpaceInputAxis, TimeInputAxis};
pub use axis_size::{AnyAxisSize, AxisSizeReference, FixedAxisSize, ParameterizedAxisSize};
pub use input_tensor::InputTensorDescr;
pub use space_unit::SpaceUnit;
pub use tensor_id::TensorId;
pub use time_unit::TimeUnit;

pub struct ModelRdf {
    pub base: Rdf,
    // inputs: u32
}

pub struct ModelRdfV05 {}

pub type TensorDescription = BoundedString<0, 128>;
