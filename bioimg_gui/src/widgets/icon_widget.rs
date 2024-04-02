use bioimg_spec::rdf;
use bioimg_runtime as rt;

use super::{image_widget::ImageWidget, staging_string::StagingString, StatefulWidget};
use crate::result::Result;
use super::error_display::show_error;



#[derive(Default)]
pub struct IconImageWidget{
    pub image_widget: ImageWidget,
}

impl StatefulWidget for IconImageWidget{
    type Value<'p> = Result<rt::IconImage>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                self.image_widget.draw_and_parse(ui, id);
            });
            match self.image_widget.state(){
                Err(err) => show_error(ui, err),
                Ok(img) => {
                    if let Err(err) = rt::IconImage::try_from(img){
                        show_error(ui, err)
                    }
                }
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(rt::IconImage::try_from(self.image_widget.state()?)?)
    }
}

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
    image_icon_widget: IconImageWidget,
    input_mode: InputMode,
}

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
