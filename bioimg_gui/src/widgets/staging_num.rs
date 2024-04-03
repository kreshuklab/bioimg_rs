use std::fmt::Display;

use crate::result::{GuiError, Result};

use super::{error_display::show_if_error, StatefulWidget};

pub struct StagingNum<Raw, Parsed> {
    pub raw: Raw,
    pub parsed: Result<Parsed>,
}

impl<Raw, Parsed> StagingNum<Raw, Parsed>
where
    Raw: Clone,
    Parsed: TryFrom<Raw>,
    Parsed::Error: Display,
{
    pub fn new_with_raw(raw: Raw) -> Self{
        Self {
            raw: raw.clone(),
            parsed: Parsed::try_from(raw).map_err(|err| GuiError::new(err.to_string())),
        }
    }
}

impl<Raw, Parsed> Default for StagingNum<Raw, Parsed>
where
    Raw: Default,
    Parsed: TryFrom<Raw>,
    Parsed::Error: Display,
{
    fn default() -> Self {
        Self {
            raw: Raw::default(),
            parsed: Parsed::try_from(Raw::default()).map_err(|err| GuiError::new(err.to_string())),
        }
    }
}

impl<Raw, Parsed> StatefulWidget for StagingNum<Raw, Parsed>
where
    Raw: egui::emath::Numeric,
    Parsed: TryFrom<Raw> + Clone,
    Parsed::Error: Display + Clone,
{
    type Value<'p> = Result<Parsed> where Parsed: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.add(egui::widgets::DragValue::new(&mut self.raw));
        self.parsed = Parsed::try_from(self.raw.clone()).map_err(|err| GuiError::new(err.to_string()));
        show_if_error(ui, &self.parsed);
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}
