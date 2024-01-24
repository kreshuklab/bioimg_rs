pub mod author_widget;
pub mod file_widget;
pub mod cover_image_widget;
pub mod url_widget;
pub mod cite_widget;
pub mod error_display;

use std::{fmt::Display, marker::PhantomData};

pub trait DrawAndParse{
    type Value<'p> where Self: 'p;
    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Self::Value<'p>;
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
    type Value<'p> = Result<T, T::Error> where T: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) -> Result<T, T::Error> {
        match self.input_lines{
            InputLines::SingleLine => {
                ui.add( //FIXME: any way we can not hardcode this? at least use font size?
                    egui::TextEdit::singleline(&mut self.raw).min_size(egui::Vec2{x: 200.0, y: 10.0})
                );
            },
            InputLines::Multiline => {ui.text_edit_multiline(&mut self.raw);},
        }
        T::try_from(self.raw.clone())
    }
}


#[derive(Clone, Debug, Default)]
pub struct StagingOpt<Stg: DrawAndParse>(Option<Stg>);

impl<Stg> DrawAndParse for StagingOpt<Stg> where Stg: Default + DrawAndParse{
    type Value<'p> = Option<Stg::Value<'p>>
    where
        Stg::Value<'p>: 'p,
        Stg: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Option<Stg::Value<'p>> {
        if self.0.is_none(){  // FIXME: https://github.com/rust-lang/rust/issues/51545
            ui.horizontal(|ui|{
                ui.label("None");
                if ui.button("Add").clicked(){
                    self.0.replace(Stg::default());
                };
            });
            return None //FIXME: "state-tearing"?
        }

        ui.horizontal(|ui|{
            if ui.button("🗙").clicked(){
                self.0.take();
                return None
            }
            //FIXME: like above, unwrap because https://github.com/rust-lang/rust/issues/51545
            Some(self.0.as_mut().unwrap().draw_and_parse(ui, id))
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
    type Value<'p> = Vec<Stg::Value<'p>>
    where
        Stg: 'p,
        Stg::Value<'p>: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Vec<Stg::Value<'p>> {
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
            self.staging.iter_mut()
                .enumerate()
                .map(|(idx, staging_item)| {
                    ui.label(format!("#{}", idx + 1));
                    let res = staging_item.draw_and_parse(ui, id.with(idx));
                    // ui.separator();
                    res
                })
                .collect()
        }).inner
    }
}
