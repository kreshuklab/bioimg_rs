pub mod author_widget;
pub mod file_widget;

use std::{fmt::Display, marker::PhantomData, path::PathBuf};

pub trait DrawAndParse{
    type Parsed<'p> where Self: 'p;
    type Error;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id);
    fn parsed<'p>(&'p self) -> Result<Self::Parsed<'p>, Self::Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum FilePickerError{
    #[error("Empty")]
    Empty,
    #[error("User cancelled")]
    UserCancelled,
    #[error("{0}")]
    IoError(#[source] #[from] std::io::Error),
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
                }
                let data = match std::fs::read(&pth){
                    Ok(d) => {
                        self.contents = Ok((pth, d));
                    },
                    Err(err) => {
                        self.contents = Err(err.into());
                    }
                }
            }
        });
    }

    fn parsed<'p>(&'p self) -> Result<&'p [u8], Self::Error> {
        let a = self.contents.as_ref().map(|(_, data)| data.as_slice());
        let b = a.map_err(|e| (*e).clone());
        b
    }
}

// #[derive(Clone, Debug)]
// pub enum InputLines{
//     SingleLine,
//     Multiline
// }

// #[derive(Clone, Debug)]
// pub struct StagingString<T: TryFrom<String>>
// where
// T::Error : Display
// {
//     raw: String,
//     parsed: Result<T, T::Error>,
//     input_lines: InputLines,
//     marker: PhantomData<T>,
// }

// impl<T: TryFrom<String>> StagingString<T> where T::Error : Display{
//     pub fn new(input_lines: InputLines) -> Self{
//         let raw = String::default();
//         Self{
//             parsed: T::try_from(raw.clone()),
//             raw,
//             input_lines,
//             marker: PhantomData,
//         }
//     }
// }

// impl<T> DrawAndParse for StagingString<T>
// where
//     T: TryFrom<String>,
//     T::Error : Display,
// {
//     type Parsed = T;
//     type Error = T::Error;

//     fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id){
//         match self.input_lines{
//             InputLines::SingleLine => {ui.text_edit_singleline(&mut self.raw);},
//             InputLines::Multiline => {ui.text_edit_multiline(&mut self.raw);},
//         }

//         self.parsed = T::try_from(self.raw.clone());
//         if let Err(ref err) = self.parsed {
//             let error_text = format!("{err}");
//             ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
//         };
//     }

//     fn parsed(&self) -> Result<&T, Self::Error> {
//         self.parsed.map(|v| &v)
//     }
// }

#[derive(Clone, Debug, Default)]
pub struct StagingOpt<STG: DrawAndParse>(Option<STG>);


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
        let staging = self.0.as_mut().unwrap(); //FIXME: https://github.com/rust-lang/rust/issues/51545
        staging.draw_and_parse(ui, id);
        if ui.button("Remove").clicked(){
            self.0.take();
        }
    }

    fn parsed<'p>(&'p self) -> Result<Self::Parsed<'p>, Self::Error> {
        let Some(staging) = &self.0 else{
            return Ok(None)
        };
    }
}

// pub struct StagingVec<STAGING>(Vec<STAGING>);
// impl<STAGING: Default> Default for StagingVec<STAGING>{
//     fn default() -> Self {
//         Self(vec![STAGING::default()])
//     }
// }

// impl<STAGING: DrawAndParse> DrawAndParse for StagingVec<STAGING>
// where
// STAGING::Error : Display,
// STAGING: Default + Clone{
//     type Parsed<'p> = Vec<STAGING::Parsed<'p>> where STAGING: 'p;
//     type Error = STAGING::Error;

//     fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error> {
//         let parsed_item_results = ui.vertical(|ui|{
//             let parsed_item_results: Vec<_> = self.0.iter_mut().enumerate().map(|(idx, staging_item)| {
//                 ui.label(format!("#{}", idx + 1));
//                 let res = staging_item.draw_and_parse(ui, id.with(idx));
//                 ui.separator();
//                 res
//             }).collect();
//             ui.horizontal(|ui|{
//                 if ui.button("+").clicked(){
//                     self.0.resize(self.0.len() + 1, STAGING::default());
//                 }
//                 if ui.button("-").clicked() && self.0.len() > 1{
//                     self.0.resize(self.0.len() - 1, STAGING::default());
//                 }
//             });
//             parsed_item_results
//         }).inner;
//         let out: Result<Vec<_>, Self::Error> = parsed_item_results.into_iter().collect();
//         Ok(out?)
//     }
// }
