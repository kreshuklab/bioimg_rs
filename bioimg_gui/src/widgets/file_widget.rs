use std::{fmt::Display, path::{Path, PathBuf}, sync::Arc};

use crate::{project_data::FileWidgetRawData, result::{GuiError, Result}};
use super::{error_display::show_error, Restore, StatefulWidget, ValueWidget};

pub trait ParsedFile: Send + 'static {
    fn parse(path: PathBuf, ctx: egui::Context) -> Self;
    fn render(&self, ui: &mut egui::Ui, id: egui::Id);
}

#[derive(Default)]
pub enum FileWidget<V: Send + 'static> {
    #[default]
    Empty,
    AboutToLoad{ path: Arc<Path> },
    Loading {
        path: Arc<Path>,
        promise: poll_promise::Promise<V>,
    },
    Finished {
        path: Arc<Path>,
        value: V,
    },
}

impl<V: Send + 'static> FileWidget<V>{
    pub fn loaded_value(&self) -> Option<&V> {
        if let Self::Finished { value, .. } = self {
            Some(value)
        } else {
            None
        }
    }
}

impl<T: Send + 'static> Restore for FileWidget<T>{
    type RawData = FileWidgetRawData;
    fn dump(&self) -> Self::RawData {
        match self{
            Self::Empty => FileWidgetRawData::Empty,
            Self::AboutToLoad{path, ..} | Self::Loading{path, ..} | Self::Finished{path, ..} => {
                FileWidgetRawData::AboutToLoad{path: path.as_ref().to_owned()}
            }
        }
    }
    fn restore(&mut self, raw: Self::RawData) {
        *self = match raw{
            FileWidgetRawData::Empty => Self::Empty,
            FileWidgetRawData::AboutToLoad { path } => Self::AboutToLoad { path: Arc::from(path.as_ref()) }
        }
    }
}

impl<T, E> FileWidget<Result<T, E>>
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

    pub fn ok_mut(&mut self) -> Result<&mut T, GuiError>{
        match self{
            Self::Empty => Err(GuiError::new("No file selected".to_owned())),
            Self::AboutToLoad{ .. } | Self::Loading { .. } => Err(GuiError::new("File not loaded yet".to_owned())),
            Self::Finished { value, .. } => value.as_mut().map_err(|err| GuiError::new(format!("{}", err)))
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
        match &self {
            Self::Empty => None,
            Self::AboutToLoad { path } => Some(path),
            Self::Loading { path, .. } => Some(path),
            Self::Finished { path, .. } => Some(path),
        }
    }
}

impl<T: ParsedFile> FileWidget<T>{
    pub fn set_path(&mut self, path: PathBuf){
        *self = Self::AboutToLoad { path: Arc::from(path.as_ref()) }; //FIXME: don't use pathbuf?
    }
}

impl<PF: ParsedFile + Send + 'static> FileWidget<PF> {
    pub fn update(&mut self, ctx: &egui::Context){
        *self = match std::mem::replace(self, Self::Empty){
            Self::AboutToLoad { path } => {
                let ctx = ctx.clone();
                Self::Loading {
                    path: path.clone(),
                    promise: poll_promise::Promise::spawn_thread(
                        "loading file",
                        move || PF::parse(path.as_ref().to_owned(), ctx) //FIXME: maybe don't use to_owned?
                    )
                }
            },
            Self::Loading { path, promise } => match promise.try_take() {
                Err(promise) => Self::Loading { path, promise },
                Ok(parsed) => Self::Finished{
                    path,
                    value: parsed
                },
            },
            Self::Finished{ path, value } => Self::Finished{path, value},
            Self::Empty => Self::Empty,
        };
    }
}

impl<PF: ParsedFile> StatefulWidget for FileWidget<PF> {
    type Value<'p> = &'p Self;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.update(ui.ctx()); //remove this once calling update becomes the standard
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                if ui.button("Open...").clicked(){
                    if let Some(path) = rfd::FileDialog::new().pick_file(){
                        self.set_path(path);
                        return;
                    }
                }
                match &self{
                    Self::AboutToLoad { path } | Self::Loading { path, .. } => {
                        ui.ctx().request_repaint();
                        ui.label(format!("Loading {} ...", path.to_string_lossy()));
                    },
                    Self::Finished{ path, value } => {
                        ui.weak(path.to_string_lossy());
                        value.render(ui, id.with("parsed"));
                    },
                    Self::Empty => {
                        show_error(ui, "No file selected");
                    },
                };
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self
    }
}


impl ParsedFile for Result<PathBuf>{
    fn parse(path: PathBuf, _ctx: egui::Context) -> Self{
        if path.exists(){
            Ok(path)
        }else{
            Err(GuiError::new("File does not exist"))
        }
    }
    fn render(&self, _ui: &mut egui::Ui, _id: egui::Id){

    }
}
