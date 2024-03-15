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
pub mod rdf_attachment;
pub mod staging_from_vec;
pub mod staging_num;
pub mod staging_opt;
pub mod staging_string;
pub mod staging_vec;
pub mod tensor_axis_widget;
pub mod url_widget;
pub mod util;
pub mod weights_widget;

use bioimg_runtime as rt;

use self::file_widget::FileWidget;
use crate::result::Result;

pub trait StatefulWidget {
    type Value<'p>
    where
        Self: 'p;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id);
    fn state<'p>(&'p self) -> Self::Value<'p>;
}

pub type LocalFileRefWidget = FileWidget<Result<rt::LocalRdfFileRef>>;