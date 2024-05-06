use bioimg_spec::rdf;
use bioimg_spec::rdf::model as modelrdf;
use bioimg_runtime as rt;

use crate::result::Result;
use super::{file_widget::FileWidget, json_editor_widget::JsonObjectEditorWidget, staging_opt::StagingOpt, staging_string::StagingString, util::group_frame, weights_widget::WeightsDescrBaseWidget, StatefulWidget};

#[derive(Default)]
pub struct PytorchArchWidget{
    pub callable_widget: StagingString<rdf::Identifier>,
    pub kwargs_widget: JsonObjectEditorWidget,
    pub import_from_widget: StagingString<String>,
}


impl StatefulWidget for PytorchArchWidget{
    type Value<'p> = Result<modelrdf::weights::PytorchArchitectureDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Callable: ");
                self.callable_widget.draw_and_parse(ui, id.with("callable".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Keyword Arguments: ");
                self.kwargs_widget.draw_and_parse(ui, id.with("kwargs".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Import from: ");
                self.import_from_widget.draw_and_parse(ui, id.with("import".as_ptr()));
                if !self.import_from_widget.raw.is_empty(){
                    ui.weak(format!(
                        "Will be interpreted as 'from {} import {}'",
                        self.import_from_widget.raw,
                        self.callable_widget.raw,
                    ));
                }
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        if self.import_from_widget.raw.is_empty(){
            Ok(modelrdf::weights::PyTorchArchitectureFromFileDescr{
                callable: self.callable_widget.state()?,
                kwargs: self.kwargs_widget.state().as_ref().map_err(|err| err.clone())?.clone()
            }.into())
        }else{
            Ok(modelrdf::weights::PyTorchArchitectureFromLibraryDescr{
                callable: self.callable_widget.state()?,
                kwargs: self.kwargs_widget.state().as_ref().map_err(|err| err.clone())?.clone(),
                import_from: self.import_from_widget.state()?,
            }.into())
        }
    }
}

#[derive(Default)]
pub struct PytorchStateDictWidget{
    pub base_widget: WeightsDescrBaseWidget,
    pub architecture_widget: PytorchArchWidget,
    pub version_widget: StagingString<rdf::Version>,
    pub dependencies_widget: StagingOpt<FileWidget<Result<rt::CondaEnv>>>,
}

impl StatefulWidget for PytorchStateDictWidget{
    type Value<'p> = Result<rt::model_weights::PytorchStateDictWeights>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            self.base_widget.draw_and_parse(ui, id.with("base".as_ptr()));
            ui.horizontal(|ui|{
                ui.strong("Architecture: ");
                group_frame(ui, |ui|{
                    self.architecture_widget.draw_and_parse(ui, id.with("arch".as_ptr()));
                })
            });
            ui.horizontal(|ui|{
                ui.strong("Pytorch Version: ");
                self.version_widget.draw_and_parse(ui, id.with("ver".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Environment File: ");
                self.dependencies_widget.draw_and_parse(ui, id.with("env".as_ptr()));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(rt::model_weights::PytorchStateDictWeights{
            dependencies: self.dependencies_widget.state().map(|file_state| file_state.ok()).transpose()?.cloned(),
            weights: self.base_widget.state()?,
            architecture: self.architecture_widget.state()?,
            pytorch_version: self.version_widget.state()?,
        })
    }
}
