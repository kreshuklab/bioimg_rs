use bioimg_spec::rdf::icon::{Icon, IconParsingError};

use super::StatefulWidget;

#[derive(Copy, Clone, PartialEq, Eq)]
enum InputMode {
    Emoji,
    File,
}

pub struct StagingIcon {
    raw: String,
    parsed: Result<Icon, IconParsingError>,
    input_mode: InputMode,
}

impl StatefulWidget for StagingIcon {
    type Value<'p> = &'p Result<Icon, IconParsingError>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.radio_value(&mut self.input_mode, InputMode::Emoji, "emoji");
        ui.add_enabled_ui(self.input_mode == InputMode::Emoji, |ui| {
            ui.label("Icon text (e.g. emoji): ");
            ui.text_edit_singleline(&mut self.raw);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}
