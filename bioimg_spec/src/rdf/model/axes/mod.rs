pub mod output_axes;
pub mod input_axes;


use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

use super::{
    axis_size::AnyAxisSize,
    FixedAxisSize,
};
use crate::rdf::{bounded_string::BoundedString, identifier::Identifier, literal::{LitStr, LiteralInt, StrMarker}, lowercase::{Lowercase, LowercaseParsingError}, non_empty_list::NonEmptyList};

pub type AxisId = Lowercase<BoundedString<1, { 16 - 1 }>>;
pub type AxisDescription = BoundedString<0, { 128 - 1 }>;

#[derive(thiserror::Error, Debug)]
pub enum AxisIdParsingError{
    #[error("AxisId mut be lowercase: {0}")]
    LowercaseParsingError(#[from] LowercaseParsingError),
    #[error("Axis can't be 'batch'")]
    CantBeBatch,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NonBatchAxisId(AxisId);

impl TryFrom<AxisId> for NonBatchAxisId{
    type Error = AxisIdParsingError;
    fn try_from(value: AxisId) -> Result<Self, Self::Error> {
        let raw_axis_id: &'_ str = value.borrow();
        if raw_axis_id == "batch"{
            Err(AxisIdParsingError::CantBeBatch)
        }else{
            Ok(Self(value))
        }
    }
}

impl TryFrom<String> for NonBatchAxisId{
    type Error = AxisIdParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let axisid = AxisId::try_from(value)?;
        Ok(Self::try_from(axisid)?)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct AxisScale(f32);

impl From<AxisScale> for f32{
    fn from(value: AxisScale) -> Self {
        value.0
    }
}

impl Default for AxisScale {
    fn default() -> Self {
        Self(1.0)
    }
}

//FIXME: why isn't Halo just a NonNegativeU64?
#[derive(thiserror::Error, Debug, Clone)]
pub enum HaloParsingError {
    #[error("Halo must be a positive integer, found {found}")]
    MustBePositive { found: u64 },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Halo(u64);

impl From<Halo> for u64{
    fn from(value: Halo) -> Self {
        value.0
    }
}

impl TryFrom<u64> for Halo {
    type Error = HaloParsingError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(HaloParsingError::MustBePositive { found: value });
        }
        Ok(Self(value))
    }
}

pub trait AxisIdMarker: StrMarker{}

#[derive(Default, Clone, Copy, Debug)]
pub struct Batch;
impl StrMarker for Batch { const NAME: &'static str = "batch"; }
impl AxisIdMarker for Batch{}

#[derive(Default, Clone, Copy, Debug)]
pub struct Index;
impl StrMarker for Index { const NAME: &'static str = "index"; }
impl AxisIdMarker for Index{}

#[derive(Default, Clone, Copy, Debug)]
pub struct Channel;
impl StrMarker for Channel { const NAME: &'static str = "channel"; }
impl AxisIdMarker for Channel{}

#[derive(Default, Clone, Copy, Debug)]
pub struct Space;
impl StrMarker for Space { const NAME: &'static str = "space"; }

#[derive(Default, Clone, Copy, Debug)]
pub struct Time;
impl StrMarker for Time { const NAME: &'static str = "time"; }


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpecialAxisId<M: AxisIdMarker>(#[serde(bound = "M: AxisIdMarker")]LitStr<M>);

impl<M: AxisIdMarker> SpecialAxisId<M>{
    pub fn new() -> Self{
        Self(LitStr::new())
    }
}

impl<M: AxisIdMarker> From<&SpecialAxisId<M>> for AxisId {
    fn from(_value: &SpecialAxisId<M>) -> AxisId {
        AxisId::try_from(M::NAME.to_owned()).unwrap()
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum SpecialAxisIdParsingError {
    #[error("Expected '{expected}', found '{found}'")]
    BadAxisId { expected: &'static str, found: String },
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum AxisType {
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "channel")]
    Channel,
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "time")]
    Time,
    #[default]
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

// ///////////////////////

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BatchAxis {
    pub id: SpecialAxisId<Batch>,
    #[serde(default)]
    pub description: AxisDescription,
    #[serde(default)]
    pub size: Option<LiteralInt<1>>,
}

#[test]
fn test_batch_marker_serialization(){
    match serde_json::to_value(SpecialAxisId::<Batch>::default()).unwrap() {
        serde_json::Value::String(s) => assert_eq!(s, "batch"),
        _ => panic!("Did not serialize batch id into a string"),
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelAxis {
    pub id: SpecialAxisId<Channel>,
    #[serde(default)]
    pub description: AxisDescription,
    // pub size: FixedAxisSize,
    pub channel_names: NonEmptyList<Identifier>,
    // #[serde(default)]
    // pub channel_names: ChannelNames, // FIXME: do we need to handle "#channel_names" ?
}

impl ChannelAxis{
    pub fn size(&self) -> FixedAxisSize{
        self.channel_names.len()
    }
}

impl ChannelAxis {
    pub fn is_compatible_with_extent(&self, extent: usize) -> bool {
        let len: usize = self.channel_names.len().into();
        len == extent
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexAxis {
    pub id: SpecialAxisId<Index>,
    #[serde(default)]
    pub description: AxisDescription,
    pub size: AnyAxisSize,
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

#[derive(thiserror::Error, Debug)]
pub enum AxisGroupValidationError {
    #[error("Tensor axes descriptions cannot be empty")]
    Empty,
    #[error("Repeated Axis type: {0:?}")] //FIXME: don't use debug repr
    RepeatedAxisType(AxisType),
}

macro_rules!  impl_axis_group{($inout:ident) => { paste::paste!{
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
