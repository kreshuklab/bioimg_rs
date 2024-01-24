use url::Url;

use super::DrawAndParse;

#[derive(Default)]
pub struct StagingUrl{
    raw: String,
}

impl DrawAndParse for StagingUrl{
    type Value<'p> = Result<Url, url::ParseError>;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) -> Self::Value<'p> {

        ui.add(
            egui::TextEdit::singleline(&mut self.raw).min_size(egui::Vec2{x: 200.0, y: 10.0})
        );
        let raw_ref: &str = &self.raw;
        Url::try_from(raw_ref)
    }
}