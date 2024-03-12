use bioimg_spec::rdf::HttpUrl;

use super::{error_display::show_if_error, StatefulWidget};
use crate::result::{GuiError, Result};

pub struct StagingUrl {
    raw: String,
    parsed: Result<HttpUrl>,
}

impl Default for StagingUrl {
    fn default() -> Self {
        let raw = String::default();
        Self {
            raw: raw.clone(),
            parsed: HttpUrl::try_from(raw.clone()).map_err(|err| GuiError::new(err.to_string())),
        }
    }
}

impl StatefulWidget for StagingUrl {
    type Value<'p> = Result<HttpUrl>;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.add(egui::TextEdit::singleline(&mut self.raw).min_size(egui::Vec2 { x: 200.0, y: 10.0 }));
        self.parsed = HttpUrl::try_from(self.raw.clone()).map_err(|err| GuiError::new(err.to_string()));
        show_if_error(ui, &self.parsed);
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}
