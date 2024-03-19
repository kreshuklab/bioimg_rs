use std::path::PathBuf;

use bioimg_spec::rdf::Version;
use bioimg_runtime as rt;

use crate::result::{GuiError, Result, VecResultExt};
use super::{
    author_widget::StagingAuthor2, file_widget::{FileWidget, FileWidgetState}, staging_opt::StagingOpt, staging_string::StagingString, staging_vec::StagingVec, StatefulWidget
};

#[derive(Default)]
pub struct WeightsWidget{
    pub keras_weights_widget: StagingOpt<KerasHdf5WeightsWidget>,
    pub torchscript_weights_widget: StagingOpt<TorchscriptWeightsWidget>,
}

impl StatefulWidget for WeightsWidget{
    type Value<'p> = Result<rt::ModelWeights>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Torchscript weights: ");
                self.torchscript_weights_widget.draw_and_parse(ui, id.with("tsweights"));
            });
            ui.horizontal(|ui|{
                ui.strong("Keras Weights: ");
                self.keras_weights_widget.draw_and_parse(ui, id.with("keras"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(rt::ModelWeights::new(
            self.keras_weights_widget.state().transpose()?,
            None,
            None,
            None,
            None,
            self.torchscript_weights_widget.state().transpose()?,
        )?)
    }
}

#[derive(Default)]
pub struct WeightsDescrBaseWidget{
    pub source_widget: FileWidget<Result<PathBuf>>,
    pub authors_widget: StagingOpt<StagingVec<StagingAuthor2>>,
    // pub parent_widget: Option<WeightsFormat>,
}

impl StatefulWidget for WeightsDescrBaseWidget{
    type Value<'p> = Result<rt::WeightsBase>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Source: ");
                self.source_widget.draw_and_parse(ui, id.with("source"));
            });
            ui.horizontal(|ui|{
                ui.strong("Authors: ");
                self.authors_widget.draw_and_parse(ui, id.with("authors"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let authors  = self.authors_widget.state().map(|authors|{
            authors.collect_result()
        }).transpose()?;
        let source = match self.source_widget.state(){
            FileWidgetState::Finished { value: Ok(val), .. } => val.clone(),
            _ => return Err(GuiError::new("Review cover images".into()))
        };
        Ok(rt::WeightsBase{authors, source})
    }
}

//////////////////////////////

#[derive(Default)]
pub struct KerasHdf5WeightsWidget{
    pub base_widget: WeightsDescrBaseWidget,
    pub tensorflow_version_widget: StagingString<Version>,
}

impl StatefulWidget for KerasHdf5WeightsWidget{
    type Value<'p> = Result<rt::KerasHdf5Weights>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            self.base_widget.draw_and_parse(ui, id.with("base"));
            ui.horizontal(|ui|{
                ui.strong("Tensor Flow Version: ");
                self.tensorflow_version_widget.draw_and_parse(ui, id.with("tfversion"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(rt::KerasHdf5Weights{
            weights: self.base_widget.state()?,
            tensorflow_version: self.tensorflow_version_widget.state()?,
        })
    }
}

////////////////////////////

#[derive(Default)]
pub struct TorchscriptWeightsWidget{
    pub base_widget: WeightsDescrBaseWidget,
    pub pytorch_version_widget: StagingString<Version>,
}

impl StatefulWidget for TorchscriptWeightsWidget{
    type Value<'p> = Result<rt::TorchscriptWeights>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            self.base_widget.draw_and_parse(ui, id.with("base"));
            ui.horizontal(|ui|{
                ui.strong("Pytorch Version: ");
                self.pytorch_version_widget.draw_and_parse(ui, id.with("ptversion"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(rt::TorchscriptWeights{
            weights: self.base_widget.state()?,
            pytorch_version: self.pytorch_version_widget.state()?,
        })
    }
}