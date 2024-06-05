use std::{error::Error, fmt::Display, str::FromStr};
use std::fmt::Write;

use crate::result::{GuiError, Result};
use super::{error_display::show_if_error, StatefulWidget, ValueWidget};

pub struct StagingFloat<T>{
    pub raw: String,
    pub parsed: Result<T>,
}

impl<T> Default for StagingFloat<T>{
    fn default() -> Self {
        Self{
            raw: String::new(),
            parsed: Err(GuiError::new("empty".to_owned()))
        }
    }
}

impl<T> StatefulWidget for StagingFloat<T>
where
    T: FromStr + Copy,
    <T as FromStr>::Err: Error,
{
    type Value<'p> = Result<T> where T: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.horizontal(|ui|{
            ui.add(
                egui::TextEdit::singleline(&mut self.raw).min_size(egui::Vec2 { x: 200.0, y: 10.0 }),
            );
            self.parsed = T::from_str(&self.raw).map_err(|err| GuiError::from(err));
            show_if_error(ui, &self.parsed);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}

impl<T: Display> ValueWidget for StagingFloat<T>{
    type Value<'v> = T;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.raw.clear();
        write!(self.raw, "{}", value).unwrap();
        self.parsed = Ok(value)
    }
}

impl<T> StagingFloat<T>
where
    T: FromStr,
    <T as FromStr>::Err: Error
{
    pub fn new_with_raw(value: f32) -> Self{
        let raw = value.to_string();
        let parsed = T::from_str(&raw).map_err(|err| GuiError::from(err));
        Self{raw, parsed}
    }
}
