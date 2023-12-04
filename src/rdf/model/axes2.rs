use serde::{Deserialize, Serialize};

use crate::{util::{LiteralInt, PeggedString}, rdf::lowercase::Lowercase};

use super::channel_name::ChannelNames;

pub type AxisId = Lowercase<PeggedString<1, {16 - 1}>>;

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
