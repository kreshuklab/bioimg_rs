use std::sync::Arc;

use indoc::indoc;

use bioimg_runtime::model_interface::{InputSlot, OutputSlot};
use bioimg_runtime::npy_array::ArcNpyArray;
use bioimg_runtime::NpyArray;

use crate::result::{GuiError, Result, VecResultExt};
use bioimg_spec::rdf::model as modelrdf;
use bioimg_spec::rdf::model::input_tensor as rdfinput;

use super::collapsible_widget::{CollapsibleWidget, SummarizableWidget};
use super::error_display::{show_error, show_if_error};
use super::posstprocessing_widget::PostprocessingWidget;
use super::preprocessing_widget::PreprocessingWidget;
use super::staging_string::StagingString;
use super::staging_vec::StagingVec;
use super::input_axis_widget::InputAxisWidget;
use super::output_axis_widget::OutputAxisWidget;
use super::test_tensor_widget::{TestTensorWidget, TestTensorWidgetState};
use super::{Restore, StatefulWidget, ValueWidget};
use crate::widgets::staging_vec::ItemWidgetConf;

#[derive(Restore)]
pub struct InputTensorWidget {
    #[restore_default]
    adjust_num_axes_on_file_selected: bool,

    pub id_widget: StagingString<modelrdf::TensorId>,
    pub is_optional: bool,
    pub description_widget: StagingString<modelrdf::TensorTextDescription>,
    pub axes_widget: StagingVec<CollapsibleWidget<InputAxisWidget>>,
    pub test_tensor_widget: TestTensorWidget,
    pub preprocessing_widget: StagingVec<CollapsibleWidget<PreprocessingWidget>>,

    #[restore_on_update]
    pub parsed: Result<InputSlot<Arc<NpyArray>>>,
}


impl Default for InputTensorWidget{
    fn default() -> Self {
        Self{
            adjust_num_axes_on_file_selected: false,
            id_widget: Default::default(),
            is_optional: Default::default(),
            description_widget: Default::default(),
            axes_widget: Default::default(),
            test_tensor_widget: Default::default(),
            preprocessing_widget: Default::default(),
            parsed: Err(GuiError::new("empty".to_owned())),
        }
    }
}

impl ValueWidget for InputTensorWidget{
    type Value<'v> = InputSlot<ArcNpyArray>;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.axes_widget.set_value(value.tensor_meta.axes().to_vec()); //FIXME
        self.preprocessing_widget.set_value(value.tensor_meta.preprocessing().clone()); //FIXME: clone
        self.id_widget.set_value(value.tensor_meta.id);
        self.description_widget.set_value(value.tensor_meta.description);
        self.test_tensor_widget.set_value(value.test_tensor);
    }
}

impl ItemWidgetConf for InputTensorWidget{
    const ITEM_NAME: &'static str = "Input Tensor";
}

impl ItemWidgetConf for CollapsibleWidget<InputTensorWidget>{
    const ITEM_NAME: &'static str = "Input Tensor";
    const GROUP_FRAME: bool = false;
}

impl SummarizableWidget for InputTensorWidget{
    fn summarize(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        self.update();
        match self.state(){
            Ok(slot) => {
                ui.label(slot.to_string());
            },
            Err(err) => {
                show_error(ui, err);
            }
        }
    }
}

impl InputTensorWidget{
    pub fn update(&mut self){
        'auto_adjust_axes: {
            let TestTensorWidgetState::Loaded { path, data: gui_npy_arr } = &*self.test_tensor_widget.state() else {
                self.adjust_num_axes_on_file_selected = true;
                break 'auto_adjust_axes;
            };
            if !self.adjust_num_axes_on_file_selected{
                break 'auto_adjust_axes;
            }
            self.adjust_num_axes_on_file_selected = false;
            if self.id_widget.raw.is_empty() {
                if let Some(path) = path{
                    self.id_widget.raw = path
                        .file_stem()
                        .map(|osstr| String::from(osstr.to_string_lossy()))
                        .unwrap_or(String::default());
                }
            }
            let sample_shape = gui_npy_arr.shape();
            let mut extents = sample_shape.iter().skip(self.axes_widget.staging.len());

            while let Some(extent) = extents.next() {
                let mut axis_widget = InputAxisWidget::default();
                axis_widget.axis_type_widget.value = if *extent == 1{
                    modelrdf::AxisType::Channel
                } else {
                    modelrdf::AxisType::Space
                };
                axis_widget.space_axis_widget.prefil_parameterized_size(*extent);
                self.axes_widget.staging.push(CollapsibleWidget { is_closed: false, inner: axis_widget })
            }
        }
        self.parsed = || -> Result<InputSlot<ArcNpyArray>> {
            let TestTensorWidgetState::Loaded { data: gui_npy_array, .. } = &*self.test_tensor_widget.state() else {
                return Err(GuiError::new("Test tensor is missing"));
            };
            let axes = self.axes_widget.state().into_iter().collect::<Result<Vec<_>>>()?;
            let sample_shape = gui_npy_array.shape();
            if sample_shape.len() != axes.len(){
                return Err(GuiError::new(format!(
                    "Example tensor has {} dimensions but there are {} axes defined", sample_shape.len(), axes.len()
                )))
            }
            let input_axis_group = modelrdf::InputAxisGroup::try_from(axes)?; //FIXME: parse in draw_and_parse?
            let meta_msg = rdfinput::InputTensorMetadataMsg{
                id: self.id_widget.state()?.clone(),
                optional: self.is_optional,
                preprocessing: self.preprocessing_widget.state().collect_result()?,
                description: self.description_widget.state()?.clone(),
                axes: input_axis_group,
            };
            Ok(
                InputSlot{ tensor_meta: meta_msg.try_into()?, test_tensor: Arc::clone(gui_npy_array) }
            )
        }();
    }
}

impl StatefulWidget for InputTensorWidget {
    type Value<'p> = &'p Result<InputSlot<ArcNpyArray>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.update();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Test Sample Input: ").on_hover_text(indoc!("
                    A .npy file with a sample input for testing this model. This data, along with that from other \
                    input tensors will be put through preprocessing and fed to the model network weights. \
                    The outputs from the network will then be postprocessed and compared to `Expected Test outputs` \
                    provided in the model output description fields to determine if this model is working properly."
                ));
                self.test_tensor_widget.draw_and_parse(ui, id.with("test tensor"));
                if matches!(&*self.test_tensor_widget.state(), TestTensorWidgetState::Empty) {
                    show_error(ui, "Missing a npy test tensor");
                }
            });
            ui.horizontal(|ui|{
                ui.strong("Input is optional: ").on_hover_text(indoc!("
                    Marks whether the model can do inference without this input."
                ));
                ui.add(egui::widgets::Checkbox::without_text(&mut self.is_optional));
            });
            ui.horizontal(|ui| {
                ui.strong("Tensor Id: ").on_hover_text(indoc!(
                    "The name of this input tensor. During inference, tensors are passed to the model as a \
                    mapping of strings to tensors; The keys in this Mapping should be the tensor IDs \
                    entered in fields like this one."
                ));
                self.id_widget.draw_and_parse(ui, id.with("Id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Description: ").on_hover_ui(|ui|{
                    ui.label(indoc!("
                        A human-readable description of this input tensor to help users of the model produce \
                        compliant inputs."
                    ));
                    ui.horizontal(|ui|{
                        ui.label("E.g.:");
                        ui.label(egui::RichText::new(indoc!("
                            'An xyz, float32 tensor with values between 0 and 1.0 representing \
                            the likelyhood of a pixel being a cell nucleus'"
                        )).italics());
                    });
                });
                self.description_widget.draw_and_parse(ui, id.with("Description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Axes: ").on_hover_text(indoc!("
                    A list of axis descriptions that determine how this tensor is to be interpreted. Notice \
                    that the axis should be given in C-order, i.e., that last axis given is the one that changes \
                    more quickly when going through the bytes of the tensor.
                "));
                self.axes_widget.draw_and_parse(ui, id.with("Axes"));
            });
            ui.horizontal(|ui| {
                ui.strong("Preprocessing: ").on_hover_text(indoc!("
                    A list of preprocessing steps that will be applied to this input tensor before it is \
                    fed to the model weights."
                ));
                self.preprocessing_widget.draw_and_parse(ui, id.with("preproc".as_ptr()));
            });
            show_if_error(ui, &self.parsed);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}

#[derive(Restore)]
pub struct OutputTensorWidget {
    #[restore_default]
    adjust_num_axes_on_file_selected: bool,

    pub id_widget: StagingString<modelrdf::TensorId>,
    pub description_widget: StagingString<modelrdf::TensorTextDescription>,
    pub axes_widget: StagingVec<CollapsibleWidget<OutputAxisWidget>>,
    pub test_tensor_widget: TestTensorWidget,
    pub postprocessing_widget: StagingVec<CollapsibleWidget<PostprocessingWidget>>,

    #[restore_on_update]
    pub parsed: Result<OutputSlot<Arc<NpyArray>>>,
}


impl Default for OutputTensorWidget{
    fn default() -> Self {
        Self{
            adjust_num_axes_on_file_selected: false,
            id_widget: Default::default(),
            description_widget: Default::default(),
            axes_widget: Default::default(),
            test_tensor_widget: Default::default(),
            postprocessing_widget: Default::default(),
            parsed: Err(GuiError::new("empty".to_owned()))
        }
    }
}

impl ValueWidget for OutputTensorWidget{
    type Value<'v> = OutputSlot<ArcNpyArray>;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.axes_widget.set_value(value.tensor_meta.axes().to_vec()); //FIXME
        self.postprocessing_widget.set_value(value.tensor_meta.postprocessing().clone());
        self.id_widget.set_value(value.tensor_meta.id);
        self.description_widget.set_value(value.tensor_meta.description);
        self.test_tensor_widget.set_value(value.test_tensor);
    }
}

impl ItemWidgetConf for OutputTensorWidget{
    const ITEM_NAME: &'static str = "Output Tensor";
}
impl ItemWidgetConf for CollapsibleWidget<OutputTensorWidget>{
    const ITEM_NAME: &'static str = "Output Tensor";
    const GROUP_FRAME: bool = false;
}

impl SummarizableWidget for OutputTensorWidget{
    fn summarize(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        self.update();
        match self.state(){
            Ok(slot) => {
                ui.label(slot.to_string());
            },
            Err(err) => {
                show_error(ui, err);
            }
        }
    }
}

impl OutputTensorWidget{
    pub fn update(&mut self){
        'auto_adjust_axes: {
            let TestTensorWidgetState::Loaded { path, data: gui_npy_arr } = &*self.test_tensor_widget.state() else {
                self.adjust_num_axes_on_file_selected = true;
                break 'auto_adjust_axes;
            };
            if !self.adjust_num_axes_on_file_selected{
                break 'auto_adjust_axes;
            }
            self.adjust_num_axes_on_file_selected = false;
            if self.id_widget.raw.is_empty() {
                if let Some(path) = path{
                    self.id_widget.raw = path
                        .file_stem()
                        .map(|osstr| String::from(osstr.to_string_lossy()))
                        .unwrap_or(String::default());
                }
            }
            let sample_shape = gui_npy_arr.shape();
            let mut extents = sample_shape.iter().skip(self.axes_widget.staging.len());

            while let Some(extent) = extents.next() {
                let mut axis_widget = OutputAxisWidget::default();
                axis_widget.axis_type_widget.value = if *extent == 1{
                    modelrdf::AxisType::Channel
                } else {
                    modelrdf::AxisType::Space
                };
                axis_widget.space_axis_widget.prefil_parameterized_size(*extent);
                self.axes_widget.staging.push(CollapsibleWidget { is_closed: false, inner: axis_widget })
            }
        }
        self.parsed = || -> Result<OutputSlot<ArcNpyArray>> {
            let TestTensorWidgetState::Loaded { data: gui_npy_array, .. } = &*self.test_tensor_widget.state() else {
                return Err(GuiError::new("Test tensor is missing"));
            };
            let axes = self.axes_widget.state().into_iter().collect::<Result<Vec<_>>>()?;
            let sample_shape = gui_npy_array.shape();
            if sample_shape.len() != axes.len(){
                return Err(GuiError::new(format!(
                    "Example tensor has {} dimensions but there are {} axes defined", sample_shape.len(), axes.len()
                )))
            }
            let axis_group = modelrdf::OutputAxisGroup::try_from(axes)?; //FIXME: parse in draw_and_parse?
            let meta_msg = modelrdf::output_tensor::OutputTensorMetadataMsg{
                id: self.id_widget.state()?.clone(),
                postprocessing: self.postprocessing_widget.state().collect_result()?,
                description: self.description_widget.state()?.clone(),
                axes: axis_group,
            };
            Ok(
                OutputSlot{ tensor_meta: meta_msg.try_into()?, test_tensor: Arc::clone(gui_npy_array) }
            )
        }();
    }
}

impl StatefulWidget for OutputTensorWidget {
    type Value<'p> = &'p Result<OutputSlot<ArcNpyArray>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.update();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Expected Test Output: ").on_hover_text(indoc!("
                    A .npy file with a sample output for testing this model. The 'Test Sample Inputs' from \
                    the model input fields will be put through preprocessing and fed to the model network weights. \
                    The outputs from the network will then be postprocessed and compared to the data in fields like \
                    this one, to determine if this model is working properly."
                ));
                self.test_tensor_widget.draw_and_parse(ui, id.with("test tensor"));
                if matches!(*self.test_tensor_widget.state(), TestTensorWidgetState::Empty) {
                    show_error(ui, "Missing a npy test tensor");
                }
            });
            ui.horizontal(|ui| {
                ui.strong("Tensor Id: ").on_hover_text(indoc!("
                    The name of this output tensor. Running this model will produce a mapping of strings \
                    to tensors. The keys in this mapping should be the IDs entered in this field."
                ));
                self.id_widget.draw_and_parse(ui, id.with("Id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Description: ").on_hover_ui(|ui|{
                    ui.label(indoc!("
                        A human-readable description of this output tensor to help users of the model \
                        understand the semantics of the model outputs."
                    ));
                    ui.horizontal(|ui|{
                        ui.label("E.g.:");
                        ui.label(
                            egui::RichText::new(indoc!("
                                'An xyz, float32 tensor with values between 0 and 1.0 representing \
                                the likelyhood of a pixel being a cell nucleus'"
                            ))
                            .italics()
                        );
                    });
                });
                self.description_widget.draw_and_parse(ui, id.with("Description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Axes: ").on_hover_text(indoc!("
                    A list of axis descriptions that determine how this tensor is to be interpreted. Notice \
                    that the axis should be given in C-order, i.e., that last axis given is the one that changes \
                    more quickly when going through the bytes of the tensor.
                "));
                self.axes_widget.draw_and_parse(ui, id.with("Axes"));
            });
            ui.horizontal(|ui| {
                ui.strong("Postprocessing: ").on_hover_text(indoc!("
                    A list of postprocessing steps that will be applied to this output tensor \
                    after it is produced by the network models."
                ));
                self.postprocessing_widget.draw_and_parse(ui, id.with("postproc".as_ptr()));
            });
            show_if_error(ui, &self.parsed);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}
