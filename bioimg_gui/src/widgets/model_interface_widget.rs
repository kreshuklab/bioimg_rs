use indoc::indoc;

use super::collapsible_widget::CollapsibleWidget;
use super::preprocessing_widget::{PreprocessingWidget, PreprocessingWidgetMode};
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
            inputs_widget: {
                let mut out = StagingVec::default();
                out.staging = vec![
                    {
                        let mut cw: CollapsibleWidget<InputTensorWidget> = Default::default();
                        cw.inner.id_widget.raw = "blas".to_owned();
                        cw.inner.preprocessing_widget = vec![
                            {
                                let mut preproc: PreprocessingWidget = Default::default();
                                preproc.mode_widget.value = PreprocessingWidgetMode::Binarize;
                                preproc.binarize_widget.simple_binarize_widget.threshold_widget.raw = "1.2".to_owned();
                                preproc
                            },
                            {
                                let mut preproc: PreprocessingWidget = Default::default();
                                preproc.mode_widget.value = PreprocessingWidgetMode::Sigmoid;
                                preproc
                            }
                        ];
                        cw
                    }
                ];
                out
            },
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
            Err(e) => {
                self.parsed = Err(GuiError::new_with_rect("Check inputs for errors", e.failed_widget_rect));
                return;
            }
        };
        let outputs = match self.outputs_widget.state().into_iter().map(|i| i.clone()).collect::<Result<Vec<_>>>() {
            Ok(outs) => outs,
            Err(e) => {
                self.parsed = Err(GuiError::new_with_rect("Check outputs for errors", e.failed_widget_rect));
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
                ui.strong("Model Inputs: ").on_hover_text(indoc!("
                    During runtime, the model weights will be fed with input data. This input data must be \
                    in a particular shape, order, and of a particular data type (e.g. int32, float64, etc) \
                    to be accepted by the overall Zoo Model.

                    This data is preprocessed in a pipeline described in the 'preprocessing' fields, and then fed into the model weights."
                ));
                self.inputs_widget.draw_and_parse(ui, id.with("in"));
            });
            ui.horizontal(|ui| {
                ui.strong("Model Outputs: ").on_hover_text(indoc!("
                    The data comming out of the model weights is postprocessed (as specified in the 'postprocessing' \
                    field), and ultimately returned in the shape, order and data type specified in these fields."
                ));
                self.outputs_widget.draw_and_parse(ui, id.with("out"));
            });

            show_if_error(ui, &self.parsed);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}
