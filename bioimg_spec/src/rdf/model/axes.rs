use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use paste::paste;
use serde::{Deserialize, Serialize};

use super::{
    axis_size::{AnyAxisSize, FixedAxisSize, QualifiedAxisId, ResolvedAxisSize},
    channel_name::ChannelNames,
    space_unit::SpaceUnit,
    time_unit::TimeUnit,
};
use crate::rdf::{bounded_string::BoundedString, identifier::Identifier, literal::LiteralInt, lowercase::Lowercase};

pub type AxisId = Lowercase<BoundedString<1, { 16 - 1 }>>;
pub type AxisDescription = BoundedString<0, { 128 - 1 }>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct AxisScale(f32);

impl Default for AxisScale {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AxisType {
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "channel")]
    Channel,
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "space")]
    Space,
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
    pub description: AxisDescription,
    #[serde(default)]
    pub size: Option<LiteralInt<1>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelAxis {
    #[serde(default = "_default_channel_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: AxisDescription,
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
    pub description: AxisDescription,
    pub size: AnyAxisSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeInputAxis {
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
pub struct TimeOutputAxis {
    #[serde(default = "_default_time_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: AxisDescription,
    #[serde(default)]
    pub unit: Option<TimeUnit>,
    #[serde(default)]
    pub scale: AxisScale,
    pub size: AnyAxisSize,
    #[serde(default)]
    pub halo: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpaceInputAxis {
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
pub struct SpaceOutputAxis {
    #[serde(default = "_default_space_axis_id")]
    pub id: AxisId,
    #[serde(default)]
    pub description: AxisDescription,
    #[serde(default)]
    pub unit: Option<SpaceUnit>,
    #[serde(default)]
    pub scale: AxisScale,
    pub size: AnyAxisSize,
    #[serde(default)]
    pub halo: usize,
}

#[rustfmt::skip]
macro_rules! impl_resolve_size_with {($axis_kind:ident) => {paste!{
    impl [<$axis_kind Axis>]{
        pub fn resolve_size_with(&mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>) -> ResolvedAxisSize {
            self.size.resolve_with(size_map)
        }
    }
}};}

impl_resolve_size_with!(Index);
impl_resolve_size_with!(SpaceInput);
impl_resolve_size_with!(TimeInput);
impl_resolve_size_with!(SpaceOutput);
impl_resolve_size_with!(TimeOutput);

#[rustfmt::skip]
macro_rules! declare_axis_enum {($inout:ident) => { paste!{
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type")]
    pub enum [<$inout Axis>] {
        #[serde(rename = "batch")]
        Batch(BatchAxis),
        #[serde(rename = "channel")]
        Channel(ChannelAxis),
        #[serde(rename = "index")]
        Index(IndexAxis),
        #[serde(rename = "time")]
        Time([<Time $inout Axis>]),
        #[serde(rename = "space")]
        Space([<Space $inout Axis>]),
    }
}};}

declare_axis_enum!(Input);
declare_axis_enum!(Output);

#[rustfmt::skip]
macro_rules! impl_axis_enum {($inout:ident) => { paste! {
    impl [<$inout Axis>]{
        pub fn axis_type(&self) -> AxisType {
            match self {
                Self::Batch(_) => AxisType::Batch,
                Self::Channel(_) => AxisType::Channel,
                Self::Index(_) => AxisType::Index,
                Self::Time(_) => AxisType::Time,
                Self::Space(_) => AxisType::Space,
           }
        }

        pub fn id(&self) -> &AxisId {
            match self {
                Self::Batch(axis) => &axis.id,
                Self::Channel(axis) => &axis.id,
                Self::Index(axis) => &axis.id,
                Self::Time(axis) => &axis.id,
                Self::Space(axis) => &axis.id,
            }
        }

        pub fn resolve_size_with(&mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>) -> Option<ResolvedAxisSize> {
            match self {
                Self::Index(axis) => Some(axis.size.resolve_with(size_map)),
                Self::Time(axis) => Some(axis.size.resolve_with(size_map)),
                Self::Space(axis) => Some(axis.size.resolve_with(size_map)),
                _ => None,
            }
        }

        pub fn size(&self) -> Option<&AnyAxisSize>{
            match self {
                Self::Index(axis) => Some(&axis.size),
                Self::Time(axis) => Some(&axis.size),
                Self::Space(axis) => Some(&axis.size),
                _ => None,
            }
        }
    }
}};}

impl_axis_enum!(Input);
impl_axis_enum!(Output);

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

#[derive(thiserror::Error, Debug)]
pub enum AxisGroupValidationError {
    #[error("Tensor axes descriptions cannot be empty")]
    Empty,
    #[error("Repeated Axis type: {0:?}")] //FIXME: don't use debug repr
    RepeatedAxisType(AxisType),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(try_from = "Vec::<OutputAxis>")]
pub struct OutputAxisGroup(Vec<OutputAxis>);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(try_from = "Vec::<InputAxis>")]
pub struct InputAxisGroup(Vec<InputAxis>);

#[rustfmt::skip]
macro_rules!  impl_axis_group{($inout:ident) => { paste!{
    impl [<$inout AxisGroup>] {
        pub fn resolve_sizes_with(&mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>) -> Vec<Option<ResolvedAxisSize>> {
            self.0.iter_mut().map(|axis| axis.resolve_size_with(size_map)).collect()
        }
    }

    impl Deref for [<$inout AxisGroup>] {
        type Target = [ [<$inout Axis>] ];

        fn deref(&self) -> &Self::Target {
            return &self.0;
        }
    }

    impl TryFrom<Vec< [<$inout Axis>] >> for [<$inout AxisGroup>] {
        type Error = AxisGroupValidationError;
        fn try_from(value: Vec< [<$inout Axis>] >) -> Result<Self, Self::Error> {
            if value.len() == 0 {
                return Err(AxisGroupValidationError::Empty);
            }
            let mut axis_types = HashSet::<AxisType>::with_capacity(5);
            for val in value.iter() {
                if !axis_types.insert(val.axis_type()) {
                    return Err(AxisGroupValidationError::RepeatedAxisType(val.axis_type()));
                }
            }
            //FIXME: there's probably other invalid stuff like only Batch or only Index
            return Ok(Self(value));
        }
    }
}};}

impl_axis_group!(Input);
impl_axis_group!(Output);
