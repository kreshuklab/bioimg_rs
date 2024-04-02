use bioimg_runtime as rt;

use super::{image_widget::ImageWidget, staging_vec::ItemWidgetConf};


//FIXME: maybe this should just be a marker type instead?
impl ItemWidgetConf for ImageWidget<rt::CoverImage>{
    const ITEM_NAME: &'static str = "Cover Image";
    const MIN_NUM_ITEMS: usize = 1;
}