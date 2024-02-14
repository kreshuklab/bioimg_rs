use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

use super::{axes::AxisId, tensor_id::TensorId};

pub type FixedAxisSize = NonZeroUsize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxisSizeReference {
    pub tensor_id: TensorId,
    pub axis_id: AxisId,
    pub offset: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterizedAxisSize {
    pub min: NonZeroUsize,
    pub step: NonZeroUsize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnyAxisSize {
    Fixed(FixedAxisSize),
    Reference(AxisSizeReference),
    Parameterized(ParameterizedAxisSize),
}
