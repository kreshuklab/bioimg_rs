use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

use crate::rdf::{lowercase::Lowercase, literal::LiteralInt, pegged_string::PeggedString};

use super::channel_name::ChannelNames;

pub type AxisId = Lowercase<PeggedString<1, {16 - 1}>>;

pub enum AxisSize{
    Fixed(NonZeroUsize),
    Reference,
}

// pub struct TensorAxisId{
//     tensor_id: String, //FIXME
//     axis_id: AxisId,
// }

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
