use std::path::PathBuf;
use std::sync::Arc;

use bioimg_runtime::model_interface::{InputSlot, OutputSlot};
use bioimg_runtime::npy_array::ArcNpyArray;

use crate::result::{GuiError, Result};
use bioimg_spec::rdf::model as modelrdf;

use super::error_display::show_error;
use super::file_widget::{FileWidget, FileWidgetState};
use super::staging_string::StagingString;
use super::staging_vec::StagingVec;
use super::input_axis_widget::InputAxisWidget;
use super::output_axis_widget::OutputAxisWidget;
use super::{StatefulWidget, ValueWidget};
use crate::widgets::staging_vec::ItemWidgetConf;


#[derive(Default)]
pub struct InputTensorWidget {
    pub id_widget: StagingString<modelrdf::TensorId>,
    pub is_optional: bool,
    pub description_widget: StagingString<modelrdf::TensorTextDescription>,
    pub axes_widget: StagingVec<InputAxisWidget>,
    pub test_tensor_widget: FileWidget<Result<ArcNpyArray>>,
}

impl ValueWidget for InputTensorWidget{
    type Value<'v> = (modelrdf::TensorId, modelrdf::TensorTextDescription, Vec<modelrdf::InputAxis>, ArcNpyArray);
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.id_widget.set_value(value.0);
        self.description_widget.set_value(value.1);
        self.axes_widget.set_value(value.2);
        self.test_tensor_widget.state = FileWidgetState::Finished {
            path: PathBuf::from("__dummy__"),
            value: Ok(value.3)
        }
    }
}

impl ItemWidgetConf for InputTensorWidget{
    const ITEM_NAME: &'static str = "Input Tensor";
}

impl StatefulWidget for InputTensorWidget {
    type Value<'p> = Result<InputSlot<ArcNpyArray>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        if let FileWidgetState::Finished { path, value: Ok(gui_npy_arr) } = self.test_tensor_widget.state() {
            if self.id_widget.raw.is_empty() {
                self.id_widget.raw = path
                    .file_stem()
                    .map(|osstr| String::from(osstr.to_string_lossy()))
                    .unwrap_or(String::default());
            }
            let sample_shape = gui_npy_arr.shape();
            let mut extents = sample_shape.iter().skip(self.axes_widget.staging.len());

            while let Some(extent) = extents.next() {
                let mut axis_widget = InputAxisWidget::default();
                axis_widget.axis_type = if *extent == 1{
                    modelrdf::AxisType::Channel
                } else {
                    modelrdf::AxisType::Space
                };
                axis_widget.space_axis_widget.prefil_parameterized_size(*extent);
                self.axes_widget.staging.push(axis_widget)
            }
        };
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Test Tensor: ");
                self.test_tensor_widget.draw_and_parse(ui, id.with("test tensor"));
                if matches!(self.test_tensor_widget.state(), FileWidgetState::Empty) {
                    show_error(ui, "Missing a npy test tensor");
                }
            });
            ui.horizontal(|ui|{
                ui.strong("Input is optional: ");
                ui.add(egui::widgets::Checkbox::without_text(&mut self.is_optional));
            });
            ui.horizontal(|ui| {
                ui.strong("Tensor Id: ");
                self.id_widget.draw_and_parse(ui, id.with("Id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Description: ");
                self.description_widget.draw_and_parse(ui, id.with("Description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Axes: ");
                self.axes_widget.draw_and_parse(ui, id.with("Axes"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let FileWidgetState::Finished { value: Ok(gui_npy_array), .. } = self.test_tensor_widget.state() else {
            return Err(GuiError::new("Test tensor is missing".into()));
        };
        let axes = self.axes_widget.state().into_iter().collect::<Result<Vec<_>>>()?;
        let input_axis_group = modelrdf::InputAxisGroup::try_from(axes)?; //FIXME: parse in draw_and_parse?
        Ok( InputSlot {
            id: self.id_widget.state()?,
            optional: self.is_optional,
            preprocessing: vec![], //FIXME
            description: self.description_widget.state()?,
            axes: input_axis_group,
            test_tensor: Arc::clone(gui_npy_array),
        })
    }
}

#[derive(Default)]
pub struct OutputTensorWidget {
    pub id_widget: StagingString<modelrdf::TensorId>,
    pub description_widget: StagingString<modelrdf::TensorTextDescription>,
    pub axes_widget: StagingVec<OutputAxisWidget>,
    pub test_tensor_widget: FileWidget<Result<ArcNpyArray>>,
}

impl ValueWidget for OutputTensorWidget{
    type Value<'v> = (modelrdf::TensorId, modelrdf::TensorTextDescription, Vec<modelrdf::OutputAxis>, ArcNpyArray);
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.id_widget.set_value(value.0);
        self.description_widget.set_value(value.1);
        self.axes_widget.set_value(value.2);
        self.test_tensor_widget.state = FileWidgetState::Finished {
            path: PathBuf::from("__dummy__"),
            value: Ok(value.3)
        }
    }
}

impl ItemWidgetConf for OutputTensorWidget{
    const ITEM_NAME: &'static str = "Output Tensor";
}

impl StatefulWidget for OutputTensorWidget {
    type Value<'p> = Result<OutputSlot<ArcNpyArray>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        if let FileWidgetState::Finished { path, value: Ok(gui_npy_arr) } = self.test_tensor_widget.state() {
            if self.id_widget.raw.is_empty() {
                self.id_widget.raw = path
                    .file_stem()
                    .map(|osstr| String::from(osstr.to_string_lossy()))
                    .unwrap_or(String::default());
            }
            let sample_shape = gui_npy_arr.shape();
            let mut extents = sample_shape.iter().skip(self.axes_widget.staging.len());

            while let Some(extent) = extents.next() {
                let mut axis_widget = OutputAxisWidget::default();
                axis_widget.axis_type = if *extent == 1{
                    modelrdf::AxisType::Channel
                } else {
                    modelrdf::AxisType::Space
                };
                axis_widget.space_axis_widget.prefil_parameterized_size(*extent);
                self.axes_widget.staging.push(axis_widget)
            }
        };
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Test Tensor: ");
                self.test_tensor_widget.draw_and_parse(ui, id.with("test tensor"));
                if matches!(self.test_tensor_widget.state(), FileWidgetState::Empty) {
                    show_error(ui, "Missing a npy test tensor");
                }
            });
            ui.horizontal(|ui| {
                ui.strong("Tensor Id: ");
                self.id_widget.draw_and_parse(ui, id.with("Id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Description: ");
                self.description_widget.draw_and_parse(ui, id.with("Description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Axes: ");
                self.axes_widget.draw_and_parse(ui, id.with("Axes"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let FileWidgetState::Finished { value: Ok(gui_npy_array), .. } = self.test_tensor_widget.state() else {
            return Err(GuiError::new("Test tensor is missing".into()));
        };
        let axes = self.axes_widget.state().into_iter().collect::<Result<Vec<_>>>()?;
        let input_axis_group = modelrdf::OutputAxisGroup::try_from(axes)?; //FIXME: parse in draw_and_parse?
        Ok( OutputSlot {
            id: self.id_widget.state()?,
            description: self.description_widget.state()?,
            axes: input_axis_group,
            test_tensor: Arc::clone(gui_npy_array),
        })
    }
}
