use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

use super::{axes::AxisId, tensor_id::TensorId};

pub type FixedAxisSize = NonZeroUsize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxisSizeReference {
    tensor_id: TensorId,
    axis_id: AxisId,
    offset: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterizedAxisSize {
    min: NonZeroUsize,
    step: NonZeroUsize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnyAxisSize {
    Fixed(FixedAxisSize),
    Reference(AxisSizeReference),
    Parameterized(ParameterizedAxisSize),
}
