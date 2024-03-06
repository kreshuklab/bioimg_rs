use super::{non_empty_list::NonEmptyList, BoundedString};

pub mod axes;
pub mod axis_size;
pub mod channel_name;
pub mod data_range;
pub mod data_type;
pub mod input_tensor;
pub mod output_tensor;
pub mod preprocessing;
pub mod space_unit;
pub mod tensor_data_descr;
pub mod tensor_id;
pub mod time_unit;

pub use axes::{
    AxisId, AxisScale, BatchAxis, ChannelAxis, IndexAxis, InputAxis, InputAxisGroup, OutputAxis, OutputAxisGroup, SpaceInputAxis,
    SpaceOutputAxis, TimeInputAxis, TimeOutputAxis,
};
pub use axis_size::{AnyAxisSize, AxisSizeReference, FixedAxisSize, ParameterizedAxisSize, QualifiedAxisId, ResolvedAxisSize};
pub use input_tensor::InputTensorDescr;
pub use output_tensor::OutputTensorDescr;
pub use space_unit::SpaceUnit;
pub use tensor_id::TensorId;
pub use time_unit::TimeUnit;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ModelRdf {
    // pub base: Rdf,
    pub inputs: NonEmptyList<InputTensorDescr>,
    pub outputs: NonEmptyList<OutputTensorDescr>,
}

pub type TensorDescription = BoundedString<0, 128>;
