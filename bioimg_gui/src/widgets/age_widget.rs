use super::ParsingWidget;

#[derive(thiserror::Error, Debug)]
pub enum AgeParsingError {
    #[error("Too old")]
    TooOld,
}

#[derive(Debug)]
pub struct Age(u8);
impl TryFrom<u8> for Age {
    type Error = AgeParsingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 120 {
            return Err(AgeParsingError::TooOld);
        }
        return Ok(Self(value));
    }
}

impl ParsingWidget<u8> for Age{
    fn draw_and_parse(ui: &mut egui::Ui, raw: &mut u8) -> Result<Self, Self::Error> {
        ui.add(egui::DragValue::new(raw).speed(1.0));
        return Age::try_from(*raw)
    }
}