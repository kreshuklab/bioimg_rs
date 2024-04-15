use serde::{Deserialize, Serialize};

use crate::rdf::literal::LitStrMarker;
use crate::rdf::model::axis_size::FixedOrRefAxisSize;
use crate::rdf::model::AnyAxisSize;

use super::{impl_axis_group, AxisDescription, AxisId, AxisScale, AxisType, BatchAxis, ChannelAxis, Halo, IndexAxis, Space, Time, _default_space_axis_id, _default_time_axis_id};
use crate::rdf::model::time_unit::TimeUnit;
use crate::rdf::model::space_unit::SpaceUnit;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeOutputAxis {
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
pub struct TimeOutputAxisWithHalo {
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
    pub size: FixedOrRefAxisSize,
    pub halo: Halo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceOutputAxis {
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
pub struct SpaceOutputAxisWithHalo {
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
    pub size: FixedOrRefAxisSize,
    pub halo: Halo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OutputAxis {
    Batch(BatchAxis),
    Channel(ChannelAxis),
    Index(IndexAxis),
    TimeWithHalo(TimeOutputAxisWithHalo),  // haloed must come first since it has more fields
    Time(TimeOutputAxis),
    SpaceWithHalo(SpaceOutputAxisWithHalo), // haloed must come first since it has more fields
    Space(SpaceOutputAxis),
}

impl OutputAxis{
    pub fn axis_type(&self) -> AxisType {
        match self {
            Self::Batch(_) => AxisType::Batch,
            Self::Channel(_) => AxisType::Channel,
            Self::Index(_) => AxisType::Index,
            Self::Time(_) => AxisType::Time,
            Self::TimeWithHalo(_) => AxisType::Time,
            Self::Space(_) => AxisType::Space,
            Self::SpaceWithHalo(_) => AxisType::Space,
       }
    }

    pub fn id(&self) -> AxisId {
        match self {
            Self::Batch(axis) => AxisId::from(&axis.id),
            Self::Channel(axis) => AxisId::from(&axis.id),
            Self::Index(axis) => AxisId::from(&axis.id),
            Self::Time(axis) => axis.id.clone(),
            Self::TimeWithHalo(axis) => axis.id.clone(),
            Self::Space(axis) => axis.id.clone(),
            Self::SpaceWithHalo(axis) => axis.id.clone(),
        }
    }

    pub fn size(&self) -> Option<AnyAxisSize>{
        match self {
            Self::Batch(_) => None,
            Self::Channel(axis) => Some(AnyAxisSize::Fixed(axis.size())),
            Self::Index(axis) => Some(axis.size.clone()),
            Self::Time(axis) => Some(axis.size.clone()),
            Self::TimeWithHalo(axis) => Some(axis.size.clone().into()),
            Self::Space(axis) => Some(axis.size.clone()),
            Self::SpaceWithHalo(axis) => Some(axis.size.clone().into()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(try_from = "Vec::<OutputAxis>")]
pub struct OutputAxisGroup(Vec<OutputAxis>);

impl_axis_group!(Output);
