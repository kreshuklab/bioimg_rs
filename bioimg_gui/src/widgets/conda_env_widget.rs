// pub struct CondaEnvWidget{}

use bioimg_runtime as rt;

use crate::result::Result;
use super::{error_display::show_if_error, file_widget::ParsedFile};

impl ParsedFile for Result<rt::CondaEnv>{
    fn parse(path: std::path::PathBuf, _ctx: egui::Context) -> Self {
        let yaml_file = std::fs::File::open(&path)?;
        Ok(rt::CondaEnv::try_load(yaml_file)?)
    }

    fn render(&self, ui: &mut egui::Ui, _id: egui::Id) {
        show_if_error(ui, self)
    }
}
