use std::{borrow::Borrow, error::Error, num::NonZeroUsize};

use serde::{Deserialize, Serialize};

use super::{channel_name::ChannelNames, space_unit::SpaceUnit, tensor_id::TensorId, time_unit::TimeUnit};
use crate::rdf::{
    literal::LiteralInt,
    lowercase::{Lowercase, LowercaseParsingError},
    pegged_string::PeggedString,
};

pub type AxisId = Lowercase<PeggedString<1, { 16 - 1 }>>;

#[derive(thiserror::Error, Debug)]
pub enum AxisSizeParsingError {
    #[error("Bad component:  {source}")]
    BadComponent { source: Box<dyn Error + 'static> },
    #[error("Bad identifier")]
    BadIdentifier { value: String, ident: String },
    #[error("Expected at most 2 period-separated components: '{value}'")]
    WrongNumberOfComponents { value: String },
    #[error("Cant have an empty component before or after the period: '{value}'")]
    EmptyComponent { value: String },
}

impl From<LowercaseParsingError> for AxisSizeParsingError {
    fn from(value: LowercaseParsingError) -> Self {
        AxisSizeParsingError::BadComponent { source: Box::new(value) }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AxisSize {
    Fixed(NonZeroUsize),
    Ref { reference: AxisSizeReference, offset: usize },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub enum AxisSizeReference {
    AxisRef(AxisId),
    TensorAxisRef { tensor_id: TensorId, axis_id: AxisId },
}

impl TryFrom<String> for AxisSizeReference {
    type Error = AxisSizeParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(".").collect();
        match TryInto::<[&str; 1]>::try_into(parts) {
            Ok(single_part) => {
                let axis_id = AxisId::try_from(String::from(single_part[0]))?;
                Ok(AxisSizeReference::AxisRef(axis_id))
            }
            Err(parts) => {
                let Ok(two_parts) = TryInto::<[&str; 2]>::try_into(parts) else {
                    return Err(AxisSizeParsingError::WrongNumberOfComponents { value });
                };
                let tensor_id = TensorId::try_from(String::from(two_parts[0]))?;
                let axis_id = AxisId::try_from(String::from(two_parts[1]))?;
                Ok(AxisSizeReference::TensorAxisRef { tensor_id, axis_id })
            }
        }
    }
}

impl From<AxisSizeReference> for String {
    fn from(value: AxisSizeReference) -> Self {
        match value {
            AxisSizeReference::AxisRef(axis_id) => Borrow::<str>::borrow(&axis_id).into(),
            AxisSizeReference::TensorAxisRef { tensor_id, axis_id } => {
                format!("{}.{}", tensor_id, axis_id)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IndexTimeSpaceAxisSize {
    Parameterized { min: NonZeroUsize, step: NonZeroUsize },
    AxisSize(AxisSize),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchAxis {
    #[serde(default = "_default_batch_axis_id")]
    id: AxisId,
    #[serde(default)]
    description: String,
    #[serde(default)]
    size: Option<LiteralInt<1>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelAxis {
    #[serde(default = "_default_channel_axis_id")]
    id: AxisId,
    #[serde(default)]
    description: String,
    #[serde(default)]
    channel_names: ChannelNames, //FIXME: do we need to handle "#channel_names" ?
    size: Option<AxisSize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexAxis {
    size: IndexTimeSpaceAxisSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeInputAxis {
    #[serde(default = "_default_time_axis_id")]
    id: AxisId,
    #[serde(default)]
    unit: Option<TimeUnit>,
    #[serde(default = "_default_axis_scale")]
    scale: f32, //FIXME: enforce greater than 1
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceInputAxis {
    #[serde(default = "_default_space_axis_id")]
    id: AxisId,
    #[serde(default)]
    unit: Option<SpaceUnit>,
    #[serde(default = "_default_axis_scale")]
    scale: f32, //FIXME: enforce greater than 1
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeOutputAxis {
    #[serde(default = "_default_time_axis_id")]
    id: AxisId,
    #[serde(default)]
    unit: Option<TimeUnit>,
    #[serde(default = "_default_axis_scale")]
    scale: f32, //FIXME: enforce greater than 1
    #[serde(default)]
    halo: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceOutputAxis {
    #[serde(default = "_default_space_axis_id")]
    id: AxisId,
    #[serde(default)]
    unit: Option<SpaceUnit>,
    #[serde(default = "_default_axis_scale")]
    scale: f32, //FIXME: enforce greater than 1
    #[serde(default)]
    halo: usize,
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
fn _default_space_axis_id() -> AxisId {
    String::from("x").try_into().unwrap()
}
fn _default_axis_scale() -> f32 {
    1.0
}
