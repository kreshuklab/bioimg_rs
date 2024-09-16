use super::collapsible_widget::CollapsibleWidget;
use super::{Restore, ValueWidget};
use super::{
    error_display::show_if_error,
    inout_tensor_widget::{InputTensorWidget, OutputTensorWidget},
    staging_vec::StagingVec,
    StatefulWidget,
};
use bioimg_runtime as specrt;
use bioimg_runtime::npy_array::ArcNpyArray;

use crate::result::{GuiError, Result};

#[derive(Restore)]
pub struct ModelInterfaceWidget {
    pub inputs_widget: StagingVec<CollapsibleWidget<InputTensorWidget>>,
    pub outputs_widget: StagingVec<CollapsibleWidget<OutputTensorWidget>>,
    #[restore_on_update]
    pub parsed: Result<specrt::ModelInterface<ArcNpyArray>>,
}

impl ModelInterfaceWidget{
    pub fn set_value(&mut self, value: specrt::ModelInterface<ArcNpyArray>){
        self.inputs_widget.set_value(value.inputs().clone().into_inner());
        self.outputs_widget.set_value(value.outputs().clone().into_inner());
    }
}

impl Default for ModelInterfaceWidget {
    fn default() -> Self {
        Self {
            inputs_widget: StagingVec::default(),
            outputs_widget: StagingVec::default(),
            parsed: Err(GuiError::new("emtpy")), //FIXME?
        }
    }
}

impl ModelInterfaceWidget{
    pub fn update(&mut self){
        // self.inputs_widget.update();
        // self.outputs_widget.update();
        let inputs = match self.inputs_widget.state().into_iter().map(|i| i.clone()).collect::<Result<Vec<_>>>() {
            Ok(inps) => inps,
            Err(_) => {
                self.parsed = Err(GuiError::new("Check inputs for errors".to_owned()));
                return;
            }
        };
        let outputs = match self.outputs_widget.state().into_iter().map(|i| i.clone()).collect::<Result<Vec<_>>>() {
            Ok(outs) => outs,
            Err(_) => {
                self.parsed = Err(GuiError::new("Check outputs for errors".to_owned()));
                return;
            }
        };
        self.parsed = specrt::ModelInterface::try_build(inputs, outputs).map_err(|err| GuiError::from(err));
    }
}

impl StatefulWidget for ModelInterfaceWidget {
    type Value<'p> = &'p Result<specrt::ModelInterface<ArcNpyArray>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.update();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Inputs: ");
                self.inputs_widget.draw_and_parse(ui, id.with("in"));
            });
            ui.horizontal(|ui| {
                ui.strong("Outputs: ");
                self.outputs_widget.draw_and_parse(ui, id.with("out"));
            });

            show_if_error(ui, &self.parsed);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}
