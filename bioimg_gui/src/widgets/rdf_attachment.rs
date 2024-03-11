use std::path::PathBuf;

use bioimg_runtime as rt;

use super::{error_display::show_error, file_widget::ParsedFile};
use crate::result::Result;

impl ParsedFile for Result<rt::LocalRdfAttachment> {
    fn parse(path: PathBuf, _ctx: egui::Context) -> Self {
        Ok(rt::LocalRdfAttachment::new(path)?)
    }

    fn render(&self, ui: &mut egui::Ui, _id: egui::Id) {
        let attachment = match self {
            Ok(attachment) => attachment,
            Err(err) => {
                show_error(ui, err.to_string());
                return;
            }
        };
    }
}
