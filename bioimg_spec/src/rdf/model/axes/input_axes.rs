use serde::{Deserialize, Serialize};

use crate::rdf::{model::{AnyAxisSize, SpaceUnit, TimeUnit}, LitStrMarker};

use super::{AxisDescription, AxisId, AxisScale, AxisType, BatchAxis, ChannelAxis, IndexAxis, Space, Time, _default_space_axis_id, _default_time_axis_id, impl_axis_group};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeInputAxis {
    #[serde(rename = "type")]
    pub tag: LitStrMarker<Time>,
    #[serde(default = "_default_time_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: AxisDescription,
    #[serde(default)]
    pub unit: Option<TimeUnit>,
    #[serde(default)]
    pub scale: AxisScale,
    pub size: AnyAxisSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceInputAxis {
    #[serde(rename = "type")]
    pub tag: LitStrMarker<Space>,
    #[serde(default = "_default_space_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: AxisDescription,
    #[serde(default)]
    pub unit: Option<SpaceUnit>,
    #[serde(default)]
    pub scale: AxisScale,
    pub size: AnyAxisSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputAxis {
    Batch(BatchAxis),
    Channel(ChannelAxis),
    Index(IndexAxis),
    Time(TimeInputAxis),
    Space(SpaceInputAxis),
}


impl InputAxis{
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
            Self::Time(axis) => Some(axis.size.clone()),
            Self::Space(axis) => Some(axis.size.clone()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(try_from = "Vec::<InputAxis>")]
pub struct InputAxisGroup(Vec<InputAxis>);

impl_axis_group!(Input);
