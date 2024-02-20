use crate::result::{GuiError, Result};
use crate::widgets::error_display::show_error;
use bioimg_spec::rdf;
use bioimg_spec::rdf::model as modelrdf;
use bioimg_spec::rdf::non_empty_list::NonEmptyList;
use bioimg_spec::runtime as specrt;

use super::file_widget::{FileWidget, FileWidgetState};
use super::gui_npy_array::GuiNpyArray;
use super::tensor_axis_widget::InputTensorAxisWidget;
use super::{StagingString, StagingVec, StatefulWidget};

pub struct InputTensorWidget {
    id_widget: StagingString<modelrdf::TensorId>,
    description_widget: StagingString<rdf::BoundedString<0, 128>>,
    axes_widget: StagingVec<InputTensorAxisWidget>,
    test_tensor_widget: FileWidget<Result<GuiNpyArray>>,

    parsed: Result<specrt::InputTensor>,
}

impl StatefulWidget for InputTensorWidget {
    type Value<'p> = Result<specrt::InputTensor>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Id: ");
                self.id_widget.draw_and_parse(ui, id.with("Id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Description: ");
                self.description_widget.draw_and_parse(ui, id.with("Description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Test Tensor: ");
                self.test_tensor_widget.draw_and_parse(ui, id.with("test tensor"));
            });
            ui.horizontal(|ui| {
                ui.strong("Axes: ");
                self.axes_widget.draw_and_parse(ui, id.with("Axes"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let FileWidgetState::Finished {
            value: Ok(test_tensor), ..
        } = self.test_tensor_widget.state()
        else {
            return Err(GuiError::new("Test tensor is missing".into()));
        };
        let axes = self.axes_widget.state().into_iter().collect::<Result<Vec<_>>>()?;
        let Ok(input_axes) = NonEmptyList::<modelrdf::InputAxis>::try_from(axes) else {
            return Err(GuiError::new("Empty input axes".into())); //FIXME
        };
        Ok(specrt::InputTensor::new(
            self.id_widget.state()?,
            self.description_widget.state()?,
            input_axes,
            test_tensor.contents().clone(), //FIXME: this can be expensive if called every frame
        )?)
    }
}
