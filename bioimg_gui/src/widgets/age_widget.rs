use super::ParsingWidget;

#[derive(thiserror::Error, Debug, Clone)]
pub enum AgeParsingError {
    #[error("Too old")]
    TooOld,
}

#[derive(Debug, Clone)]
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

impl ParsingWidget for Age{
    type Raw = u8;
    fn draw_and_parse(ui: &mut egui::Ui, raw: &mut u8) -> Result<Self, Self::Error> {
        ui.add(egui::DragValue::new(raw).speed(1.0));
        return Age::try_from(*raw)
    }
}

pub struct StagingAge{
    pub raw: u8,
    pub parsed: Result<Age, AgeParsingError>,
}

impl StagingAge{
    pub fn draw_and_update(&mut self, ui: &mut egui::Ui){
        ui.add(egui::DragValue::new(&mut self.raw).speed(1.0));
        self.parsed = Age::try_from(self.raw.clone());
        if let Err(ref err) = self.parsed {
            let error_text = format!("{err}");
            ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
        };
    }
}