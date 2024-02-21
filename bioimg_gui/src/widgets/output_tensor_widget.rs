use std::sync::Arc;

use crate::result::{GuiError, Result};
use bioimg_spec::rdf;
use bioimg_spec::rdf::model as modelrdf;
use bioimg_spec::runtime::NpyArray;

use super::file_widget::{FileWidget, FileWidgetState};
use super::gui_npy_array::GuiNpyArray;
use super::tensor_axis_widget::OutputTensorAxisWidget;
use super::{StagingString, StagingVec, StatefulWidget};

pub struct OutputTensorWidget {
    pub id_widget: StagingString<modelrdf::TensorId>,
    pub description_widget: StagingString<rdf::BoundedString<0, 128>>,
    pub axes_widget: StagingVec<OutputTensorAxisWidget>,
    pub test_tensor_widget: FileWidget<Result<GuiNpyArray>>,
}

impl Default for OutputTensorWidget {
    fn default() -> Self {
        Self {
            id_widget: Default::default(),
            description_widget: Default::default(),
            axes_widget: StagingVec::new("Axis"),
            test_tensor_widget: Default::default(),
        }
    }
}

impl StatefulWidget for OutputTensorWidget {
    type Value<'p> = Result<(modelrdf::OutputTensorDescr, Arc<NpyArray>)>;

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
            path,
            value: Ok(gui_npy_array),
            ..
        } = self.test_tensor_widget.state()
        else {
            return Err(GuiError::new("Test tensor is missing".into()));
        };
        let axes = self.axes_widget.state().into_iter().collect::<Result<Vec<_>>>()?;
        let output_axis_group = modelrdf::OutputAxisGroup::try_from(axes)?; //FIXME: parse in draw_and_parse?
        Ok((
            modelrdf::OutputTensorDescr {
                id: self.id_widget.state()?,
                description: self.description_widget.state()?,
                axes: output_axis_group,
                test_tensor: rdf::FileReference::Path(path.clone()),
                sample_tensor: None, //FIXME
            },
            Arc::clone(gui_npy_array.contents()),
        ))
    }
}
