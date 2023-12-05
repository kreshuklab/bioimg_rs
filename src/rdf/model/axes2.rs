use std::{num::NonZeroUsize, error::Error};

use serde::{Deserialize, Serialize};

use crate::rdf::{lowercase::{Lowercase, LowercaseParsingError}, literal::LiteralInt, pegged_string::PeggedString, identifier::Identifier};

use super::{channel_name::ChannelNames, tensor_id::TensorId};

pub type AxisId = Lowercase<PeggedString<1, {16 - 1}>>;

#[derive(thiserror::Error, Debug)]
pub enum AxisSizeParsingError{
    #[error("Bad component:  {source}")]
    BadComponent{source: Box<dyn Error + 'static>},
    #[error("Bad identifier")]
    BadIdentifier{value: String, ident: String},
    #[error("Expected at most 2 period-separated components: '{value}'")]
    WrongNumberOfComponents{value: String},
    #[error("Cant have an empty component before or after the period: '{value}'")]
    EmptyComponent{value: String},
}

impl From<LowercaseParsingError> for AxisSizeParsingError{
    fn from(value: LowercaseParsingError) -> Self {
        AxisSizeParsingError::BadComponent { source: Box::new(value) }
    }
}

pub enum AxisSize{
    Fixed(NonZeroUsize),
    Ref{reference: AxisSizeReference, offset: usize},
}

pub enum AxisSizeReference{
    AxisReference(AxisId),
    TensorAxisReference{tensor_id: TensorId, axis_id: AxisId}
}

impl TryFrom<String> for AxisSizeReference{
    type Error = AxisSizeParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(".").collect();
        match TryInto::<[&str; 1]>::try_into(parts){
            Ok(single_part) => {
                let axis_id = AxisId::try_from(String::from(single_part[0]))?;
                Ok(AxisSizeReference::AxisReference(axis_id))
            }
            Err(parts) => {
                let Ok(two_parts) = TryInto::<[&str; 1]>::try_into(parts) else{
                    return Err(AxisSizeParsingError::WrongNumberOfComponents { value })
                };
                let tensor_id = TensorId::try_from(String::from(two_parts[0]))?;
                let axis_id = AxisId::try_from(String::from(two_parts[1]))?;
                Ok(AxisSizeReference::TensorAxisReference { tensor_id, axis_id })
            },
        }
    }
}

pub struct TensorAxisId{
    pub axis_id: AxisId,
    pub tensor_id: Identifier<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Axis {
    #[serde(rename = "batch")]
    BatchAxis {
        #[serde(default = "_default_batch_axis_name")]
        id: AxisId,
        #[serde(default)]
        description: String,
        #[serde(default)]
        size: Option<LiteralInt<1>>,
    },
    #[serde(rename = "channel")]
    ChannelAxis {
        #[serde(default = "_default_channel_axis_name")]
        id: AxisId,
        #[serde(default)]
        description: String,
        #[serde(default)]
        channel_names: ChannelNames,
        size: usize,
    },
}

// pub StaticChannelName

fn _default_batch_axis_name() -> AxisId {
    AxisId::try_from(String::from("b")).unwrap()
}
fn _default_channel_axis_name() -> AxisId {
    AxisId::try_from(String::from("c")).unwrap()
}
