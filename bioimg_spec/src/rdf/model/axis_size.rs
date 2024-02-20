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

impl AnyAxisSize {
    // FIXME: maybe this should be done in the runtime logic
    pub fn is_compatible_with_extent(&self, extent: usize) -> bool {
        match self {
            Self::Fixed(fixed) => return usize::from(*fixed) == extent,
            Self::Parameterized(ParameterizedAxisSize { min, step }) => {
                let min = usize::from(*min);
                let step = usize::from(*step);
                return (extent - min) % step == 0;
            }
            Self::Reference(_) => return true, //FIXME: gotta resolve this
        }
    }
}
