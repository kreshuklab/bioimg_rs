pub mod age_widget;
pub mod fancy_string_widget;
pub mod person_widget;

use std::fmt::Display;

pub trait ParsingWidget
where
    Self: TryFrom<Self::Raw>,
    Self::Error: Display,
{
    type Raw: Clone;

    fn draw_and_parse(ui: &mut egui::Ui, raw: &mut Self::Raw) -> Result<Self, Self::Error>;
}
