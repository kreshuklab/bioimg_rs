use std::{fmt::Display, path::{Path, PathBuf}};

use crate::result::{GuiError, Result};
use super::{StatefulWidget, ValueWidget};

pub trait ParsedFile: Send + 'static {
    fn parse(path: PathBuf, ctx: egui::Context) -> Self;
    fn render(&self, ui: &mut egui::Ui, id: egui::Id);
}

pub enum FileWidgetState<V: Send + 'static> {
    Empty,
    AboutToLoad{ path: PathBuf },
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

impl<T, E> FileWidgetState<Result<T, E>>
where
    T: Send + 'static,
    E: Send + 'static + Display,
{
    pub fn ok(&self) -> Result<&T, GuiError>{
        match self{
            Self::Empty => Err(GuiError::new("No file selected".to_owned())),
            Self::AboutToLoad{ .. } | Self::Loading { .. } => Err(GuiError::new("File not loaded yet".to_owned())),
            Self::Finished { value, .. } => value.as_ref().map_err(|err| GuiError::new(format!("{}", err)))
        }
    }
    
}

impl<PF: ParsedFile> ValueWidget for FileWidget<PF>{
    type Value<'a> = PathBuf;
    fn set_value<'a>(&mut self, value: Self::Value<'a>) {
        self.set_path(value)
    }
}

impl<PF: ParsedFile> FileWidget<PF> {
    #[allow(dead_code)]
    pub fn path(&self) -> Option<&Path> {
        match &self.state {
            FileWidgetState::Empty => None,
            FileWidgetState::AboutToLoad { path } => Some(path),
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

impl<T: ParsedFile> FileWidget<T>{
    pub fn set_path(&mut self, path: PathBuf){
        self.state = FileWidgetState::AboutToLoad { path };
    }
}

impl<PF: ParsedFile> StatefulWidget for FileWidget<PF> {
    type Value<'p> = &'p FileWidgetState<PF>;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                if ui.button("Open...").clicked(){
                    if let Some(path) = rfd::FileDialog::new().pick_file(){
                        self.set_path(path);
                        return;
                    }
                }
                self.state = match std::mem::replace(&mut self.state, FileWidgetState::Empty){
                    FileWidgetState::AboutToLoad { path } => {
                        ui.ctx().request_repaint();
                        let texture_name: String = path.to_string_lossy().into();
                        ui.label(format!("Loading {} ...", texture_name));

                        let ctx = ui.ctx().clone();
                        FileWidgetState::Loading {
                            path: path.clone(),
                            promise: poll_promise::Promise::spawn_thread(
                                "loading file",
                                move || { PF::parse(path, ctx) }
                            )
                        }
                    },
                    FileWidgetState::Loading { path, promise } => {
                        ui.ctx().request_repaint();
                        ui.label(format!("Loading {} ...", path.to_string_lossy()));
                        match promise.try_take() {
                            Err(promise) => FileWidgetState::Loading { path, promise },
                            Ok(parsed) => FileWidgetState::Finished{
                                path,
                                value: parsed
                            },
                        }
                    },
                    FileWidgetState::Finished{ path, value } => {
                        ui.weak(path.to_string_lossy());
                        value.render(ui, id.with("parsed"));
                        FileWidgetState::Finished{path, value}
                    },
                    FileWidgetState::Empty => FileWidgetState::Empty,
                };
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.state
    }
}


impl ParsedFile for Result<PathBuf>{
    fn parse(path: PathBuf, _ctx: egui::Context) -> Self{
        if path.exists(){
            Ok(path)
        }else{
            Err(GuiError::new("File does not exist".into()))
        }
    }
    fn render(&self, _ui: &mut egui::Ui, _id: egui::Id){

    }
}
