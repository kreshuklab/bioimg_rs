use serde::{Deserialize, Serialize};

use super::{
    axis_size::{AnyAxisSize, FixedAxisSize},
    channel_name::ChannelNames,
    space_unit::SpaceUnit,
    time_unit::TimeUnit,
};
use crate::rdf::{bounded_string::BoundedString, identifier::Identifier, literal::LiteralInt, lowercase::Lowercase};

pub type AxisId = Lowercase<BoundedString<1, { 16 - 1 }>>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct AxisScale(f32);

impl Default for AxisScale {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(thiserror::Error, PartialEq, Clone, Debug)]
pub enum AxisScaleParsingError {
    #[error("Axis scale is less than 0.0: {0}")]
    LessThanZero(f32),
}

impl TryFrom<f32> for AxisScale {
    type Error = AxisScaleParsingError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(AxisScaleParsingError::LessThanZero(value))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchAxis {
    #[serde(default = "_default_batch_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: BoundedString<0, { 128 - 1 }>,
    #[serde(default)]
    pub size: Option<LiteralInt<1>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelAxis {
    #[serde(default = "_default_channel_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: BoundedString<0, { 128 - 1 }>,
    // pub size: FixedAxisSize,
    pub channel_names: Vec<Identifier<String>>,
    // #[serde(default)]
    // pub channel_names: ChannelNames, // FIXME: do we need to handle "#channel_names" ?
}

impl ChannelAxis {
    pub fn is_compatible_with_extent(&self, extent: usize) -> bool {
        return self.channel_names.len() == extent;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexAxis {
    #[serde(default = "_default_index_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: BoundedString<0, { 128 - 1 }>,
    pub size: AnyAxisSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeInputAxis {
    #[serde(default = "_default_time_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: BoundedString<0, { 128 - 1 }>,
    #[serde(default)]
    pub unit: Option<TimeUnit>,
    #[serde(default)]
    pub scale: AxisScale,
    pub size: AnyAxisSize,
}

impl TimeInputAxis {
    pub fn is_compatible_with_extent(&self, extent: usize) -> bool {
        return self.size.is_compatible_with_extent(extent);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeOutputAxis {
    #[serde(flatten)]
    pub base: TimeInputAxis,
    #[serde(default)]
    pub halo: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceInputAxis {
    #[serde(default = "_default_space_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: BoundedString<0, { 128 - 1 }>,
    #[serde(default)]
    pub unit: Option<SpaceUnit>,
    #[serde(default)]
    pub scale: AxisScale,
    pub size: AnyAxisSize,
}

impl SpaceInputAxis {
    pub fn is_compatible_with_extent(&self, extent: usize) -> bool {
        return self.size.is_compatible_with_extent(extent);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceOutputAxis {
    #[serde(flatten)]
    pub base: SpaceInputAxis,
    #[serde(default)]
    pub halo: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InputAxis {
    #[serde(rename = "batch")]
    Batch(BatchAxis),
    #[serde(rename = "channel")]
    Channel(ChannelAxis),
    #[serde(rename = "index")]
    Index(IndexAxis),
    #[serde(rename = "time")]
    Time(TimeInputAxis),
    #[serde(rename = "space")]
    Space(SpaceInputAxis),
}

impl InputAxis {
    pub fn id(&self) -> &AxisId {
        match self {
            Self::Batch(axis) => &axis.id,
            Self::Channel(axis) => &axis.id,
            Self::Index(axis) => &axis.id,
            Self::Time(axis) => &axis.id,
            Self::Space(axis) => &axis.id,
        }
    }
    pub fn is_compatible_with_extent(&self, extent: usize) -> bool {
        match self {
            Self::Space(space_axis) => space_axis.is_compatible_with_extent(extent),
            Self::Time(time_axis) => time_axis.is_compatible_with_extent(extent),
            Self::Channel(channel_axis) => channel_axis.is_compatible_with_extent(extent),
            _ => true, // FIXME: can we check this?
        }
    }
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

fn _default_batch_axis_id() -> AxisId {
    String::from("batch").try_into().unwrap()
}
fn _default_channel_axis_id() -> AxisId {
    String::from("channel").try_into().unwrap()
}
fn _default_time_axis_id() -> AxisId {
    String::from("time").try_into().unwrap()
}
fn _default_index_axis_id() -> AxisId {
    String::from("index").try_into().unwrap()
}
fn _default_space_axis_id() -> AxisId {
    String::from("x").try_into().unwrap()
}
fn _default_axis_scale() -> f32 {
    1.0
}
