use bioimg_spec::rdf;

use super::{staging_string::StagingString, staging_vec::{ItemWidgetConf, StagingVec}};



pub struct ChannelNameItemConf;

impl ItemWidgetConf for ChannelNameItemConf{
    const ITEM_NAME: &'static str = "Channel Name";
    const INLINE_ITEM: bool = true;
}

pub type ChannelNamesWidget = StagingVec<StagingString<rdf::Identifier<String>>, ChannelNameItemConf>;
