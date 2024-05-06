use crate::result::Result;
use super::{code_editor_widget::CodeEditorWidget, error_display::show_if_error, StatefulWidget};

pub struct JsonObjectEditorWidget{
    pub code_editor_widget: CodeEditorWidget,
    pub parsed: Result<serde_json::Map<String, serde_json::Value>>
}

impl Default for JsonObjectEditorWidget{
    fn default() -> Self {
        let default_value = serde_json::Map::new();
        Self{
            code_editor_widget: {
                let mut widget: CodeEditorWidget = Default::default();
                widget.raw = serde_json::to_string(&default_value).unwrap();
                widget
            },
            parsed: Ok(default_value)
        }
    }
}

impl StatefulWidget for JsonObjectEditorWidget{
    type Value<'p> = &'p Result<serde_json::Map<String, serde_json::Value>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            self.code_editor_widget.draw_and_parse(ui, id.with("code".as_ptr()));
            self.parsed = serde_json::from_str(&self.code_editor_widget.raw)
                .map_err(|err| err.into());
            show_if_error(ui, &self.parsed);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}
