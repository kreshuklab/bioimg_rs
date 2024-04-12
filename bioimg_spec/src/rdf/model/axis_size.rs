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
#[serde(untagged)]
pub enum AnyAxisSize {
    Fixed(FixedAxisSize),
    Parameterized(ParameterizedAxisSize),
    Reference(AxisSizeReference),
}

impl AnyAxisSize {
    //FIXME: return a ref?
    pub fn resolve_with(&mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>) -> ResolvedAxisSize {
        match self {
            Self::Reference(size_ref) => {
                let resolved = size_ref.resolve_with(size_map);
                *self = resolved.clone().into();
                resolved.clone() //FIXME?
            },
            Self::Fixed(fixed) => ResolvedAxisSize::Fixed(fixed.clone()),
            Self::Parameterized(parameterized) => ResolvedAxisSize::Parameterized(parameterized.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
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

impl From<ResolvedAxisSize> for AnyAxisSize{
    fn from(value: ResolvedAxisSize) -> Self {
        match value{
            ResolvedAxisSize::Fixed(fixed) => AnyAxisSize::Fixed(fixed),
            ResolvedAxisSize::Parameterized(parameterized) => AnyAxisSize::Parameterized(parameterized)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FixedOrRefAxisSize{
    Fixed(FixedAxisSize),
    Reference(AxisSizeReference),
}

#[derive(thiserror::Error, Debug)]
pub enum AxisSizeResolutionError{
    #[error("Parameterized axis size not allowed")]
    ParameterizedNotAllowed(ParameterizedAxisSize)
}

impl From<FixedOrRefAxisSize> for AnyAxisSize{
    fn from(value: FixedOrRefAxisSize) -> Self {
        match value{
            FixedOrRefAxisSize::Fixed(fixed) => AnyAxisSize::Fixed(fixed),
            FixedOrRefAxisSize::Reference(reference) => AnyAxisSize::Reference(reference)
        }
    }
}

impl TryFrom<AnyAxisSize> for FixedOrRefAxisSize{
    type Error = AxisSizeResolutionError;
    fn try_from(value: AnyAxisSize) -> Result<Self, Self::Error> {
        match value{
            AnyAxisSize::Fixed(fixed) => Ok(Self::Fixed(fixed)),
            AnyAxisSize::Reference(reference) => Ok(Self::Reference(reference)),
            AnyAxisSize::Parameterized(parameterized) => Err(AxisSizeResolutionError::ParameterizedNotAllowed(parameterized))
        }
    }
} 
