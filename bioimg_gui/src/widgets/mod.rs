pub mod author_widget;
pub mod axis_size_widget;
pub mod cite_widget;
pub mod code_editor_widget;
pub mod cover_image_widget;
pub mod enum_widget;
pub mod error_display;
pub mod file_widget;
pub mod functional;
pub mod gui_npy_array;
pub mod icon_widget;
pub mod inout_tensor_widget;
pub mod maintainer_widget;
pub mod model_interface_widget;
pub mod staging_from_vec;
pub mod staging_num;
pub mod staging_opt;
pub mod staging_string;
pub mod staging_vec;
pub mod axis_widget;
pub mod url_widget;
pub mod util;
pub mod weights_widget;
pub mod attachments_widget;
pub mod tags_widget;
pub mod channel_name_widget;
pub mod notice_widget;
pub mod image_widget;
pub mod output_axis_widget;
pub mod input_axis_widget;

pub trait StatefulWidget {
    type Value<'p>
    where
        Self: 'p;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id);
    fn state<'p>(&'p self) -> Self::Value<'p>;
}

pub trait ValueWidget{
    type Value<'v>;
    fn set_value<'v>(&mut self, value: Self::Value<'v>);
}