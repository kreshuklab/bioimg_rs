pub mod author_widget;

use std::{fmt::Display, marker::PhantomData};

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

impl<T: TryFrom<String>> StagingString<T>
where
T::Error : Display
{
    pub fn draw_and_update(&mut self, ui: &mut egui::Ui) -> Result<T, T::Error>{
        ui.text_edit_singleline(&mut self.0);
        let res = T::try_from(self.0.clone());
        if let Err(ref err) = res {
            let error_text = format!("{err}");
            ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
        };
        res
    }
}

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

impl<T: TryFrom<String>> StagingOptString<T>
where
T::Error : Display
{
    pub fn draw_and_update(&mut self, ui: &mut egui::Ui) -> Result<Option<T>, T::Error>{
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