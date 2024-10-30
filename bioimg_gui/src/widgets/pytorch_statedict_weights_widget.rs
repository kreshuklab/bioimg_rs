use std::{path::PathBuf, sync::Arc};

use bioimg_spec::rdf;
use bioimg_spec::rdf::model as modelrdf;
use bioimg_runtime as rt;

use crate::{project_data::PytorchArchModeRawData, result::Result};
use super::{file_source_widget::FileSourceWidget, file_widget::FileWidget, json_editor_widget::JsonObjectEditorWidget, search_and_pick_widget::SearchAndPickWidget, staging_opt::StagingOpt, staging_string::StagingString, util::group_frame, version_widget::VersionWidget, weights_widget::WeightsDescrBaseWidget, Restore, StatefulWidget, ValueWidget};

#[derive(Clone, strum::AsRefStr, strum::VariantArray, strum::VariantNames, Default, strum::Display)]
pub enum PytorchArchMode{
    #[default]
    #[strum(to_string="From File")]
    FromFile,
    #[strum(to_string="From Library")]
    FromLib
}

impl Restore for PytorchArchMode{
    type RawData = PytorchArchModeRawData;
    fn dump(&self) -> Self::RawData {
        match self{
            Self::FromFile => Self::RawData::FromFile,
            Self::FromLib => Self::RawData::FromLib,
        }
    }
    fn restore(&mut self, raw: Self::RawData) {
        *self = match raw{
            Self::RawData::FromFile => Self::FromFile,
            Self::RawData::FromLib => Self::FromLib,
        }
    }
}

#[derive(Default, Restore)]
pub struct PytorchArchWidget{
    pub mode_widget: SearchAndPickWidget<PytorchArchMode>,
    pub callable_widget: StagingString<rdf::Identifier>,
    pub kwargs_widget: JsonObjectEditorWidget,
    
    pub import_from_widget: StagingString<String>,
    pub source_widget: FileSourceWidget,
}

impl ValueWidget for PytorchArchWidget{
    type Value<'v> = rt::model_weights::PytorchArch;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        match value{
            rt::model_weights::PytorchArch::FromLib(fromlib) => {
                self.mode_widget.value = PytorchArchMode::FromLib;
                self.callable_widget.set_value(fromlib.callable);
                self.kwargs_widget.set_value(fromlib.kwargs);
                self.import_from_widget.set_value(fromlib.import_from);
            },
            rt::model_weights::PytorchArch::FromFile { file_source, callable, kwargs } => {
                self.mode_widget.value = PytorchArchMode::FromFile;
                self.source_widget.set_value(file_source);
                self.callable_widget.set_value(callable);
                self.kwargs_widget.set_value(kwargs);
            }
        }
    }
}


impl StatefulWidget for PytorchArchWidget{
    type Value<'p> = Result<rt::model_weights::PytorchArch>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Mode: ");
                self.mode_widget.draw_and_parse(ui, id.with("mode".as_ptr()));
            });
            match self.mode_widget.value{
                PytorchArchMode::FromLib => {
                    ui.horizontal(|ui|{
                        ui.strong("Import from: ").on_hover_text(
                            "A python module path where this model resides"
                        );
                        self.import_from_widget.draw_and_parse(ui, id.with("import".as_ptr()));
                        if !self.import_from_widget.raw.is_empty(){
                            ui.weak(format!(
                                "Will be interpreted as 'from {} import {}'",
                                self.import_from_widget.raw,
                                self.callable_widget.raw,
                            ));
                        }
                    });
                },
                PytorchArchMode::FromFile => {
                    ui.horizontal(|ui|{
                        ui.strong("Source File: ").on_hover_text("The source file where where the model code resides");
                        group_frame(ui, |ui|{
                            self.source_widget.draw_and_parse(ui, id.with("source".as_ptr()));
                        })
                    });
                }
            }
            ui.horizontal(|ui|{
                ui.strong("Callable: ").on_hover_text("A callable symbol inside the module from the 'Inmport From' field");
                self.callable_widget.draw_and_parse(ui, id.with("callable".as_ptr()));
            });
            ui.horizontal(|ui|{
                let callable_name = match self.callable_widget.state(){
                    Ok(identifier) => identifier.to_string(),
                    Err(_) => "the function in the 'Callable' field above".to_owned(),
                };
                ui.strong("Keyword Arguments: ").on_hover_text(format!("Keyword arguments to be passed to {callable_name}"));
                self.kwargs_widget.draw_and_parse(ui, id.with("kwargs".as_ptr()));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        match self.mode_widget.value{
            PytorchArchMode::FromFile => {
                Ok(rt::model_weights::PytorchArch::FromFile {
                    file_source: self.source_widget.state()?,
                    callable: self.callable_widget.state()?.clone(),
                    kwargs: self.kwargs_widget.state().as_ref().map_err(|err| err.clone())?.clone()
                })
            },
            PytorchArchMode::FromLib => {
                Ok(rt::model_weights::PytorchArch::FromLib(modelrdf::weights::PyTorchArchitectureFromLibraryDescr{
                    callable: self.callable_widget.state()?.clone(),
                    kwargs: self.kwargs_widget.state().as_ref().map_err(|err| err.clone())?.clone(),
                    import_from: self.import_from_widget.state()?.clone(),
                }))
            }
        }
    }
}

#[derive(Default, Restore)]
pub struct PytorchStateDictWidget{
    pub base_widget: WeightsDescrBaseWidget,
    pub architecture_widget: PytorchArchWidget,
    pub version_widget: VersionWidget,
    pub dependencies_widget: StagingOpt<FileWidget<Result<rt::CondaEnv>>>,
}

impl ValueWidget for PytorchStateDictWidget{
    type Value<'v> = rt::model_weights::PytorchStateDictWeights;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.base_widget.set_value(value.weights);
        self.architecture_widget.set_value(value.architecture);
        self.version_widget.set_value(value.pytorch_version);
        self.dependencies_widget.0 = value.dependencies.map(|deps| FileWidget::Finished {
            path: Arc::from(PathBuf::from("None").as_ref()), //FIXME
            value: Ok(deps)
        })
    }
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
            pytorch_version: self.version_widget.state()?.clone(),
        })
    }
}
