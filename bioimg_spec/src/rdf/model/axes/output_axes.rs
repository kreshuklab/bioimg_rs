use serde::{Deserialize, Serialize};

use crate::rdf::model::axis_size::FixedOrRefAxisSize;
use crate::rdf::model::AnyAxisSize;

use super::{
    impl_axis_group, AxisDescription, AxisId, AxisScale, AxisType, BatchAxis, ChannelAxis,
    Halo, IndexAxis, _default_space_axis_id, _default_time_axis_id
};
use crate::rdf::model::time_unit::TimeUnit;
use crate::rdf::model::space_unit::SpaceUnit;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum OutputSpacetimeSize{
    Haloed{
        size: FixedOrRefAxisSize,
        halo: Halo,
    },
    Standard{size: AnyAxisSize},
}

impl From<AnyAxisSize> for OutputSpacetimeSize{
    fn from(size: AnyAxisSize) -> Self {
        Self::Standard{size}
    }
}

impl OutputSpacetimeSize{
    pub fn size(&self) -> AnyAxisSize{
        match self{
            Self::Standard{ size } => size.clone(),
            Self::Haloed { size, .. } => size.clone().into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeOutputAxis {
    #[serde(default = "_default_time_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: AxisDescription,
    #[serde(default)]
    pub unit: Option<TimeUnit>,
    #[serde(default)]
    pub scale: AxisScale,
    #[serde(flatten)]
    pub size: OutputSpacetimeSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceOutputAxis {
    #[serde(default = "_default_space_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: AxisDescription,
    #[serde(default)]
    pub unit: Option<SpaceUnit>,
    #[serde(default)]
    pub scale: AxisScale,
    #[serde(flatten)]
    pub size: OutputSpacetimeSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum OutputAxis {
    #[serde(rename = "batch")]
    Batch(BatchAxis),
    #[serde(rename = "channel")]
    Channel(ChannelAxis),
    #[serde(rename = "index")]
    Index(IndexAxis),
    #[serde(rename = "time")]
    Time(TimeOutputAxis),
    #[serde(rename = "space")]
    Space(SpaceOutputAxis),
}

impl From<BatchAxis> for OutputAxis{
    fn from(value: BatchAxis) -> Self {
        OutputAxis::Batch(value)
    }
}
impl From<ChannelAxis> for OutputAxis{
    fn from(value: ChannelAxis) -> Self {
        OutputAxis::Channel(value)
    }
}
impl From<IndexAxis> for OutputAxis{
    fn from(value: IndexAxis) -> Self {
        OutputAxis::Index(value)
    }
}
impl From<TimeOutputAxis> for OutputAxis{
    fn from(value: TimeOutputAxis) -> Self {
        OutputAxis::Time(value)
    }
}
impl From<SpaceOutputAxis> for OutputAxis{
    fn from(value: SpaceOutputAxis) -> Self {
        OutputAxis::Space(value)
    }
}

impl OutputAxis{
    pub fn axis_type(&self) -> AxisType {
        match self {
            Self::Batch(_) => AxisType::Batch,
            Self::Channel(_) => AxisType::Channel,
            Self::Index(_) => AxisType::Index,
            Self::Time(_) => AxisType::Time,
            Self::Space(_) => AxisType::Space,
       }
    }

    pub fn id(&self) -> AxisId {
        match self {
            Self::Batch(axis) => AxisId::from(&axis.id),
            Self::Channel(axis) => AxisId::from(&axis.id),
            Self::Index(axis) => AxisId::from(&axis.id),
            Self::Time(axis) => axis.id.clone(),
            Self::Space(axis) => axis.id.clone(),
        }
    }

    pub fn size(&self) -> Option<AnyAxisSize>{
        match self {
            Self::Batch(_) => None,
            Self::Channel(axis) => Some(AnyAxisSize::Fixed(axis.size())),
            Self::Index(axis) => Some(axis.size.clone()),
            Self::Time(axis) => Some(axis.size.size()),
            Self::Space(axis) => Some(axis.size.size()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(try_from = "Vec::<OutputAxis>")]
pub struct OutputAxisGroup(Vec<OutputAxis>);

impl_axis_group!(Output);
