use bioimg_spec::rdf::BoundedString;

use super::{staging_string::StagingString, staging_vec::ItemWidgetConf};

pub type TagWidget = StagingString<BoundedString<3, 1024>>;
impl ItemWidgetConf for TagWidget{
    const ITEM_NAME: &'static str = "Tag";
}