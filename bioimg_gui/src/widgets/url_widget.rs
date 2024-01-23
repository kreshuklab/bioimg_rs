use url::Url;

use super::DrawAndParse;

#[derive(Default)]
pub struct StagingUrl{
    raw: String,
}

impl DrawAndParse for StagingUrl{
    type Parsed<'p> = Url;
    type Error = url::ParseError;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error> {
        ui.text_edit_singleline(&mut self.raw);
        let raw_ref: &str = &self.raw;
        Url::try_from(raw_ref)
    }
}