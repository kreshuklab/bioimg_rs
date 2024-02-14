use serde::{Deserialize, Serialize};

use super::{
    axis_size::{AnyAxisSize, FixedAxisSize},
    channel_name::ChannelNames,
    space_unit::SpaceUnit,
    time_unit::TimeUnit,
};
use crate::rdf::{bounded_string::BoundedString, literal::LiteralInt, lowercase::Lowercase};

pub type AxisId = Lowercase<BoundedString<1, { 16 - 1 }>>;

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
    pub size: FixedAxisSize,
    #[serde(default)]
    pub channel_names: ChannelNames, // FIXME: do we need to handle "#channel_names" ?
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
    pub unit: Option<TimeUnit>,
    #[serde(default = "_default_axis_scale")]
    pub scale: f32, //FIXME: enforce greater than 0
    pub size: AnyAxisSize,
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
    pub unit: Option<SpaceUnit>,
    #[serde(default = "_default_axis_scale")]
    pub scale: f32, //FIXME: enforce greater than 0
    pub size: AnyAxisSize,
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
