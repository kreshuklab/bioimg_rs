use super::ParsingWidget;

#[derive(thiserror::Error, Debug)]
pub enum FancyStringParsingError {
    #[error("String is too long to be fancy")]
    TooLong,
}

#[derive(Clone, Debug)]
pub struct FancyString(String);
impl TryFrom<String> for FancyString {
    type Error = FancyStringParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 10 {
            Err(FancyStringParsingError::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}
impl ParsingWidget<String> for FancyString {
    fn draw_and_parse(ui: &mut egui::Ui, raw: &mut String) -> Result<Self, Self::Error> {
        ui.text_edit_singleline(raw);
        let result = Self::try_from(raw.clone());
        if let Err(ref err) = result {
            let error_text = format!("{err}");
            ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
        };
        return result
    }
}
