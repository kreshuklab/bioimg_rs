use super::{collapsible_widget::CollapsibleWidget, file_source_widget::FileSourceWidget, staging_vec::ItemWidgetConf};

pub type AttachmentsWidget = FileSourceWidget;
impl ItemWidgetConf for CollapsibleWidget<AttachmentsWidget>{
    const ITEM_NAME: &'static str = "Attachment";
}
