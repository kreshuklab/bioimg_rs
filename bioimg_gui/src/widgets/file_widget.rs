use std::{path::PathBuf, thread::JoinHandle};

use super::DrawAndParse;

#[derive(thiserror::Error, Debug, Clone)]
pub enum FilePickerError {
    #[error("Could not open {path}: {reason}")]
    IoError { path: PathBuf, reason: String },
    #[error("Could not join loader thread")]
    ThreadJoinError,
}

pub enum FileWidgetState {
    Empty,
    Loading {
        path: PathBuf,
        resolver: JoinHandle<std::io::Result<Vec<u8>>>,
    },
    Loaded {
        path: PathBuf,
        data: Vec<u8>,
    },
    Failed(FilePickerError),
}

pub struct FileWidget {
    state: FileWidgetState,
}

impl Default for FileWidget {
    fn default() -> Self {
        Self {
            state: FileWidgetState::Empty,
        }
    }
}

impl DrawAndParse for FileWidget {
    type Value<'p> = &'p FileWidgetState;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) -> &'p FileWidgetState {
        ui.horizontal(|ui| {
            self.state = match std::mem::replace(&mut self.state, FileWidgetState::Empty) {
                FileWidgetState::Empty => {
                    ui.label("None");
                    FileWidgetState::Empty
                }
                FileWidgetState::Failed(err) => {
                    ui.label(format!("Could not load file"));
                    FileWidgetState::Failed(err)
                }
                FileWidgetState::Loaded { path, data } => {
                    ui.label(path.to_string_lossy());
                    FileWidgetState::Loaded { path, data }
                }
                FileWidgetState::Loading { path, resolver } => 'joining: {
                    if !resolver.is_finished() {
                        ui.label("Loading...");
                        break 'joining FileWidgetState::Loading { path, resolver };
                    }
                    let join_value = match resolver.join() {
                        Err(join_err) => break 'joining FileWidgetState::Failed(FilePickerError::ThreadJoinError),
                        Ok(val) => val,
                    };
                    match join_value {
                        Ok(data) => FileWidgetState::Loaded { path, data },
                        Err(err) => FileWidgetState::Failed(FilePickerError::IoError {
                            path,
                            reason: err.to_string(),
                        }),
                    }
                }
            };

            if !ui.button("Open...").clicked() {
                return;
            }
            self.state = 'open_dialog: {
                let path_buf = rfd::FileDialog::new() //FIXME: web?
                    .pick_file();
                let Some(pth) = path_buf else {
                    break 'open_dialog FileWidgetState::Empty;
                };
                break 'open_dialog FileWidgetState::Loading {
                    path: pth.clone(),
                    resolver: std::thread::spawn(move || std::fs::read(pth)),
                };
            }
        });
        return &self.state;
    }
}
