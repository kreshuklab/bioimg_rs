use std::fmt::Display;

use self::error_display::show_if_error;
use crate::result::{GuiError, Result};

pub mod author_widget;
pub mod axis_size_widget;
pub mod cite_widget;
pub mod code_editor_widget;
pub mod cover_image_widget;
pub mod enum_widget;
pub mod error_display;
pub mod file_widget;
pub mod functional;
pub mod gui_npy_array;
pub mod icon_widget;
pub mod input_tensor_widget;
pub mod maintainer_widget;
pub mod output_tensor_widget;
pub mod staging_opt;
pub mod staging_string;
pub mod staging_vec;
pub mod tensor_axis_widget;
pub mod url_widget;
pub mod util;

pub trait StatefulWidget {
    type Value<'p>
    where
        Self: 'p;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id);
    fn state<'p>(&'p self) -> Self::Value<'p>;
}

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
