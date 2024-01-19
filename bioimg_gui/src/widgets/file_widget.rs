use std::{path::PathBuf, sync::{Mutex, Arc}};

use crate::task::GenerationalMutex;

use super::DrawAndParse;


#[derive(thiserror::Error, Debug, Clone)]
pub enum FilePickerError{
    #[error("Empty")]
    Empty,
    #[error("Loading")]
    Loading,
    #[error("Could not open {path}: {reason}")]
    IoError{path: PathBuf, reason: String},
}

pub struct LoadedFile{
    path: PathBuf,
    contents: Vec<u8>,
}
impl LoadedFile{
    fn path(&self) -> &PathBuf{
        &self.path
    }
    fn contents(&self) -> &[u8]{
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
    type Parsed<'p> = &'p LoadedFile;
    type Error= FilePickerError;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error>{
        let out = ui.horizontal(|ui|{
            let contents_lock = self.contents.lock().unwrap();
            match &*contents_lock{
                Ok(loaded_file) => ui.label(loaded_file.path.to_string_lossy()),
                Err(_) => ui.label("None"),
            };

            if ui.button("Open...").clicked(){
                // FIXME: async + web
                let path_buf = rfd::FileDialog::new()
                    .set_directory("/")
                    .pick_file();

                'file_read: {
                    let Some(pth) = path_buf else{
                        self.contents = GenerationalMutexArc::new(Err(FilePickerError::Empty)); //FIXME
                        break 'file_read;
                    };

                    {
                        self.contents = Arc::new(Err(FilePickerError::Empty));
                        let mut contents = Arc::clone(&self.contents);
                        std::thread::Builder::new()
                            .name("my_file_loader".into())
                            .spawn(move ||{
                                match std::fs::read(&pth){
                                    Ok(d) => {
                                        *Arc::get_mut(&mut contents).unwrap() = Ok(LoadedFile{path: pth, contents: d});
                                    },
                                    Err(err) => {
                                        *Arc::get_mut(&mut contents).unwrap() = Err(
                                            FilePickerError::IoError { path: pth, reason: err.to_string() }
                                        );
                                    }
                                }
                            })
                            .expect("Could not spawn thread");
                    }
                }
            }

            let x = self.contents.as_ref().as_ref().map_err(|err| err.clone());
            x
        }).inner;
        return out
    }
}
