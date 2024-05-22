use super::staging_vec::ItemWidgetConf;

pub struct CoverImageItemConf;

//FIXME: maybe this should just be a marker type instead?
impl ItemWidgetConf for CoverImageItemConf{
    const ITEM_NAME: &'static str = "Cover Image";
    const MIN_NUM_ITEMS: usize = 1;
}
