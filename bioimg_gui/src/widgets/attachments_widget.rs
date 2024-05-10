use super::{file_source_widget::FileSourceWidget, staging_vec::ItemWidgetConf};

pub type AttachmentsWidget = FileSourceWidget;
impl ItemWidgetConf for AttachmentsWidget{
    const ITEM_NAME: &'static str = "Attachment";
}