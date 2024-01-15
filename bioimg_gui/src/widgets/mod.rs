pub mod author_widget;
pub mod file_widget;

use std::{fmt::Display, path::PathBuf, marker::PhantomData};

pub trait DrawAndParse{
    type Parsed<'p> where Self: 'p;
    type Error;
    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error>;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum FilePickerError{
    #[error("Empty")]
    Empty,
    #[error("Loading")]
    Loading,
    #[error("Could not open {path}: {reason}")]
    IoError{path: PathBuf, reason: String},
}

pub struct ImageWidget{
    contents: Result<(PathBuf, Vec<u8>), FilePickerError>,
}

impl Default for ImageWidget{
    fn default() -> Self {
        Self{contents: Err(FilePickerError::Empty)}
    }
}

impl ImageWidget{
    pub fn path(&self) -> Option<&PathBuf>{
        self.contents.as_ref().ok().map(|(path, _)| path)
    }
}

impl DrawAndParse for ImageWidget{
    type Parsed<'p> = &'p (PathBuf, Vec<u8>);
    type Error= FilePickerError;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error>{
        ui.horizontal(|ui|{
            match &self.path(){
                None => ui.label("None"),
                Some(path) => ui.label(path.to_string_lossy())
            };

            if ui.button("Open...").clicked(){
                // FIXME: async + web
                let path_buf = rfd::FileDialog::new()
                    .set_directory("/")
                    .pick_file();
                self.contents = Err(FilePickerError::Empty);

                'file_read: {
                    let Some(pth) = path_buf else{
                        break 'file_read;
                    };
                    match std::fs::read(&pth){
                        Ok(d) => {
                            self.contents = Ok((pth, d));
                        },
                        Err(err) => {
                            self.contents = Err(FilePickerError::IoError { path: pth, reason: err.to_string() });
                        }
                    }
                }
            }
            self.contents.as_ref().map_err(|err| err.clone())
        }).inner
    }
}

#[derive(Clone, Debug)]
pub enum InputLines{
    SingleLine,
    Multiline
}

#[derive(Debug)]
pub struct StagingString<T: TryFrom<String>>
where
T::Error : Display
{
    raw: String,
    input_lines: InputLines,
    marker: PhantomData<T>,
}

impl<T: TryFrom<String>> Default for StagingString<T>
where
    T::Error : Display
{
    fn default() -> Self {
        let raw = String::default();
        Self {
            raw: raw.clone(), input_lines: InputLines::SingleLine, marker: PhantomData
        }
    }
}

impl<T: TryFrom<String>> StagingString<T> where T::Error : Display{
    pub fn new(input_lines: InputLines) -> Self{
        let raw = String::default();
        Self{
            raw,
            input_lines,
            marker: PhantomData,
        }
    }
}

impl<T> DrawAndParse for StagingString<T>
where
    T: TryFrom<String>,
    T::Error : Display,
{
    type Parsed<'p> = T where T: 'p;
    type Error = T::Error;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) -> Result<T, T::Error> {
        match self.input_lines{
            InputLines::SingleLine => {ui.text_edit_singleline(&mut self.raw);},
            InputLines::Multiline => {ui.text_edit_multiline(&mut self.raw);},
        }

        let parsed = T::try_from(self.raw.clone());
        if let Err(ref err) = parsed {
            let error_text = format!("{err}");
            ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
        }else{
            ui.label("");
        }
        parsed
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

    type Error = Stg::Error;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error> {
        ui.horizontal(|ui|{
            if self.0.is_none(){  // FIXME: https://github.com/rust-lang/rust/issues/51545
                ui.label("None");
                if ui.button("Add").clicked(){
                    self.0.replace(Stg::default());
                };
                return Ok(None) //FIXME: "state-tearing"?
            }

            if ui.button("🗙").clicked(){
                self.0.take();
                return Ok(None)
            }
            //FIXME: like above, unwrap becausehttps://github.com/rust-lang/rust/issues/51545
            self.0.as_mut().unwrap().draw_and_parse(ui, id).map(|v| Some(v))
        }).inner
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

    type Error = Stg::Error;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error> {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                if ui.button("+").clicked(){
                    self.staging.resize_with(self.staging.len() + 1, Stg::default);
                }
                if ui.button("-").clicked() && self.staging.len() > 1{
                    self.staging.resize_with(self.staging.len() - 1, Stg::default);
                }
            });
            ui.separator();
            let x = self.staging.iter_mut()
                .enumerate()
                .map(|(idx, staging_item)| {
                    ui.label(format!("#{}", idx + 1));
                    let res = staging_item.draw_and_parse(ui, id.with(idx));
                    ui.separator();
                    res
                })
                .collect();
            return x
        }).inner
    }
}
