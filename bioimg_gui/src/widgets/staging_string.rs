use std::{error::Error, fmt::Display};

use crate::result::{GuiError, Result};

use super::{error_display::show_if_error, StatefulWidget};

#[derive(Clone, Debug)]
pub enum InputLines {
    SingleLine,
    Multiline,
}

#[derive(Debug)]
pub struct StagingString<T> {
    pub raw: String,
    pub parsed: Result<T>,
    pub input_lines: InputLines,
}

impl<T: Into<String> + Clone> StagingString<T>{
    pub fn set_value(&mut self, value: T){
        self.raw = value.clone().into();
        self.parsed = Ok(value)
    }
}

impl<T> StagingString<T>
where
    T: TryFrom<String>,
    <T as TryFrom<String>>::Error: Error,
{
    pub fn new_with_raw(raw: impl Into<String>) -> Self{
        let raw = raw.into();
        Self {
            raw: raw.clone(),
            parsed: T::try_from(raw).map_err(|err| GuiError::new(err.to_string())),
            input_lines: InputLines::SingleLine,
        }
    }
}

impl<T> Default for StagingString<T>
where
    T: TryFrom<String>,
    T::Error: Display,
{
    fn default() -> Self {
        let raw = String::default();
        Self {
            raw: raw.clone(),
            parsed: T::try_from(raw).map_err(|err| GuiError::new(err.to_string())),
            input_lines: InputLines::SingleLine,
        }
    }
}

impl<T> StagingString<T>
where
    T: TryFrom<String>,
    T::Error: Display,
{
    pub fn new(input_lines: InputLines) -> Self {
        let raw = String::default();
        Self {
            raw: raw.clone(),
            parsed: T::try_from(raw).map_err(|err| GuiError::new(err.to_string())),
            input_lines,
        }
    }
}

impl<T> StatefulWidget for StagingString<T>
where
    T: TryFrom<String> + Clone,
    T::Error: Display,
{
    type Value<'p> = Result<T> where T: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.horizontal(|ui| {
            match self.input_lines {
                InputLines::SingleLine => {
                    ui.add(
                        //FIXME: any way we can not hardcode this? at least use font size?
                        egui::TextEdit::singleline(&mut self.raw).min_size(egui::Vec2 { x: 200.0, y: 10.0 }),
                    );
                }
                InputLines::Multiline => {
                    ui.text_edit_multiline(&mut self.raw);
                }
            }
            self.parsed = T::try_from(self.raw.clone()).map_err(|err| GuiError::new(err.to_string()));
            show_if_error(ui, &self.parsed);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}
