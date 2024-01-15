pub mod author_widget;
pub mod file_widget;

use std::{fmt::Display, path::PathBuf};

pub trait DrawAndParse{
    type Parsed<'p> where Self: 'p;
    type Error<'p> where Self: 'p;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id);
    fn result<'p>(&'p self) -> Result<Self::Parsed<'p>, Self::Error<'p>>;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum FilePickerError{
    #[error("Empty")]
    Empty,
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
    // pub fn data(&self) -> Option<&[u8]>{
    //     self.contents.as_ref().ok().map(|(_, data)| data.as_slice())
    // }
}

impl DrawAndParse for ImageWidget{
    type Parsed<'p> = &'p (PathBuf, Vec<u8>);
    type Error<'p> = FilePickerError;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id){
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

    fn result(&self) -> Result<&(PathBuf, Vec<u8>), FilePickerError> {
        self.contents.as_ref().map_err(|err| err.clone())
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
    T::Error : Display,
{
    type Parsed<'p> = &'p T where T: 'p;
    type Error<'p> = &'p T::Error where T::Error : 'p, T: 'p;

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

    fn result<'p>(&'p self) -> Result<Self::Parsed<'p>, Self::Error<'p>> {
        self.parsed.as_ref()
    }
}


#[derive(Clone, Debug, Default)]
pub struct StagingOpt<Stg: DrawAndParse>(Option<Stg>);

impl<Stg: DrawAndParse> StagingOpt<Stg>{
    pub fn new() -> Self{
        Self(None)
    }
}

impl<Stg> DrawAndParse for StagingOpt<Stg> where Stg: Default + DrawAndParse{
    type Parsed<'p> = Option<Stg::Parsed<'p>>
    where
        Stg::Parsed<'p>: 'p,
        Stg: 'p;

    type Error<'p> = Stg::Error<'p>
    where
        Stg::Error<'p>: 'p,
        Stg: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        let contents = &mut self.0;
        ui.horizontal(|ui|{
            let Some(staging) = contents else{
                ui.label("None");
                if ui.button("Add").clicked(){
                    contents.replace(Stg::default());
                };
                return
            };
            let remove_clicked = ui.button("🗙").clicked();
            staging.draw_and_parse(ui, id);
            if remove_clicked{
                contents.take();
            }
        });
    }

    fn result<'p>(&'p self) -> Result<Self::Parsed<'p>, Self::Error<'p>> {
        let Some(ref staging) = self.0 else{
            return Ok(None)
        };
        staging.result().map(|v| Some(v))
    }
}

pub struct StagingVec<Stg> where Stg: DrawAndParse{
    staging: Vec<Stg>,
}

impl<Stg: DrawAndParse + Default> Default for StagingVec<Stg>{
    fn default() -> Self {
        Self{staging: vec![Stg::default()]}
    }
}

impl<Stg: DrawAndParse> DrawAndParse for StagingVec<Stg>
where
Stg: Default{
    type Parsed<'p> = Vec<Stg::Parsed<'p>>
    where
        Stg: 'p,
        Stg::Parsed<'p>: 'p;

    type Error<'p> = Stg::Error<'p>
    where
        Stg: 'p,
        Stg::Parsed<'p>: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.vertical(|ui|{
            self.staging.iter_mut().enumerate().for_each(|(idx, staging_item)| {
                ui.label(format!("#{}", idx + 1));
                staging_item.draw_and_parse(ui, id.with(idx));
                ui.separator();
            });
            ui.horizontal(|ui|{
                if ui.button("+").clicked(){
                    self.staging.resize_with(self.staging.len() + 1, Stg::default);
                }
                if ui.button("-").clicked() && self.staging.len() > 1{
                    self.staging.resize_with(self.staging.len() - 1, Stg::default);
                }
            });
        });
    }

    fn result<'p>(&'p self) -> Result<Self::Parsed<'p>, Self::Error<'p>> {
        self.staging.iter().map(|stg| stg.result()).collect()
    }
}
