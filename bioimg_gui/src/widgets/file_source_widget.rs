use std::{marker::PhantomData, path::{Path, PathBuf}, sync::Arc};

use bioimg_runtime as rt;

use crate::{
    project_data::{FileSourceWidgetRawData, LocalFileSourceWidgetRawData},
    result::{GuiError, Result},
    widgets::popup_widget::draw_fullscreen_popup,
};
use super::{
    error_display::show_error,
    popup_widget::PopupResult,
    search_and_pick_widget::SearchAndPickWidget,
    url_widget::StagingUrl,
    Restore, StatefulWidget, ValueWidget,
};


#[derive(Default)]
pub enum LocalFileSourceWidget{
    #[default]
    Empty,
    Failed(GuiError),
    AboutToLoad{path: Arc<Path>, inner_path: Option<String>},
    LoadingExternal{path: Arc<Path>, promise: poll_promise::Promise<Box<Self>>},
    PickedNormalFile{path: Arc<Path>},
    PickedEmptyZip{path: Arc<Path>},
    PickingInner{outer: Arc<Path>, inner_options_widget: SearchAndPickWidget<String>}
}

impl Restore for LocalFileSourceWidget{
    type RawData = LocalFileSourceWidgetRawData;
    fn dump(&self) -> Self::RawData {
        match self{
            Self::Empty | Self::Failed(_) => Self::RawData::Empty,
            Self::AboutToLoad { path, inner_path } => {
                Self::RawData::AboutToLoad{path: path.to_string_lossy().into(), inner_path: inner_path.clone()}
            },
            Self::LoadingExternal{path, ..} | Self::PickedNormalFile {path} | Self::PickedEmptyZip {path} => {
                Self::RawData::AboutToLoad{path: path.to_string_lossy().into(), inner_path: None}
            },
            Self::PickingInner { outer, inner_options_widget } => {
                Self::RawData::AboutToLoad{path: outer.to_string_lossy().into(), inner_path: Some(inner_options_widget.value.clone())}
            }
        }
    }
    fn restore(&mut self, raw: Self::RawData) {
        match raw{
            Self::RawData::Empty => {
                *self = Self::Empty;
            },
            Self::RawData::AboutToLoad{path, inner_path} => {
                *self = Self::AboutToLoad { path: Arc::from(PathBuf::from(path).as_path()) , inner_path }
            }
        }
    } 
}

impl LocalFileSourceWidget{
    pub fn boxed(self) -> Box<Self>{
        Box::new(self)
    }
    pub fn update(&mut self){
        *self = match std::mem::replace(self, Self::Empty){
            Self::AboutToLoad { path, inner_path } => {
                println!("Should trigger move to loading_external....");
                Self::LoadingExternal {
                    path: path.clone(),
                    promise: poll_promise::Promise::spawn_thread("loading file", move || {
                        if !path.exists(){
                            return Self::Failed(GuiError::new("File does not exist".to_owned())).boxed()
                        }
                        match path.extension(){
                            None => return Self::PickedNormalFile { path }.boxed(),
                            Some(ext) if ext != "zip" => return Self::PickedNormalFile { path }.boxed(),
                            _ => ()
                        }
                        let inner_options = || -> Result<Vec<String>> {
                            let archive_file = std::fs::File::open(&path)?;
                            let archive = zip::ZipArchive::new(archive_file)?;
                            Ok(archive.file_names()
                                .filter(|fname| !fname.ends_with('/') && !fname.ends_with('\\'))
                                .map(|fname| fname.to_owned())
                                .collect())
                        }();
                        let mut inner_options = match inner_options{
                            Ok(opts) => opts,
                            Err(err) => return Self::Failed(err).boxed(),
                        };
                        inner_options.sort();
                        let selected_inner_path = match inner_options.first(){
                            None => return Self::PickedEmptyZip { path: path.clone() }.boxed(),
                            Some(first) => inner_path.unwrap_or(first.clone())
                        };
                        Self::PickingInner {
                            outer: path.clone(),
                            inner_options_widget: SearchAndPickWidget::new(selected_inner_path, inner_options)
                        }.boxed()
                    })
                }
            },
            Self::LoadingExternal { path, promise } => match promise.try_take() {
                Err(promise) => Self::LoadingExternal{ path, promise },
                Ok(parsed) => *parsed,
            },
            state => state,
        };
    }
}

impl StatefulWidget for LocalFileSourceWidget{
    type Value<'p> = Result<rt::FileSource>;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.update();
        ui.vertical(|ui|{
            let inner_widget = ui.horizontal(|ui|{
                if ui.button("Open...").clicked(){
                    if let Some(path) = rfd::FileDialog::new().pick_file(){
                        *self = Self::AboutToLoad { path: Arc::from(path), inner_path: None}
                    }
                }
                match self{
                    Self::Empty => {
                        // ui.weak("Empty");
                        None
                    },
                    Self::Failed(err) => {
                        show_error(ui, err);
                        None
                    },
                    Self::AboutToLoad{path, ..} | Self::LoadingExternal {path, ..} => {
                        ui.weak(format!("Loading {}", path.to_string_lossy()));
                        None
                    }, //FIXME: user inner
                    Self::PickedNormalFile{path} => {
                        ui.weak(path.to_string_lossy());
                        None
                    },
                    Self::PickedEmptyZip{path} => {
                        show_error(ui, format!("Empty zip file: {}", path.to_string_lossy()));
                        None
                    },
                    Self::PickingInner{outer, inner_options_widget} => {
                        ui.weak(outer.to_string_lossy());
                        Some(inner_options_widget)
                    }
                }
            }).inner;
            if let Some(inner_widget) = inner_widget {
                ui.horizontal(|ui|{
                    ui.strong("Inner Path: ");
                    inner_widget.draw_and_parse(ui, id.with("inner_widget".as_ptr()));
                });
            }
        });
    }
    fn state<'p>(&'p self) -> Self::Value<'p> {
        match self{
            Self::Failed(err) => Err(err.clone()),
            Self::AboutToLoad{..} | Self::LoadingExternal{..} | Self::Empty | Self::PickedEmptyZip{..} => {
                Err(GuiError::new("Empty"))
            },
            Self::PickingInner{outer, inner_options_widget} => {
                Ok(rt::FileSource::FileInZipArchive {
                    outer_path: outer.clone(),
                    inner_path: Arc::from(inner_options_widget.value.as_ref())
                })
            },
            Self::PickedNormalFile{path} => {
                Ok(rt::FileSource::LocalFile{path: path.clone()})
            },
        }
    }
}

#[derive(Default, PartialEq, Eq, strum::VariantArray, Clone, strum::Display, strum::AsRefStr)]
pub enum FileSourceWidgetMode{
    #[default]
    #[strum(to_string = "Local File")]
    Local,
    Url,
}

#[derive(Default)]
pub struct FileSourceWidget{
    pub mode_widget: SearchAndPickWidget<FileSourceWidgetMode, false>,
    pub local_file_source_widget: LocalFileSourceWidget,
    pub http_url_widget: StagingUrl,
}

impl Restore for FileSourceWidget{
    type RawData = FileSourceWidgetRawData;
    fn dump(&self) -> Self::RawData {
        match self.mode_widget.value{
            FileSourceWidgetMode::Local => {
                Self::RawData::Local(self.local_file_source_widget.dump())
            },
            FileSourceWidgetMode::Url => {
                Self::RawData::Url(self.http_url_widget.dump())
            }
        }
    }
    fn restore(&mut self, raw: Self::RawData) {
        match raw{
            Self::RawData::Local(local) => self.local_file_source_widget.restore(local),
            Self::RawData::Url(url) => self.http_url_widget.restore(url)
        }
    }
}

impl ValueWidget for FileSourceWidget{
    type Value<'v> = rt::FileSource;

    fn set_value(&mut self, value: rt::FileSource){
        match value{
            rt::FileSource::LocalFile { path } => {
                self.mode_widget.value = FileSourceWidgetMode::Local;
                self.local_file_source_widget = LocalFileSourceWidget::AboutToLoad { path, inner_path: None};
            },
            rt::FileSource::FileInZipArchive { outer_path, inner_path } => {
                self.mode_widget.value = FileSourceWidgetMode::Local;
                self.local_file_source_widget = LocalFileSourceWidget::AboutToLoad {
                    path: outer_path,
                    inner_path: Some(inner_path.as_ref().to_owned()),
                };
            },
            rt::FileSource::HttpUrl(url) => {
                self.mode_widget.value = FileSourceWidgetMode::Url;
                self.http_url_widget.set_value(url);
            },
        };
    }
}

impl FileSourceWidget{
    pub fn update(&mut self){
        self.http_url_widget.update();
        self.local_file_source_widget.update();
    }
}

impl StatefulWidget for FileSourceWidget{
    type Value<'p> = Result<rt::FileSource>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                self.mode_widget.draw_and_parse(ui, id.with("mode".as_ptr()));
                if matches!(self.mode_widget.value, FileSourceWidgetMode::Url){
                    self.http_url_widget.draw_and_parse(ui, id.with("url".as_ptr()));
                }
            });
            if matches!(self.mode_widget.value, FileSourceWidgetMode::Local) {
                self.local_file_source_widget.draw_and_parse(ui, id.with("local".as_ptr()));
            }
        });
    }

    fn state(&self) -> Result<rt::FileSource>{
        return match self.mode_widget.value{
            FileSourceWidgetMode::Local => self.local_file_source_widget.state(),
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

impl<C: FileSourcePopupConfig> ValueWidget for FileSourceWidgetPopupButton<C>{
    type Value<'v> = rt::FileSource;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        *self = Self::Ready { file_source: value, marker: PhantomData }
    }
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
