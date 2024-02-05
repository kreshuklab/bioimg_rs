use super::StatefulWidget;

#[derive(Default)]
pub struct CodeEditorWidget {
    raw: String,
}

impl StatefulWidget for CodeEditorWidget {
    type Value<'p> = &'p str;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.add(
            egui::TextEdit::multiline(&mut self.raw)
                .desired_rows(15)
                .desired_width(f32::INFINITY)
                .code_editor(),
        );
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.raw
    }
}
