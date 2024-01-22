use std::{path::PathBuf, sync::Arc};

use parking_lot::{Mutex, MutexGuard, MappedMutexGuard};

use crate::task::run_task;

use super::DrawAndParse;


#[derive(thiserror::Error, Debug, Clone)]
pub enum FilePickerError{
    #[error("Empty")]
    Empty,
    #[error("Loading")]
    Loading{path: PathBuf},
    #[error("Could not open {path}: {reason}")]
    IoError{path: PathBuf, reason: String},
}

pub struct LoadedFile{
    path: PathBuf,
    contents: Vec<u8>,
}
impl LoadedFile{
    pub fn path(&self) -> &PathBuf{
        &self.path
    }
    pub fn contents(&self) -> &[u8]{
        &self.contents
    }
}

pub struct FileWidget{
    contents: Arc<Mutex<Result<LoadedFile, FilePickerError>>>,
}

impl Default for FileWidget{
    fn default() -> Self {
        Self{
            contents: Arc::new(Mutex::new(Err(FilePickerError::Empty)))
        }
    }
}

impl DrawAndParse for FileWidget{
    type Parsed<'p> = MappedMutexGuard<'p, LoadedFile>;
    type Error= FilePickerError;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) -> Result<MappedMutexGuard<'p, LoadedFile>, FilePickerError>{
        ui.horizontal(|ui|{
            let mut contents_lock = self.contents.lock();
            let open_clicked: bool = match &*contents_lock{
                Ok(loaded_file) => {
                    ui.label(loaded_file.path.to_string_lossy());
                    ui.button("Open...").clicked()
                },
                Err(err) => match err {
                    FilePickerError::Empty => {
                        ui.label("None");
                        ui.button("Open...").clicked()
                    },
                    FilePickerError::Loading{path} => {
                        ui.add_enabled(false, egui::Button::new("Loading...")).on_hover_ui(|ui|{
                            ui.label(format!("Loading {}", path.to_string_lossy()));
                        });
                        false
                    },
                    FilePickerError::IoError { path, reason } => {
                        ui.label(format!("Error")).on_hover_ui(|ui|{
                            ui.label(format!("Could not open {}: {reason}", path.to_string_lossy()));
                        });
                        ui.button("Open...").clicked()
                    }
                },
            };

            'file_read: {
                if !open_clicked{
                    break 'file_read;
                }
                let path_buf = rfd::FileDialog::new() //FIXME: web?
                    .set_directory("/")
                    .pick_file();
                let Some(pth) = path_buf else{
                    *contents_lock = Err(FilePickerError::Empty);
                    break 'file_read;
                };

                *contents_lock = Err(FilePickerError::Loading{path: pth.clone()});

                let contents = Arc::clone(&self.contents);
                run_task(move ||{
                    match std::fs::read(&pth){
                        Ok(d) => {
                            *contents.lock() = Ok(LoadedFile{path: pth, contents: d});
                        },
                        Err(err) => {
                            *contents.lock() = Err(
                                FilePickerError::IoError { path: pth, reason: err.to_string() }
                            );
                        }
                    }
                })
            }

            match &mut (*contents_lock){
                Ok(_) => Ok(MutexGuard::map(contents_lock, |v|{
                    v.as_mut().unwrap() //FIXME?
                })),
                Err(err) => Err(err.clone())
            }
        }).inner
    }
}
