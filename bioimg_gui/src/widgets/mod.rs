pub mod author_widget;

use std::{fmt::Display, marker::PhantomData};

pub trait DrawAndParse{
    type Parsed;
    type Error;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui) -> Result<Self::Parsed, Self::Error>;
}

#[derive(Clone, Debug)]
pub struct StagingString<T: TryFrom<String>>(String, PhantomData<T>)
where
T::Error : Display;

impl<T: TryFrom<String>> Default for StagingString<T>
where
T::Error : Display{
    fn default() -> Self {
        Self(String::default(), PhantomData)
    }
}

impl<T: TryFrom<String>> DrawAndParse for StagingString<T>
where
T::Error : Display
{
    type Parsed = T;
    type Error = T::Error;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui) -> Result<T, T::Error>{
        ui.text_edit_singleline(&mut self.0);
        let res = T::try_from(self.0.clone());
        if let Err(ref err) = res {
            let error_text = format!("{err}");
            ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
        };
        res
    }
}

#[derive(Clone, Debug)]
pub struct StagingOptString<T: TryFrom<String>>(String, PhantomData<T>)
where
T::Error : Display;

impl<T: TryFrom<String>> Default for StagingOptString<T>
where
T::Error : Display{
    fn default() -> Self {
        Self(String::default(), PhantomData)
    }
}

impl<T: TryFrom<String>> DrawAndParse for StagingOptString<T>
where
T::Error : Display
{
    type Parsed = Option<T>;
    type Error = T::Error;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui) -> Result<Option<T>, T::Error>{
        ui.text_edit_singleline(&mut self.0);
        if self.0.len() == 0{
            return Ok(None)
        }
        let res = T::try_from(self.0.clone());
        if let Err(ref err) = res {
            let error_text = format!("{err}");
            ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
        };
        res.map(|ok| Some(ok))
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

    fn draw_and_parse(&mut self, ui: &mut egui::Ui) -> Result<Self::Parsed, Self::Error> {
        let parsed_item_results = ui.vertical(|ui|{
            let parsed_item_results: Vec<_> = self.0.iter_mut().enumerate().map(|(idx, staging_item)| {
                ui.label(format!("#{}", idx + 1));
                let res = staging_item.draw_and_parse(ui);
                ui.separator();
                res
            }).collect();
            ui.separator();
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