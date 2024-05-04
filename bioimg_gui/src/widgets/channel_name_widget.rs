use bioimg_spec::rdf;

use super::{staging_string::StagingString, staging_vec::{ItemWidgetConf, StagingVec}};



pub struct ChannelNameItemConf;

impl ItemWidgetConf for ChannelNameItemConf{
    const ITEM_NAME: &'static str = "Channel Name";
    const INLINE_ITEM: bool = true;
    const MIN_NUM_ITEMS: usize = 1;
}

pub type ChannelNamesWidget = StagingVec<StagingString<rdf::Identifier>, ChannelNameItemConf>;
