use super::staging_string::StagingString;
use super::staging_vec::{ItemWidgetConf, StagingVec};

pub struct LinksWidgetConf;

impl ItemWidgetConf for LinksWidgetConf {
    const ITEM_NAME: &'static str = "Link";
    const INLINE_ITEM: bool = true;
    const MIN_NUM_ITEMS: usize = 0;
    const GROUP_FRAME: bool = false;
}

pub type ModelLinksWidget = StagingVec<StagingString<String>, LinksWidgetConf>;
