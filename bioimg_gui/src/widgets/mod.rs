pub mod age_widget;
pub mod fancy_string_widget;
pub mod person_widget;

use std::fmt::Display;

pub trait ParsingWidget<RAW>
where
    Self: TryFrom<RAW>,
    Self::Error: Display,
    RAW: Clone,
{
    fn draw_and_parse(ui: &mut egui::Ui, raw: &mut RAW) -> Result<Self, Self::Error>;
}
