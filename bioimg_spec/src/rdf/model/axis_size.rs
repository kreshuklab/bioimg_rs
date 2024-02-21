use std::{collections::HashMap, fmt::Display, num::NonZeroUsize};

use serde::{Deserialize, Serialize};

use super::{axes::AxisId, tensor_id::TensorId};

pub type FixedAxisSize = NonZeroUsize;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub struct QualifiedAxisId {
    pub tensor_id: TensorId,
    pub axis_id: AxisId,
}

impl Display for QualifiedAxisId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.tensor_id, self.axis_id)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxisSizeReference {
    #[serde(flatten)]
    pub qualified_axis_id: QualifiedAxisId,
    pub offset: usize,
}

impl Display for AxisSizeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, offset={}", self.qualified_axis_id, self.offset)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterizedAxisSize {
    pub min: NonZeroUsize,
    pub step: NonZeroUsize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnyAxisSize {
    Resolved(ResolvedAxisSize),
    Reference(AxisSizeReference),
}

impl AnyAxisSize {
    //FIXME: return a ref?
    pub fn resolve_with(&mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>) -> ResolvedAxisSize {
        match self {
            Self::Reference(size_ref) => {
                let resolved = size_map.get(&size_ref.qualified_axis_id).unwrap().clone();
                *self = Self::Resolved(resolved.clone());
                resolved
            }
            Self::Resolved(resolved) => resolved.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResolvedAxisSize {
    Fixed(FixedAxisSize),
    Parameterized(ParameterizedAxisSize),
}
impl ResolvedAxisSize {
    pub fn is_compatible_with_extent(&self, extent: usize) -> bool {
        match self {
            Self::Fixed(fixed) => return usize::from(*fixed) == extent,
            Self::Parameterized(ParameterizedAxisSize { min, step }) => {
                let min = usize::from(*min);
                let step = usize::from(*step);
                return (extent - min) % step == 0;
            }
        }
    }
}
