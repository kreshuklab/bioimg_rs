use std::path::{Path, PathBuf};

use crate::result::Result;
use super::StatefulWidget;

pub trait ParsedFile: Send + 'static {
    fn parse(path: PathBuf, ctx: egui::Context) -> Self;
    fn render(&self, ui: &mut egui::Ui, id: egui::Id);
}

pub enum FileWidgetState<V: Send + 'static> {
    Empty,
    Loading {
        path: PathBuf,
        promise: poll_promise::Promise<V>,
    },
    Finished {
        path: PathBuf,
        value: V,
    },
}

impl<V: Send + 'static> FileWidgetState<V>{
    pub fn loaded_value(&self) -> Option<&V> {
        if let Self::Finished { value, .. } = self {
            Some(value)
        } else {
            None
        }
    }
}

pub struct FileWidget<PF: ParsedFile> {
    pub state: FileWidgetState<PF>,
}

impl<PF: ParsedFile> FileWidget<PF> {
    #[allow(dead_code)]
    pub fn path(&self) -> Option<&Path> {
        match &self.state {
            FileWidgetState::Empty => None,
            FileWidgetState::Loading { path, .. } => Some(path),
            FileWidgetState::Finished { path, .. } => Some(path),
        }
    }
}

impl<PF: ParsedFile> Default for FileWidget<PF> {
    fn default() -> Self {
        Self { state: FileWidgetState::Empty }
    }
}

impl<PF: ParsedFile> StatefulWidget for FileWidget<PF> {
    type Value<'p> = &'p FileWidgetState<PF>;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui| {
            self.state = match std::mem::replace(&mut self.state, FileWidgetState::Empty) {
                FileWidgetState::Empty => {
                    ui.label("None");
                    FileWidgetState::Empty
                }
                FileWidgetState::Finished { path, value } => {
                    ui.label(path.to_string_lossy());
                    value.render(ui, id.with("value"));
                    FileWidgetState::Finished { path, value }
                }
                FileWidgetState::Loading { path, promise } => {
                    ui.ctx().request_repaint();
                    match promise.try_take() {
                        Ok(value) => FileWidgetState::Finished { path, value },
                        Err(promise) => {
                            ui.label("Loading...");
                            FileWidgetState::Loading { path, promise }
                        }
                    }
                }
            };

            if !ui.button("Open...").clicked() {
                return;
            }
            let context = ui.ctx().clone();
            let path_buf = rfd::FileDialog::new().pick_file(); //FIXME: web? async?
            self.state = if let Some(pth) = path_buf {
                FileWidgetState::Loading {
                    path: pth.clone(),
                    promise: poll_promise::Promise::spawn_thread("loading file", move || PF::parse(pth, context)),
                }
            } else {
                FileWidgetState::Empty
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.state
    }
}


impl ParsedFile for Result<PathBuf>{
    fn parse(path: PathBuf, _ctx: egui::Context) -> Self{
        Ok(path)
    }
    fn render(&self, _ui: &mut egui::Ui, _id: egui::Id){

    }
}
