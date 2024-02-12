use std::{path::PathBuf, thread::JoinHandle};

use super::StatefulWidget;

pub trait ParsedFile: Send + 'static {
    fn parse(path: PathBuf, ctx: egui::Context) -> Self;
    fn render(&self, ui: &mut egui::Ui, id: egui::Id);
}

pub enum FileWidgetState<V> {
    Empty,
    Loading { path: PathBuf, promise: JoinHandle<V> },
    Finished { path: PathBuf, value: V },
    Failed { path: PathBuf, reason: String },
}

pub struct FileWidget<PF: ParsedFile> {
    state: FileWidgetState<PF>,
}

impl<PF: ParsedFile> FileWidget<PF> {
    pub fn loaded_value(&self) -> Option<&PF> {
        if let FileWidgetState::Finished { value, .. } = &self.state {
            Some(value)
        } else {
            None
        }
    }
}

impl<PF: ParsedFile> Default for FileWidget<PF> {
    fn default() -> Self {
        Self {
            state: FileWidgetState::Empty,
        }
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
                FileWidgetState::Failed { path, reason } => {
                    ui.label(format!("Could not load file")); //FIMXE: tooltip with reason?
                    FileWidgetState::Failed { path, reason }
                }
                FileWidgetState::Finished { path, value } => {
                    ui.label(path.to_string_lossy());
                    value.render(ui, id.with("value"));
                    FileWidgetState::Finished { path, value }
                }
                FileWidgetState::Loading { path, promise } => {
                    ui.ctx().request_repaint();
                    if promise.is_finished() {
                        match promise.join() {
                            Err(_) => FileWidgetState::Failed {
                                path,
                                reason: "Could not join thread".into(),
                            },
                            Ok(value) => FileWidgetState::Finished { path, value },
                        }
                    } else {
                        ui.label("Loading...");
                        FileWidgetState::Loading { path, promise }
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
                    promise: std::thread::spawn(move || PF::parse(pth, context)),
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
