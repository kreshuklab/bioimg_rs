pub mod author_widget;
pub mod file_widget;

use std::{fmt::Display, marker::PhantomData};

pub trait DrawAndParse{
    type Parsed;
    type Error;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed, Self::Error>;
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
    input_lines: InputLines,
    marker: PhantomData<T>,
}

impl<T: TryFrom<String>> StagingString<T>
where
T::Error : Display{
    pub fn multiline() -> Self{
        Self{raw: String::default(), input_lines: InputLines::Multiline, marker: PhantomData}
    }
}


impl<T: TryFrom<String>> Default for StagingString<T>
where
T::Error : Display{
    fn default() -> Self {
        Self{raw: String::default(), input_lines: InputLines::SingleLine, marker: PhantomData}
    }
}

impl<T: TryFrom<String>> DrawAndParse for StagingString<T>
where
T::Error : Display
{
    type Parsed = T;
    type Error = T::Error;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) -> Result<T, T::Error>{
        match self.input_lines{
            InputLines::SingleLine => {ui.text_edit_singleline(&mut self.raw);},
            InputLines::Multiline => {ui.text_edit_multiline(&mut self.raw);},
        }

        let res = T::try_from(self.raw.clone());
        if let Err(ref err) = res {
            let error_text = format!("{err}");
            ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
        };
        res
    }
}

#[derive(Clone, Debug, Default)]
pub struct StagingOpt<STAGING: DrawAndParse>(Option<STAGING>);


impl<STAGING: DrawAndParse> DrawAndParse for StagingOpt<STAGING>
where
STAGING: Default
{
    type Parsed = Option<STAGING::Parsed>;
    type Error = STAGING::Error;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed, Self::Error>{
        if let Some(staging) = &mut self.0{
            let (parsed_result, button_response) = ui.horizontal_top(move |ui|{
                let button_response = ui.button("None");
                let parsed_result  = staging.draw_and_parse(ui, id);
                (parsed_result, button_response)
            }).inner;
            if button_response.clicked(){
                self.0.take();
            }
            Ok(Some(parsed_result?))
        }else{
            ui.horizontal(|ui|{
                ui.label("None");
                if ui.button("Add +").clicked(){
                    self.0.replace(STAGING::default());
                }
            });
            Ok(None)
        }
    }
}

pub struct StagingVec<STAGING>(Vec<STAGING>);
impl<STAGING: Default> Default for StagingVec<STAGING>{
    fn default() -> Self {
        Self(vec![STAGING::default()])
    }
}

impl<STAGING: DrawAndParse> DrawAndParse for StagingVec<STAGING>
where
STAGING::Error : Display,
STAGING: Default + Clone{
    type Error = STAGING::Error;
    type Parsed = Vec<STAGING::Parsed>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed, Self::Error> {
        let parsed_item_results = ui.vertical(|ui|{
            let parsed_item_results: Vec<_> = self.0.iter_mut().enumerate().map(|(idx, staging_item)| {
                ui.label(format!("#{}", idx + 1));
                let res = staging_item.draw_and_parse(ui, id.with(idx));
                ui.separator();
                res
            }).collect();
            ui.horizontal(|ui|{
                if ui.button("+").clicked(){
                    self.0.resize(self.0.len() + 1, STAGING::default());
                }
                if ui.button("-").clicked() && self.0.len() > 1{
                    self.0.resize(self.0.len() - 1, STAGING::default());
                }
            });
            parsed_item_results
        }).inner;
        let out: Result<Vec<_>, Self::Error> = parsed_item_results.into_iter().collect();
        Ok(out?)
    }
}