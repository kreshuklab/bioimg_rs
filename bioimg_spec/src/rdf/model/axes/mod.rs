pub mod output_axes;

use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    ops::Deref,
};

use paste::paste;
use serde::{Deserialize, Serialize};

use super::{
    axis_size::{AnyAxisSize, FixedOrRefAxisSize, QualifiedAxisId, ResolvedAxisSize},
    space_unit::SpaceUnit,
    time_unit::TimeUnit,
};
use crate::rdf::{bounded_string::BoundedString, identifier::Identifier, literal::{declare_lowercase_marker, LitStrMarker, LiteralInt, Marker}, lowercase::Lowercase};

pub type AxisId = Lowercase<BoundedString<1, { 16 - 1 }>>;
pub type AxisDescription = BoundedString<0, { 128 - 1 }>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct AxisScale(f32);

impl Default for AxisScale {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum HaloParsingError {
    #[error("Halo must be a positive integer, found {found}")]
    MustBePositive { found: u64 },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Halo(u64);

impl TryFrom<u64> for Halo {
    type Error = HaloParsingError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(HaloParsingError::MustBePositive { found: value });
        }
        Ok(Self(value))
    }
}

pub trait AxisIdMarker: Marker{}

declare_lowercase_marker!(Batch);
impl AxisIdMarker for Batch{}
declare_lowercase_marker!(Index);
impl AxisIdMarker for Index{}
declare_lowercase_marker!(Channel);
impl AxisIdMarker for Channel{}
declare_lowercase_marker!(Space);
declare_lowercase_marker!(Time);


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpecialAxisId<M: AxisIdMarker>(LitStrMarker<M>);


impl<M: AxisIdMarker> From<&SpecialAxisId<M>> for AxisId {
    fn from(value: &SpecialAxisId<M>) -> AxisId {
        AxisId::try_from(M::NAME.to_owned()).unwrap()
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum SpecialAxisIdParsingError {
    #[error("Expected '{expected}', found '{found}'")]
    BadAxisId { expected: &'static str, found: String },
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

#[derive(thiserror::Error, Debug)]
pub enum AxisResolutionError{
    #[error("Could not resolve axis size")]
    CouldNotResolve,
    #[error("Resolved axis size type is not allowed")]
    NotAllowed,
}

// ///////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchAxis {
    #[serde(rename = "type")]
    pub tag: LitStrMarker<Batch>,
    pub id: SpecialAxisId<Batch>,
    #[serde(default)]
    pub description: AxisDescription,
    #[serde(default)]
    pub size: Option<LiteralInt<1>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelAxis {
    #[serde(rename = "type")]
    pub tag: LitStrMarker<Channel>,
    pub id: SpecialAxisId<Channel>,
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
    #[serde(rename = "type")]
    pub tag: LitStrMarker<Index>,
    pub id: SpecialAxisId<Index>,
    #[serde(default)]
    pub description: AxisDescription,
    pub size: AnyAxisSize,
}

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


#[rustfmt::skip]
macro_rules! impl_resolve_size_with {($axis:ident) => {
    impl $axis{
        pub fn resolve_size_with(
            &mut self,
            size_map: &std::collections::HashMap<
                crate::rdf::model::axis_size::QualifiedAxisId,
                crate::rdf::model::axis_size::ResolvedAxisSize
            >
        ) -> crate::rdf::model::axis_size::ResolvedAxisSize {
            self.size.resolve_with(size_map)
        }
    }
};}
pub(crate) use impl_resolve_size_with;

impl_resolve_size_with!(IndexAxis);
impl_resolve_size_with!(SpaceInputAxis);
impl_resolve_size_with!(TimeInputAxis);

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


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(try_from = "Vec::<InputAxis>")]
pub struct InputAxisGroup(Vec<InputAxis>);

#[rustfmt::skip]
macro_rules!  impl_axis_group{($inout:ident) => { paste::paste!{
    impl [<$inout AxisGroup>] {
        pub fn resolve_sizes_with(&mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>) -> Vec<Option<ResolvedAxisSize>> {
            self.0.iter_mut().map(|axis| axis.resolve_size_with(size_map)).collect()
        }
    }

    impl std::ops::Deref for [<$inout AxisGroup>] {
        type Target = [ [<$inout Axis>] ];

        fn deref(&self) -> &Self::Target {
            return &self.0;
        }
    }

    impl TryFrom<Vec< [<$inout Axis>] >> for [<$inout AxisGroup>] {
        type Error = crate::rdf::model::axes::AxisGroupValidationError;
        fn try_from(value: Vec< [<$inout Axis>] >) -> Result<Self, Self::Error> {
            if value.len() == 0 {
                return Err(crate::rdf::model::axes::AxisGroupValidationError::Empty);
            }
            let mut axis_types = std::collections::HashSet::<AxisType>::with_capacity(5); //FIXME: 5?
            for val in value.iter() {
                if ! matches!(val.axis_type(), AxisType::Space) && !axis_types.insert(val.axis_type()) {
                    return Err(crate::rdf::model::axes::AxisGroupValidationError::RepeatedAxisType(val.axis_type()));
                }
            }
            //FIXME: there's probably other invalid stuff like only Batch or only Index
            return Ok(Self(value));
        }
    }
}};}

pub(crate) use impl_axis_group;

impl_axis_group!(Input);
