use std::path::{Path, PathBuf};

use super::StatefulWidget;

pub struct PathPickerWidegt{
    path: Option<PathBuf>
}

impl StatefulWidget for PathPickerWidegt{
    type Value<'p> = Option<&'p Path>;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.horizontal(|ui|{
            if ui.button("Open ...").clicked(){
                self.path = rfd::FileDialog::new().pick_file()
            }
            match &self.path{
                None => ui.weak("Empty"),
                Some(path) =>ui.weak(path.to_string_lossy())
            }
        });
    }
    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.path.as_ref().map(|p| p.as_ref())
    }
}
