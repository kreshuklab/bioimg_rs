use std::sync::Arc;

use bioimg_runtime as rt;

use crate::result::{GuiError, Result, VecResultExt};
use super::{
    author_widget::AuthorWidget, collapsible_widget::CollapsibleWidget, error_display::show_error, file_source_widget::FileSourceWidget, onnx_weights_widget::OnnxWeightsWidget, pytorch_statedict_weights_widget::PytorchStateDictWidget, staging_opt::StagingOpt, staging_vec::StagingVec, util::group_frame, version_widget::VersionWidget, Restore, StatefulWidget, ValueWidget
};

#[derive(Restore)]
pub struct WeightsWidget{
    pub keras_weights_widget: StagingOpt<KerasHdf5WeightsWidget>,
    pub torchscript_weights_widget: StagingOpt<TorchscriptWeightsWidget>,
    pub pytorch_state_dict_widget: StagingOpt<PytorchStateDictWidget>,
    pub onnx_eights_widget: StagingOpt<OnnxWeightsWidget>,
    #[restore_on_update]
    parsed: Result<Arc<rt::ModelWeights>>
}

impl Default for WeightsWidget{
    fn default() -> Self {
        Self {
            keras_weights_widget: Default::default(),
            torchscript_weights_widget: Default::default(),
            pytorch_state_dict_widget: Default::default(),
            onnx_eights_widget: Default::default(),
            parsed: Err(GuiError::new("empty"))
        }
    }
}

impl ValueWidget for WeightsWidget{
    type Value<'v> = rt::ModelWeights;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.keras_weights_widget.set_value(value.keras_hdf5().cloned());
        self.torchscript_weights_widget.set_value(value.torchscript().cloned());
        self.pytorch_state_dict_widget.set_value(value.pytorch_state_dict().cloned());
        self.onnx_eights_widget.set_value(value.onnx().cloned());
    }
}

impl WeightsWidget{
    pub fn update(&mut self){
        self.parsed = (|| {
            Ok(Arc::new(rt::ModelWeights::new(
                self.keras_weights_widget.state().transpose()?,
                self.onnx_eights_widget.state().transpose()?,
                self.pytorch_state_dict_widget.state().transpose()?,
                None,
                None,
                self.torchscript_weights_widget.state().transpose()?,
            )?)
        )})();
    }
}

impl StatefulWidget for WeightsWidget{
    type Value<'p> = Result<Arc<rt::ModelWeights>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        self.update();
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Torchscript: ");
                self.torchscript_weights_widget.draw_and_parse(ui, id.with("tsweights".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Pytorch state dict: ");
                self.pytorch_state_dict_widget.draw_and_parse(ui, id.with("pytorch".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Keras: ");
                self.keras_weights_widget.draw_and_parse(ui, id.with("keras".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Onnx: ");
                self.onnx_eights_widget.draw_and_parse(ui, id.with("onnx".as_ptr()));
            });

            if self.parsed.is_err(){
                show_error(ui, "Please review the model weights");
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}

#[derive(Default, Restore)]
pub struct WeightsDescrBaseWidget{
    pub source_widget: FileSourceWidget,
    pub authors_widget: StagingOpt<StagingVec<CollapsibleWidget<AuthorWidget>>>,
    // pub parent_widget: Option<WeightsFormat>,
}

impl ValueWidget for WeightsDescrBaseWidget{
    type Value<'v> = rt::WeightsBase;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.source_widget.set_value(value.source);
        self.authors_widget.set_value(value.authors);
    }
}

impl StatefulWidget for WeightsDescrBaseWidget{
    type Value<'p> = Result<rt::WeightsBase>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Source: ").on_hover_text("The file containing the serialized weights and biases");
                group_frame(ui, |ui|{
                    self.source_widget.draw_and_parse(ui, id.with("source"));
                });
            });
            ui.horizontal(|ui|{
                ui.strong("Authors: ").on_hover_text("The people who trained these weights and biases");
                self.authors_widget.draw_and_parse(ui, id.with("authors"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let authors  = self.authors_widget.state().map(|authors|{
            authors.collect_result()
        }).transpose()?;
        let source = self.source_widget.state().map_err(|_| GuiError::new("Review cover images"))?;
        Ok(rt::WeightsBase{authors, source})
    }
}

//////////////////////////////

#[derive(Default, Restore)]
pub struct KerasHdf5WeightsWidget{
    pub base_widget: WeightsDescrBaseWidget,
    pub tensorflow_version_widget: VersionWidget,
}

impl ValueWidget for KerasHdf5WeightsWidget{
    type Value<'v> = rt::KerasHdf5Weights;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.base_widget.set_value(value.weights);
        self.tensorflow_version_widget.set_value(value.tensorflow_version);
    }
}

impl StatefulWidget for KerasHdf5WeightsWidget{
    type Value<'p> = Result<rt::KerasHdf5Weights>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            self.base_widget.draw_and_parse(ui, id.with("base"));
            ui.horizontal(|ui|{
                ui.strong("Tensor Flow Version: ").on_hover_text(
                    "Version of the tensor flow library used when training these weights and biases"
                );
                self.tensorflow_version_widget.draw_and_parse(ui, id.with("tfversion"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(rt::KerasHdf5Weights{
            weights: self.base_widget.state()?,
            tensorflow_version: self.tensorflow_version_widget.state()?.clone(),
        })
    }
}

////////////////////////////

#[derive(Default, Restore)]
pub struct TorchscriptWeightsWidget{
    pub base_widget: WeightsDescrBaseWidget,
    pub pytorch_version_widget: VersionWidget,
}

impl ValueWidget for TorchscriptWeightsWidget{
    type Value<'v> = rt::TorchscriptWeights;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.base_widget.set_value(value.weights);
        self.pytorch_version_widget.set_value(value.pytorch_version);
    }
}

impl StatefulWidget for TorchscriptWeightsWidget{
    type Value<'p> = Result<rt::TorchscriptWeights>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            self.base_widget.draw_and_parse(ui, id.with("base"));
            ui.horizontal(|ui|{
                ui.strong("Pytorch Version: ").on_hover_text("The pytorch library version used when training these weights and biases");
                self.pytorch_version_widget.draw_and_parse(ui, id.with("ptversion"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(rt::TorchscriptWeights{
            weights: self.base_widget.state()?,
            pytorch_version: self.pytorch_version_widget.state()?.clone(),
        })
    }
}
