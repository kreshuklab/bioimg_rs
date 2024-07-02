use super::{Restore, StatefulWidget, ValueWidget};

#[derive(Default, Restore)]
pub struct CodeEditorWidget {
    pub raw: String,
}

impl ValueWidget for CodeEditorWidget{
    type Value<'v> = &'v str;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.raw.clear();
        self.raw += value;
    }
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
