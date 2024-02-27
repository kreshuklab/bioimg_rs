use std::fmt::Display;

use crate::result::{GuiError, Result};

use super::{error_display::show_if_error, StatefulWidget};

pub struct StagingNum<N, T> {
    pub raw: N,
    pub parsed: Result<T>,
}

impl<N, T> Default for StagingNum<N, T>
where
    N: Default,
    T: TryFrom<N>,
    T::Error: Display,
{
    fn default() -> Self {
        Self {
            raw: N::default(),
            parsed: T::try_from(N::default()).map_err(|err| GuiError::new(err.to_string())),
        }
    }
}

impl<N, T> StatefulWidget for StagingNum<N, T>
where
    N: egui::emath::Numeric,
    T: TryFrom<N> + Clone,
    T::Error: Display + Clone,
{
    type Value<'p> = Result<T> where T: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.add(egui::widgets::DragValue::new(&mut self.raw));
        self.parsed = T::try_from(self.raw.clone()).map_err(|err| GuiError::new(err.to_string()));
        show_if_error(ui, &self.parsed);
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}
