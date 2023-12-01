use serde::{Deserialize, Serialize};

use crate::util::{ConfigString, LiteralInt};

use super::channel_name::ChannelNames;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Axis {
    #[serde(rename = "batch")]
    BatchAxis {
        #[serde(default = "_default_batch_axis_name")]
        name: ConfigString,
        #[serde(default)]
        description: String,
        #[serde(default)]
        size: Option<LiteralInt<1>>,
    },
    ChannelAxis {
        #[serde(default = "_default_channel_axis_name")]
        name: ConfigString,
        #[serde(default)]
        description: String,
        #[serde(default)]
        channel_names: ChannelNames,
        size: usize,
    },
}

// pub StaticChannelName

fn _default_batch_axis_name() -> ConfigString {
    ConfigString::try_from("b").unwrap()
}
fn _default_channel_axis_name() -> ConfigString {
    ConfigString::try_from("c").unwrap()
}
