use std::path::PathBuf;

use super::{file_widget::FileWidget, staging_vec::ItemWidgetConf};
use crate::result::Result;

pub type AttachmentsWidget = FileWidget<Result<PathBuf>>;
impl ItemWidgetConf for AttachmentsWidget{
    const ITEM_NAME: &'static str = "Attachment";
}