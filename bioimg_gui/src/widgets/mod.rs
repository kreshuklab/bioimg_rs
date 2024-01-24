pub mod author_widget;
pub mod file_widget;
pub mod cover_image_widget;
pub mod url_widget;
pub mod cite_widget;
pub mod error_display;

use std::{fmt::Display, marker::PhantomData};

pub trait DrawAndParse{
    type Parsed<'p> where Self: 'p;
    type Error;
    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error>;
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
        T::try_from(self.raw.clone())
    }
}


#[derive(Clone, Debug, Default)]
pub struct StagingOpt<Stg: DrawAndParse>(Option<Stg>);

impl<Stg> DrawAndParse for StagingOpt<Stg> where Stg: Default + DrawAndParse{
    type Parsed<'p> = Option<Stg::Parsed<'p>>
    where
        Stg::Parsed<'p>: 'p,
        Stg: 'p;

    type Error = Stg::Error;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error> {
        if self.0.is_none(){  // FIXME: https://github.com/rust-lang/rust/issues/51545
            ui.horizontal(|ui|{
                ui.label("None");
                if ui.button("Add").clicked(){
                    self.0.replace(Stg::default());
                };
            });
            return Ok(None) //FIXME: "state-tearing"?
        }

        ui.horizontal(|ui|{
            if ui.button("🗙").clicked(){
                self.0.take();
                return Ok(None)
            }
            //FIXME: like above, unwrap because https://github.com/rust-lang/rust/issues/51545
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
            // ui.separator();
            let x: Vec<_> = self.staging.iter_mut()
                .enumerate()
                .map(|(idx, staging_item)| {
                    ui.label(format!("#{}", idx + 1));
                    let res = staging_item.draw_and_parse(ui, id.with(idx));
                    // ui.separator();
                    res
                })
                .collect();
            let res: Result<Vec<_>, _> = x.into_iter().collect();
            res
        }).inner
    }
}
