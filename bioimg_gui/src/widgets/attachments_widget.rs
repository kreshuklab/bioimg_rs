use super::{file_widget::FileWidget, staging_vec::ItemWidgetConf};
use crate::result::Result;

pub type AttachmentsWidget = FileWidget<Result<std::fs::File>>;
impl ItemWidgetConf for AttachmentsWidget{
    const ITEM_NAME: &'static str = "Attachment";
}