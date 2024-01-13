pub mod author_widget;
pub mod file_widget;

use std::{fmt::Display, marker::PhantomData, path::PathBuf};

pub trait DrawAndParse{
    type Parsed<'p> where Self: 'p;
    type Error;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id);
    fn parsed<'p>(&'p self) -> Result<Self::Parsed<'p>, Self::Error>;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum FilePickerError{
    #[error("Empty")]
    Empty,
    #[error("User cancelled")]
    UserCancelled,
    #[error("Could not open {path}: {reason}")]
    IoError{path: PathBuf, reason: String},
}

pub struct ImageWidget{
    contents: Result<(PathBuf, Vec<u8>), FilePickerError>,
}
impl ImageWidget{
    pub fn path(&self) -> Option<&PathBuf>{
        self.contents.as_ref().ok().map(|(path, _)| path)
    }
    pub fn data(&self) -> Option<&[u8]>{
        self.contents.as_ref().ok().map(|(_, data)| data.as_slice())
    }
}

impl DrawAndParse for ImageWidget{
    type Parsed<'p> = &'p [u8];
    type Error = FilePickerError;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.horizontal(|ui|{
            match &self.path(){
                None => ui.label("None"),
                Some(path) => ui.label(path.to_string_lossy())
            };

            if ui.button("Open...").clicked(){
                let path_buf = rfd::FileDialog::new()
                    .set_directory("/")
                    .pick_file();
                if path_buf.as_ref() == self.path(){
                    return
                }
                self.contents = Err(FilePickerError::Empty);
                let Some(pth) = path_buf else{
                    return
                };
                match std::fs::read(&pth){
                    Ok(d) => {
                        self.contents = Ok((pth, d));
                    },
                    Err(err) => {
                        self.contents = Err(FilePickerError::IoError { path: pth, reason: err.to_string() });
                    }
                };
            }
        });
    }

    fn parsed<'p>(&'p self) -> Result<&'p [u8], Self::Error> {
        let a = self.contents.as_ref().map(|(_, data)| data.as_slice());
        let b = a.map_err(|e| (*e).clone());
        b
    }
}

#[derive(Clone, Debug)]
pub enum InputLines{
    SingleLine,
    Multiline
}

#[derive(Clone, Debug)]
pub struct StagingString<T: TryFrom<String>>
where
T::Error : Display
{
    raw: String,
    parsed: Result<T, T::Error>,
    input_lines: InputLines,
}

impl<T: TryFrom<String>> Default for StagingString<T>
where
    T::Error : Display
{
    fn default() -> Self {
        let raw = String::default();
        Self {
            raw: raw.clone(), parsed: T::try_from(raw), input_lines: InputLines::SingleLine, //FIXME: input lines
        }
    }
}

impl<T: TryFrom<String>> StagingString<T> where T::Error : Display{
    pub fn new(input_lines: InputLines) -> Self{
        let raw = String::default();
        Self{
            parsed: T::try_from(raw.clone()),
            raw,
            input_lines,
        }
    }
}

impl<T> DrawAndParse for StagingString<T>
where
    T: TryFrom<String>,
    T::Error : Display + Clone,
{
    type Parsed<'p> = &'p T where T: 'p;
    type Error = T::Error;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id){
        match self.input_lines{
            InputLines::SingleLine => {ui.text_edit_singleline(&mut self.raw);},
            InputLines::Multiline => {ui.text_edit_multiline(&mut self.raw);},
        }

        self.parsed = T::try_from(self.raw.clone());
        if let Err(ref err) = self.parsed {
            let error_text = format!("{err}");
            ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
        };
    }

    fn parsed(&self) -> Result<&T, Self::Error> {
        match &self.parsed{
            Ok(v) => Ok(v),
            Err(err) => Err(err.clone()) //FIXME?
        }
    }
}

#[derive(Clone, Debug)]
pub struct StagingOpt<STG: DrawAndParse>(Option<STG>);

impl<STG: DrawAndParse> StagingOpt<STG>{
    pub fn new() -> Self{
        Self(None)
    }
}


impl<STG> DrawAndParse for StagingOpt<STG> where STG: Default + DrawAndParse{
    type Parsed<'p> = Option<STG::Parsed<'p>> where STG: 'p;
    type Error = STG::Error;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id){
        if self.0.is_none(){
            ui.horizontal(|ui|{
                ui.label("None");
                if ui.button("Add").clicked(){
                    self.0.replace(STG::default());
                }
            });
        }
        let staging = self.0.as_mut().unwrap(); //FIXME: no ref return, so we can match
        staging.draw_and_parse(ui, id);
        if ui.button("Remove").clicked(){
            self.0.take();
        }
    }

    fn parsed<'p>(&'p self) -> Result<Self::Parsed<'p>, Self::Error> {
        let Some(staging) = &self.0 else{
            return Ok(None)
        };
        staging.parsed().map(|v| Some(v))
    }
}

pub struct StagingVec<STG> where STG: DrawAndParse{
    staging: Vec<STG>,
    // parsed: Result<STG::Parsed, STG::Error>,
}

impl<STG: DrawAndParse + Default> Default for StagingVec<STG>{
    fn default() -> Self {
        Self{staging: vec![STG::default()]}
    }
}

impl<STAGING: DrawAndParse> DrawAndParse for StagingVec<STAGING>
where
    STAGING::Error : Display,
    STAGING: Default + Clone
{
    type Parsed<'p> = Vec<STAGING::Parsed<'p>> where STAGING: 'p;
    type Error = STAGING::Error;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.vertical(|ui|{
            self.staging.iter_mut().enumerate().for_each(|(idx, staging_item)| {
                ui.label(format!("#{}", idx + 1));
                let res = staging_item.draw_and_parse(ui, id.with(idx));
                ui.separator();
                res
            });
            ui.horizontal(|ui|{
                if ui.button("+").clicked(){
                    self.staging.resize(self.staging.len() + 1, STAGING::default());
                }
                if ui.button("-").clicked() && self.staging.len() > 1{
                    self.staging.resize(self.staging.len() - 1, STAGING::default());
                }
            });
        });
    }

    fn parsed<'p>(&'p self) -> Result<Self::Parsed<'p>, Self::Error> {
        let v: Result<Vec<_>, _> = self.staging.iter().map(|stg| stg.parsed()).collect();
        v
    }
}
