use bioimg_spec::rdf;

use super::{staging_string::StagingString, staging_vec::ItemWidgetConf};

pub type TagWidget = StagingString<rdf::Tag>;
impl ItemWidgetConf for TagWidget{
    const ITEM_NAME: &'static str = "Tag";
    const INLINE_ITEM: bool = true;
}