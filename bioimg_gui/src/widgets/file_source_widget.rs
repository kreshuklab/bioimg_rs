use std::path::PathBuf;

use bioimg_runtime as rt;

use crate::result::{GuiError, Result};
use super::{file_widget::{FileWidget, FileWidgetState, ParsedFile}, search_and_pick_widget::SearchAndPickWidget, url_widget::StagingUrl, StatefulWidget, ValueWidget};


pub enum FileSourceState{
    PickedNormalFile{path: PathBuf},
    PickedEmptyZip{path: PathBuf},
    PickingInner{outer: PathBuf, inner_options_widget: SearchAndPickWidget<String>}
}

trait ResultOfFileSourceStateExt{
    fn parse_path(path: PathBuf) -> Self;
}

impl ResultOfFileSourceStateExt for Result<FileSourceState>{
    fn parse_path(path: PathBuf) -> Self {
        if !path.exists(){
            return Err(GuiError::new("File does not exist".to_owned()))
        }
        let Some(extension) = path.extension() else {
            return Ok(FileSourceState::PickedNormalFile { path });
        };
        if extension != "zip"{
            return Ok(FileSourceState::PickedNormalFile { path });
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
            return Ok(FileSourceState::PickedEmptyZip { path });
        };
        Ok(FileSourceState::PickingInner {
            outer: path,
            inner_options_widget: SearchAndPickWidget::new(first_file.clone(), inner_options)
        })
    }
}

impl ParsedFile for Result<FileSourceState>{
    fn parse(path: PathBuf, _ctx: egui::Context) -> Self {
        Self::parse_path(path)
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
        let mut outer_result = Result::<FileSourceState>::parse_path(outer_path.clone());
        if let Ok(FileSourceState::PickingInner { inner_options_widget, .. }) = &mut outer_result{
            if let Some(inner_path) = inner_path {
                if inner_options_widget.contains(&inner_path){
                    inner_options_widget.value = inner_path
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
                        Ok(rt::FileSource::FileInZipArchive { outer_path: outer.clone(), inner_path: inner_options_widget.value.clone() })
                    }
                }
            }
            FileSourceWidgetMode::Url => Ok(rt::FileSource::HttpUrl(self.http_url_widget.state()?)),
        }
    }
}
