use std::{fmt::Display, num::NonZeroUsize};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResolvedAxisSize {
    Fixed(FixedAxisSize),
    Parameterized(ParameterizedAxisSize),
}

impl From<FixedAxisSize> for ResolvedAxisSize{
    fn from(value: FixedAxisSize) -> Self {
        Self::Fixed(value)
    }
}

impl From<ParameterizedAxisSize> for ResolvedAxisSize{
    fn from(value: ParameterizedAxisSize) -> Self {
        Self::Parameterized(value)
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

impl From<FixedOrRefAxisSize> for AnyAxisSize{
    fn from(value: FixedOrRefAxisSize) -> Self {
        match value{
            FixedOrRefAxisSize::Fixed(fixed) => AnyAxisSize::Fixed(fixed),
            FixedOrRefAxisSize::Reference(reference) => AnyAxisSize::Reference(reference)
        }
    }
}

impl TryFrom<AnyAxisSize> for FixedOrRefAxisSize{
    type Error = ParameterizedAxisSize;
    fn try_from(value: AnyAxisSize) -> Result<Self, Self::Error> {
        match value{
            AnyAxisSize::Fixed(fixed) => Ok(Self::Fixed(fixed)),
            AnyAxisSize::Reference(reference) => Ok(Self::Reference(reference)),
            AnyAxisSize::Parameterized(parameterized) => Err(parameterized),
        }
    }
}
