use bioimg_spec::rdf;
use bioimg_runtime as rt;

use super::{image_widget::ImageWidget, staging_string::StagingString, StatefulWidget};
use crate::result::Result;



#[derive(Copy, Clone, PartialEq, Eq)]
enum InputMode {
    Emoji,
    File,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Emoji
    }
}

#[derive(Default)]
pub struct IconWidget {
    emoji_icon_widget: StagingString<rdf::EmojiIcon>,
    image_icon_widget: ImageWidget<rt::IconImage>,
    input_mode: InputMode,
}

// impl ValueWidget for IconWidget{
//     type Value<'v> = rt::Icon;

//     fn set_value<'v>(&mut self, value: Self::Value<'v>) {
//         match value{
//             rt::Icon::Image(img_icon) => {
//                 self.input_mode = InputMode::File;
//                 self.image_icon_widget.set_value(img_icon);
//             },
//             rt::Icon::Text(icon_text) => {
//                 self.input_mode = InputMode::Emoji;
//                 self.emoji_icon_widget.set_value(icon_text);
//             }
//         }
//     }
// }

impl StatefulWidget for IconWidget {
    type Value<'p> = Result<rt::Icon>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.input_mode, InputMode::Emoji, "Emoji Icon");
                ui.radio_value(&mut self.input_mode, InputMode::File, "Image File Icon");
            });
            if self.input_mode == InputMode::Emoji {
                self.emoji_icon_widget.draw_and_parse(ui, id.with("Emoji Icon"));
            }
            if self.input_mode == InputMode::File {
                self.image_icon_widget.draw_and_parse(ui, id.with("Image File Icon"));
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        match &self.input_mode{
            InputMode::Emoji => Ok(rt::Icon::Text(self.emoji_icon_widget.state()?)),
            InputMode::File => {
                Ok(rt::Icon::Image(
                    self.image_icon_widget.state()?
                ))
            }
        }
    }
}
