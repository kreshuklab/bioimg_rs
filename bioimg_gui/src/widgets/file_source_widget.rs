use std::{marker::PhantomData, path::{Path, PathBuf}, sync::Arc};

use bioimg_runtime as rt;

use crate::{result::{GuiError, Result}, widgets::popup_widget::draw_fullscreen_popup};
use super::{file_widget::{FileWidget, FileWidgetState, ParsedFile}, popup_widget::PopupResult, search_and_pick_widget::SearchAndPickWidget, url_widget::StagingUrl, StatefulWidget, ValueWidget};


pub enum FileSourceState{
    PickedNormalFile{path: Arc<Path>},
    PickedEmptyZip{path: Arc<Path>},
    PickingInner{outer: Arc<Path>, inner_options_widget: SearchAndPickWidget<String>}
}

trait ResultOfFileSourceStateExt{
    fn parse_path(path: &Arc<Path>) -> Self;
}

impl ResultOfFileSourceStateExt for Result<FileSourceState>{
    fn parse_path(path: &Arc<Path>) -> Self {
        if !path.exists(){
            return Err(GuiError::new("File does not exist".to_owned()))
        }
        let Some(extension) = path.extension() else {
            return Ok(FileSourceState::PickedNormalFile { path: path.clone() });
        };
        if extension != "zip"{
            return Ok(FileSourceState::PickedNormalFile { path: path.clone() });
        }
        let mut inner_options = || -> Result<Vec<String>> {
            let archive_file = std::fs::File::open(&path)?;
            let archive = zip::ZipArchive::new(archive_file)?;
            Ok(archive.file_names()
                .filter(|fname| !fname.ends_with('/') && !fname.ends_with('\\'))
                .map(|fname| fname.to_owned())
                .collect())
        }()?;
        inner_options.sort();
        let Some(first_file) = inner_options.first() else {
            return Ok(FileSourceState::PickedEmptyZip { path: path.clone() });
        };
        Ok(FileSourceState::PickingInner {
            outer: path.clone(),
            inner_options_widget: SearchAndPickWidget::new(first_file.clone(), inner_options)
        })
    }
}

impl ParsedFile for Result<FileSourceState>{
    fn parse(path: PathBuf, _ctx: egui::Context) -> Self {
        Self::parse_path(&Arc::from(path.as_ref()))
    }

    fn render(&self, _ui: &mut egui::Ui, _id: egui::Id) {

    }
}

#[derive(Default, PartialEq, Eq)]
pub enum FileSourceWidgetMode{
    #[default]
    Path,
    Url,
}

#[derive(Default)]
pub struct FileSourceWidget{
    pub mode: FileSourceWidgetMode,
    pub outer_file_widget: FileWidget<Result<FileSourceState>>,
    pub http_url_widget: StagingUrl,
}

impl ValueWidget for FileSourceWidget{
    type Value<'v> = rt::FileSource;

    fn set_value(&mut self, value: rt::FileSource){
        let (outer_path, inner_path) = match value{
            rt::FileSource::LocalFile { path } => (path, None),
            rt::FileSource::FileInZipArchive { outer_path, inner_path } => (outer_path, Some(inner_path)),
            rt::FileSource::HttpUrl(url) => {
                self.mode = FileSourceWidgetMode::Url;
                self.http_url_widget.set_value(url);
                return;
            },
        };
        self.mode = FileSourceWidgetMode::Path;
        let mut outer_result = Result::<FileSourceState>::parse_path(&outer_path);
        if let Ok(FileSourceState::PickingInner { inner_options_widget, .. }) = &mut outer_result{
            if let Some(inner_path) = inner_path {
                if inner_options_widget.contains::<&str, str>(&inner_path){
                    inner_options_widget.value = inner_path.as_ref().to_owned() //FIXME: set inside a method
                }
            }
        };
        self.outer_file_widget.state = FileWidgetState::Finished { path: outer_path, value: outer_result};
    }
}

impl StatefulWidget for FileSourceWidget{
    type Value<'p> = Result<rt::FileSource>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.radio_value(&mut self.mode, FileSourceWidgetMode::Path, "Path");
                ui.radio_value(&mut self.mode, FileSourceWidgetMode::Url, "Url");
            });
            match self.mode{
                FileSourceWidgetMode::Path => {
                    self.outer_file_widget.draw_and_parse(ui, id.with("outer".as_ptr()));
                    let FileWidgetState::Finished{ value: Ok(file_source_state), .. } = &mut self.outer_file_widget.state else {
                        return;
                    };
                    let FileSourceState::PickingInner { inner_options_widget, .. } = file_source_state else {
                        return;
                    };
                    ui.horizontal(|ui|{
                        ui.strong("Path within zip: ");
                        inner_options_widget.draw_and_parse(ui, id.with("inner".as_ptr()));
                    });
                },
                FileSourceWidgetMode::Url => {
                    self.http_url_widget.draw_and_parse(ui, id.with("url".as_ptr()));
                },
            }
        });
    }

    fn state(&self) -> Result<rt::FileSource>{
        return match self.mode{
            FileSourceWidgetMode::Path => {
                let FileWidgetState::Finished{ value: Ok(file_source_state), .. } = &self.outer_file_widget.state else {
                    return Err(GuiError::new("Not finished".to_owned()));
                };
                match file_source_state{
                    FileSourceState::PickedEmptyZip { .. } => Err(GuiError::new("Empty zip".to_owned())),
                    FileSourceState::PickedNormalFile { path } => Ok(rt::FileSource::LocalFile { path: path.clone() }),
                    FileSourceState::PickingInner { outer, inner_options_widget } => {
                        Ok(rt::FileSource::FileInZipArchive {
                            outer_path: outer.clone(),
                            inner_path: Arc::from(inner_options_widget.value.as_ref()),
                        })
                    }
                }
            }
            FileSourceWidgetMode::Url => Ok(rt::FileSource::HttpUrl(self.http_url_widget.state()?)),
        }
    }
}

pub trait FileSourcePopupConfig{
    const BUTTON_TEXT: &'static str = "Open...";
    const TITLE: &'static str = "Choose a file";
}

pub struct DefaultFileSourcePopupConfig;
impl FileSourcePopupConfig for DefaultFileSourcePopupConfig{}

#[derive(Default)]
pub enum FileSourceWidgetPopupButton<C: FileSourcePopupConfig = DefaultFileSourcePopupConfig>{
    #[default]
    Empty,
    Picking{file_source_widget: FileSourceWidget},
    Ready{file_source: rt::FileSource, marker: PhantomData<C>},
}

impl<C: FileSourcePopupConfig> StatefulWidget for FileSourceWidgetPopupButton<C>{
    type Value<'p> = Result<rt::FileSource> where C: 'p;
    
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.horizontal(|ui|{
            let open_button_clicked = ui.button(C::BUTTON_TEXT).clicked();
            *self = match (std::mem::take(self), open_button_clicked) {
                (Self::Empty, false) => Self::Empty,
                (Self::Ready { file_source, marker }, false) => {
                    ui.weak(&file_source.to_string());
                    Self::Ready { file_source, marker }
                },
                (Self::Picking { mut file_source_widget }, _) => {
                    let file_source_result: PopupResult<rt::FileSource> = draw_fullscreen_popup(ui, id.with("pop".as_ptr()), C::TITLE, |ui, id|{
                        let mut out = PopupResult::Continued;
                        ui.vertical(|ui|{
                            file_source_widget.draw_and_parse(ui, id);
                            let state = file_source_widget.state();
                            ui.add_space(10.0);
                            ui.horizontal(|ui|{
                                match state {
                                    Ok(file_source) => if ui.button("Ok").clicked(){
                                        out = PopupResult::Finished(file_source);
                                    },
                                    Err(_) => {
                                        ui.add_enabled_ui(false, |ui| ui.button("Ok"));
                                    }
                                };
                                if ui.button("Cancel").clicked(){
                                     out = PopupResult::Closed
                                }
                            });
                        });
                        out
                    });
                    match file_source_result{
                        PopupResult::Continued => Self::Picking { file_source_widget },
                        PopupResult::Closed => Self::Empty,
                        PopupResult::Finished(file_source) => Self::Ready { file_source, marker: PhantomData },
                    }
                },
                (Self::Empty, true) => Self::Picking{ file_source_widget: Default::default() },
                (Self::Ready{ file_source, .. }, true) => Self::Picking{
                    file_source_widget: {
                        let mut widget = FileSourceWidget::default();
                        widget.set_value(file_source);
                        widget
                    }
                },
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        match self {
            Self::Ready { file_source, .. } => {
                Ok(file_source.clone())
            },
            _ => Err(GuiError::new("not ready".to_owned()))
        }
    }
}
