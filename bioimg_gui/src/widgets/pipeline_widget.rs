use std::ops::Mul;

use egui::Widget;
use indoc::indoc;

use crate::widgets::onnx_weights_widget::OnnxWeightsWidget;
use crate::widgets::pytorch_statedict_weights_widget::PytorchStateDictWidget;

use super::collapsible_widget::CollapsibleWidget;
use super::error_display::show_error;
use super::inout_tensor_widget::{InputTensorWidget, OutputTensorWidget};
use super::posstprocessing_widget::{PostprocessingWidget, PostprocessingWidgetMode, ShowPostprocTypePicker};
use super::preprocessing_widget::{PreprocessingWidget, PreprocessingWidgetMode, ShowPreprocTypePicker};
use super::util::Arrow;
use super::weights_widget::{KerasHdf5WeightsWidget, TorchscriptWeightsWidget, WeightsWidget};
use super::StatefulWidget;


#[derive(PartialEq, Eq, Copy, Clone)]
pub enum WeightsFlavor{
    Keras,
    Torchscript,
    PytorchStateDict,
    Onnx,
}

#[derive(Default)]
pub struct PipelineWidget{
    action: PipelineAction,
}

fn draw_preproc_button(ui: &mut egui::Ui, preproc: &PreprocessingWidget) -> egui::Response{
    let color = match preproc.mode{
        PreprocessingWidgetMode::Binarize => egui::Color32::GOLD,
        PreprocessingWidgetMode::Clip => egui::Color32::BLUE,
        PreprocessingWidgetMode::ScaleLinear => egui::Color32::GREEN,
        PreprocessingWidgetMode::Sigmoid => egui::Color32::ORANGE,
        PreprocessingWidgetMode::ZeroMeanUnitVariance => egui::Color32::BROWN,
        PreprocessingWidgetMode::ScaleRange => egui::Color32::DARK_GREEN,
        PreprocessingWidgetMode::EnsureDtype => egui::Color32::LIGHT_GRAY,
        PreprocessingWidgetMode::FixedZmuv => egui::Color32::KHAKI,
    };
    match preproc.iconify(){
        Ok(widget_text) => egui::Button::new(widget_text.color(egui::Color32::BLACK).strong()).fill(color).ui(ui),
        Err(err) => {
            let text = egui::RichText::new("!").color(egui::Color32::WHITE);
            egui::Button::new(text).fill(egui::Color32::RED).ui(ui).on_hover_ui(|ui| show_error(ui, err))
        }
    }
}

fn draw_postproc_button(ui: &mut egui::Ui, postproc: &PostprocessingWidget) -> egui::Response{
    let color = match postproc.mode{
        PostprocessingWidgetMode::Binarize => egui::Color32::GOLD,
        PostprocessingWidgetMode::Clip => egui::Color32::BLUE,
        PostprocessingWidgetMode::ScaleLinear => egui::Color32::GREEN,
        PostprocessingWidgetMode::Sigmoid => egui::Color32::ORANGE,
        PostprocessingWidgetMode::ZeroMeanUnitVariance => egui::Color32::BROWN,
        PostprocessingWidgetMode::ScaleRange => egui::Color32::DARK_GREEN,
        PostprocessingWidgetMode::EnsureDtype => egui::Color32::LIGHT_GRAY,
        PostprocessingWidgetMode::FixedZmuv => egui::Color32::KHAKI,
        PostprocessingWidgetMode::ScaleMeanVariance => egui::Color32::CYAN,
    };
    match postproc.iconify(){
        Ok(widget_text) => egui::Button::new(widget_text.color(egui::Color32::BLACK).strong()).fill(color).ui(ui),
        Err(err) => {
            let text = egui::RichText::new("!").color(egui::Color32::WHITE);
            egui::Button::new(text).fill(egui::Color32::RED).ui(ui).on_hover_ui(|ui| show_error(ui, err))
        }
    }
}

fn modal(
    id: egui::Id,
    ui: &mut egui::Ui,
    mut draw_widgets: impl FnMut(&mut egui::Ui) -> Option<PipelineAction>
) -> Option<PipelineAction>{
    let mut out = None;
    egui::Modal::new(id).show(ui.ctx(), |ui| {
        egui::ScrollArea::both()
        // .max_height(ui.ctx().screen_rect().max.y - 80.0)
        // .max_width(ui.ctx().screen_rect().max.x - 80.0)
        .min_scrolled_height(ui.ctx().screen_rect().max.y - 80.0)
        .min_scrolled_width(ui.ctx().screen_rect().max.x - 80.0)
        .show(ui, |ui|{
            ui.vertical(|ui|{
                ui.with_layout(egui::Layout::right_to_left(Default::default()), |ui|{
                    if ui.button("🗙").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)){
                        out = Some(PipelineAction::Nothing);
                    }
                });
                draw_widgets(ui).map(|val| out.insert(val))
            });
        });
    });
    out
}

fn draw_input_connections(
    ui: &mut egui::Ui, input_tips: &[egui::Pos2], weights_rect: egui::Rect, stroke: egui::Stroke
){
    let weights_rect_offset = egui::Vec2{
        x: 0.0,
        y: weights_rect.height() / (input_tips.len() as f32 + 1.0),
    };
    let max_inp_x = {
        let mut max_inp_x = f32::NEG_INFINITY;
        input_tips.iter().map(|tip| tip.x).for_each(|x| if x > max_inp_x { max_inp_x = x});
        max_inp_x
    };
    let arrow_offset = egui::Vec2{x: 10.0, y: 0.0};

    for (idx, inp_tip) in input_tips.iter().enumerate(){
        ui.painter().circle_filled(*inp_tip, 5.0, stroke.color);

        let curve_origin = egui::Pos2{x: max_inp_x, y: inp_tip.y};
        ui.painter().line_segment([*inp_tip, curve_origin], stroke);

        let target = weights_rect.min
            + weights_rect_offset.mul((idx + 1) as f32)
            - arrow_offset;

        let control1 = egui::Pos2{x: curve_origin.x + 20.0, y: curve_origin.y};
        let control2 = egui::Pos2{x: target.x + -20.0, y: target.y};

        ui.painter().add(egui::epaint::CubicBezierShape{
            points: [
                curve_origin,
                control1,
                control2,
                target,
            ],
            closed: false,
            fill: egui::Color32::TRANSPARENT,
            stroke: stroke.into(),
        });
        Arrow::new(target, target + arrow_offset).color(stroke.color).draw(ui);
    }
}

fn draw_output_connections(
    ui: &mut egui::Ui, output_tails: &[egui::Pos2], weights_rect: egui::Rect, stroke: egui::Stroke
){
    let weights_rect_offset = egui::Vec2{
        x: 0.0,
        y: weights_rect.height() / (output_tails.len() + 1) as f32,
    };
    let weights_widget_top_right = egui::Pos2{ x: weights_rect.max.x, y: weights_rect.min.y };
    let arrow_offset = egui::Vec2{x: 10.0, y: 0.0};

    for (idx, out_tail) in output_tails.iter().enumerate(){
        let curve_origin = weights_widget_top_right + weights_rect_offset.mul(idx as f32 + 1.0);
        ui.painter().circle_filled(curve_origin, 5.0, stroke.color);

        let target = *out_tail - arrow_offset;

        let control1 = egui::Pos2{x: curve_origin.x + 20.0, y: curve_origin.y};
        let control2 = egui::Pos2{x: target.x + -20.0, y: target.y};

        ui.painter().add(egui::epaint::CubicBezierShape{
            points: [
                curve_origin,
                control1,
                control2,
                target,
            ],
            closed: false,
            fill: egui::Color32::TRANSPARENT,
            stroke: stroke.into(),
        });
        Arrow::new(target, *out_tail).color(stroke.color).draw(ui);
    }
}

fn slot_frame<R, F>(ui: &mut egui::Ui, f: F) -> egui::InnerResponse<R>
where
    F: FnOnce(&mut egui::Ui) -> R
{
    let tip_length = 20.0;
    let frame_resp = ui.horizontal(|ui| {
        let resp = egui::Frame::new().inner_margin(egui::Margin::same(10)).show(ui, f);
        ui.add_space(tip_length);
        resp.inner
    });

    let inp_rect = frame_resp.response.rect;
    let width = egui::Vec2{x: inp_rect.width() - tip_length, y: 0.0};
    let height = egui::Vec2{x: 0.0, y: inp_rect.height()};
    let top_left = inp_rect.min;
    let top_right = top_left + width; 
    let bottom_left = top_left + height;
    let bottom_right = top_right + height;
    let tip = egui::Pos2{x: inp_rect.max.x, y: inp_rect.center().y};
    
    let stroke = ui.visuals().window_stroke();

    ui.painter().line_segment([top_right, top_left], stroke);
    ui.painter().line_segment([top_left, bottom_left], stroke);
    ui.painter().line_segment([bottom_left, bottom_right], stroke);

    ui.painter().line_segment([bottom_right, tip], stroke);
    ui.painter().line_segment([tip, top_right], stroke);

    frame_resp
}

fn draw_weights_widget(ui: &mut egui::Ui, out: &mut PipelineAction, weights_widget: &mut WeightsWidget) -> egui::Response {
    ui.vertical(|ui|{
        egui::Frame::new()
        .inner_margin(egui::Margin::same(10))
        .stroke(ui.style().visuals.window_stroke)
        .corner_radius(10.0)
        .show(ui, |ui|{
            ui.strong("Model Weights: ").on_hover_text(indoc!("
                The serialized weights and biases underlying this model.

                Model authors are strongly encouraged to use a format other than pytorch satedicts to maximize \
                intercompatibility between tools. Pytorch statedicts contain arbitrary python code and, crucially, \
                arbitrary dependencies that are very likely to clash with the dependencies of consumer applications. \
                Further, pytorch state dicts essentially require client applications to either be written in Python or \
                to ship the Python interpreter embedded into them.

                You can include mutiple flavors of your model weights, but they all MUST produce the same results"
            ));
            let keras_resp = match &weights_widget.keras_weights_widget.0 {
                None => ui.button("Keras: Empty"),
                Some(kw) => match kw.inner.state(){
                    Err(e) => ui.button(egui::RichText::new(format!("Keras: {e}")).color(egui::Color32::RED)),
                    Ok(state) => ui.button(egui::RichText::new(format!(
                        "Keras: tensorflow v{} {}",
                        state.tensorflow_version,
                        state.weights.source.to_string(),
                    )))
                }
            };
            if keras_resp.clicked(){
                *out = PipelineAction::OpenSpewcificWeights { flavor: WeightsFlavor::Keras };
            }

            let torchscript_resp = match &weights_widget.torchscript_weights_widget.0 {
                None => ui.button("Torchscript: Empty"),
                Some(w) => match w.inner.state(){
                    Err(e) => ui.button(egui::RichText::new(format!("Torchscript: {e}")).color(egui::Color32::RED)),
                    Ok(state) => ui.button(egui::RichText::new(format!(
                        "Torchscript: pytorch v{} {}",
                        state.pytorch_version,
                        state.weights.source.to_string(),
                    )))
                }
            };
            if torchscript_resp.clicked(){
                *out = PipelineAction::OpenSpewcificWeights { flavor: WeightsFlavor::Torchscript};
            }

            let state_dict_resp = match &weights_widget.pytorch_state_dict_weights_widget.0 {
                None => ui.button("Pytorch State Dict: Empty"),
                Some(w) => match w.inner.state(){
                    Err(e) => ui.button(egui::RichText::new(format!("Pytorch State Dict: {e}")).color(egui::Color32::RED)),
                    Ok(state) => ui.button(egui::RichText::new(format!(
                        "Pytorch State Dict: pytorch v{} {}",
                        state.pytorch_version,
                        state.weights.source.to_string(),
                    )))
                }
            };
            if state_dict_resp.clicked(){
                *out = PipelineAction::OpenSpewcificWeights { flavor: WeightsFlavor::PytorchStateDict};
            }

            let onnx_dict_resp = match &weights_widget.onnx_weights_widget.0 {
                None => ui.button("Onnx: Empty"),
                Some(w) => match w.inner.state(){
                    Err(e) => ui.button(egui::RichText::new(format!("Onnx: {e}")).color(egui::Color32::RED)),
                    Ok(state) => ui.button(egui::RichText::new(format!(
                        "Onnx: opset v{} {}",
                        state.opset_version,
                        state.weights.source.to_string(),
                    )))
                }
            };
            if onnx_dict_resp.clicked(){
                *out = PipelineAction::OpenSpewcificWeights { flavor: WeightsFlavor::Onnx};
            }
        });
    }).response
}

#[derive(Default,Clone)]
enum PipelineAction{
    #[default]
    Nothing,
    OpenWeights,
    OpenSpewcificWeights{flavor: WeightsFlavor},
    OpenOutput{output_idx: usize},
    RemoveOutput{output_idx: usize},
    OpenInput{input_idx: usize},
    RemoveInput{input_idx: usize},
    OpenPreproc{input_idx: usize, preproc_idx: usize},
    OpenPostproc{output_idx: usize, postproc_idx: usize},
}

impl PipelineWidget{
    pub fn draw(
        &mut self,
        ui: &mut egui::Ui,
        id: egui::Id,
        inputs: &mut Vec<CollapsibleWidget<InputTensorWidget>>,
        weights_widget: &mut WeightsWidget,
        outputs: &mut Vec<CollapsibleWidget<OutputTensorWidget>>,
    ){

        let mut pipeline_action = self.action.clone();

        let (input_tips, weights_rect, output_tails) = ui.horizontal(|ui|{
            let mut input_tips = Vec::<egui::Pos2>::new();
            let mut output_tails = Vec::<egui::Pos2>::new();

            ui.vertical(|ui| {
                ui.strong("Inputs:");
                let id = id.with("inputs".as_ptr());
                for (input_idx, cw) in inputs.iter_mut().enumerate(){
                    let inp = &mut cw.inner;
                    let _id = id.with(input_idx);

                    let input_resp = slot_frame(ui, |ui|{
                        if ui.button("🗙").clicked(){
                            pipeline_action = PipelineAction::RemoveInput{ input_idx };
                        }
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            let input_name = if inp.id_widget.raw.len() == 0{
                                egui::RichText::new("Unnamed input").weak()
                            } else {
                                egui::RichText::new(&inp.id_widget.raw).strong()
                            };
                            if ui.add(egui::Label::new(input_name).sense(egui::Sense::click())).clicked(){
                                pipeline_action = PipelineAction::OpenInput{input_idx};
                            }
                            ui.spacing_mut().item_spacing.x = 1.0;

                            inp.preprocessing_widget.iter().enumerate().for_each(|(idx, preproc)|{
                                if draw_preproc_button(ui, preproc).clicked(){
                                    pipeline_action = PipelineAction::OpenPreproc { input_idx, preproc_idx: idx };
                                }
                            });

                            ui.add_space(10.0);
                            if ui.button("✚").on_hover_text("Add preprocesing step").clicked(){
                                inp.preprocessing_widget.push(Default::default());
                                let preproc_idx = inp.preprocessing_widget.len() - 1;
                                pipeline_action = PipelineAction::OpenPreproc{ input_idx, preproc_idx };
                            }
                        });
                    });
                    let input_rect = input_resp.response.rect;
                    input_tips.push(egui::Pos2{
                        x: input_rect.max.x,
                        y: input_rect.center().y,
                    });
                }
                if ui.button("✚ Add Model Input").clicked(){
                    inputs.push(Default::default());
                    //FIXME: maybe open the editor?
                }
            });

            ui.add_space(30.0);

            let weights_rect = draw_weights_widget(ui, &mut pipeline_action, weights_widget).rect;
            ui.add_space(30.0);

            ui.vertical(|ui| {
                ui.add(egui::Label::new(egui::RichText::new("Outputs: ").strong()).wrap_mode(egui::TextWrapMode::Extend));
                let id = id.with("outputs".as_ptr());
                for (output_idx, cw) in outputs.iter_mut().enumerate(){
                    let output = &mut cw.inner;
                    let _id = id.with(output_idx);

                    let output_resp = slot_frame(ui, |ui|{
                        if ui.button("🗙").clicked(){
                            pipeline_action = PipelineAction::RemoveOutput{ output_idx };
                        }
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            let input_name = if output.id_widget.raw.len() == 0{
                                egui::RichText::new("Unnamed output").weak()
                            } else {
                                egui::RichText::new(&output.id_widget.raw).strong()
                            };
                            if ui.add(egui::Label::new(input_name).sense(egui::Sense::click())).clicked(){
                                pipeline_action = PipelineAction::OpenOutput{output_idx};
                            }
                            ui.spacing_mut().item_spacing.x = 1.0;

                            for (idx, postproc) in output.postprocessing_widgets.iter().enumerate(){
                                if draw_postproc_button(ui, &postproc.inner).clicked(){
                                    pipeline_action = PipelineAction::OpenPostproc { output_idx, postproc_idx: idx };
                                }
                            }

                            ui.add_space(10.0);
                            if ui.button("✚").on_hover_text("Add postprocessing step").clicked(){
                                output.postprocessing_widgets.push(Default::default());
                                let postproc_idx = output.postprocessing_widgets.len() - 1;
                                pipeline_action = PipelineAction::OpenPostproc { output_idx, postproc_idx };
                            }
                        });
                    });
                    let output_rect = output_resp.response.rect;
                    output_tails.push(egui::Pos2{
                        x: output_rect.min.x,
                        y: output_rect.center().y,
                    });
                }
                if ui.button("✚ Add Model Output").clicked(){
                    outputs.push(Default::default());
                    //FIXME: maybe open the editor?
                }
            });



            self.action = pipeline_action;


            (input_tips, weights_rect, output_tails)
        }).inner;

        let stroke = egui::Stroke{color: egui::Color32::GRAY, width: 2.0};
        draw_input_connections(ui, &input_tips, weights_rect, stroke);
        draw_output_connections(ui, &output_tails, weights_rect, stroke);

        macro_rules! weights_modal {($flavor:ident, $weights_widget:ty) => { paste::paste!{ {
            use itertools::Itertools;
            let id = id.with(stringify!([<$flavor _modal>]));
            modal(id, ui, |ui| {
                let mut action = None;

                let mut model_header = stringify!([<$flavor:snake>]).split("_").join(" ");
                model_header += " Weights";
                ui.heading(model_header);
                ui.separator();

                weights_widget.[<$flavor:snake _weights_widget>].0 = match std::mem::take(&mut weights_widget.[<$flavor _weights_widget>].0) {
                    None => {
                        let mut widget: CollapsibleWidget<$weights_widget> = Default::default();
                        widget.inner.draw_and_parse(ui, id.with("weights"));
                        Some(widget)
                    },
                    Some(mut widget) => {
                        widget.inner.draw_and_parse(ui, id.with("weights"));
                        Some(widget)
                    },
                };

                ui.horizontal(|ui|{
                    if ui.button("Remove").clicked(){
                        weights_widget.[<$flavor:snake _weights_widget>].0 = None;
                        action.replace(PipelineAction::Nothing);
                    }
                    if ui.button("Ok").clicked(){
                        action.replace(PipelineAction::Nothing);
                    }
                });
                action
            }).unwrap_or(PipelineAction::OpenSpewcificWeights { flavor: WeightsFlavor::[<$flavor:camel>] })
        } }};}

        self.action = match std::mem::take(&mut self.action) {
            PipelineAction::OpenWeights => {
                let modal_id = egui::Id::from("weights modal");
                modal(modal_id, ui, |ui|{
                    let mut action = None;
                    weights_widget.draw_and_parse(ui, modal_id.with("weights widget".as_ptr()));
                    ui.separator();
                    ui.horizontal(|ui|{
                        // if ui.button("Remove").clicked(){
                        //     inputs[input_idx].inner.preprocessing_widget.remove(preproc_idx);
                        //     out = PipelineAction::Nothing;
                        // }
                        if ui.button("Ok").clicked(){
                            action.replace(PipelineAction::Nothing);
                        }
                    });
                    action
                }).unwrap_or(PipelineAction::OpenWeights)
            },
            PipelineAction::OpenSpewcificWeights { flavor } => match flavor {
                WeightsFlavor::Keras => {
                    weights_modal!(keras, KerasHdf5WeightsWidget)
                },
                WeightsFlavor::Torchscript => {
                    weights_modal!(torchscript, TorchscriptWeightsWidget)
                },
                WeightsFlavor::PytorchStateDict => {
                    weights_modal!(pytorch_state_dict, PytorchStateDictWidget)
                },
                WeightsFlavor::Onnx => {
                    weights_modal!(onnx, OnnxWeightsWidget)
                },
            }
            PipelineAction::OpenPreproc { input_idx, preproc_idx } => {
                let id = id.with("preproc modal".as_ptr()).with(input_idx).with(preproc_idx);
                modal(id, ui, |ui| {
                    let mut action = None;
                    ui.vertical(|ui|{
                        inputs[input_idx].inner.preprocessing_widget[preproc_idx].draw_and_parse(
                            ui, ShowPreprocTypePicker::Show, id.with("widget".as_ptr())
                        );
                        ui.separator();
                        ui.horizontal(|ui|{
                            if ui.button("Remove").clicked(){
                                inputs[input_idx].inner.preprocessing_widget.remove(preproc_idx);
                                action.replace(PipelineAction::Nothing);
                            }
                            if ui.button("Ok").clicked(){
                                action.replace(PipelineAction::Nothing);
                            }
                        });
                    });
                    action
                }).unwrap_or(PipelineAction::OpenPreproc { input_idx, preproc_idx })
            },
            PipelineAction::OpenPostproc { output_idx, postproc_idx } => {
                let id = id.with("postproc modal".as_ptr()).with(output_idx).with(postproc_idx);
                modal(id, ui, |ui| {
                    let mut action = None;
                    ui.vertical(|ui|{
                        outputs[output_idx].inner.postprocessing_widgets[postproc_idx].inner.draw_and_parse(
                            ui, ShowPostprocTypePicker::Show, id.with("widget".as_ptr())
                        );
                        ui.separator();
                        ui.horizontal(|ui|{
                            if ui.button("Remove").clicked(){
                                outputs[output_idx].inner.postprocessing_widgets.remove(postproc_idx);
                                action.replace(PipelineAction::Nothing);
                            }
                            if ui.button("Ok").clicked(){
                                action.replace(PipelineAction::Nothing);
                            }
                        });
                    });
                    action
                }).unwrap_or(PipelineAction::OpenPostproc { output_idx, postproc_idx })
            }
            PipelineAction::OpenInput { input_idx } => {
                let id = id.with(input_idx).with("input modal".as_ptr());
                modal(id, ui, |ui| {
                    let mut action = None;
                    inputs[input_idx].inner.draw_and_parse(ui, id.with("input widget".as_ptr()));
                    ui.separator();
                    ui.horizontal(|ui|{
                        if ui.button("Remove").clicked(){
                            inputs.remove(input_idx);
                            action.replace(PipelineAction::Nothing);
                        }
                        if ui.button("Ok").clicked(){
                            action.replace(PipelineAction::Nothing);
                        }
                    });
                    action
                }).unwrap_or(PipelineAction::OpenInput { input_idx })
            },
            PipelineAction::OpenOutput { output_idx: input_idx } => {
                let id = id.with(input_idx).with("output modal".as_ptr());
                modal(id, ui, |ui| {
                    let mut action = None;
                    outputs[input_idx].inner.draw_and_parse(ui, id.with("output widget".as_ptr()));
                    ui.separator();
                    ui.horizontal(|ui|{
                        if ui.button("Remove").clicked(){
                            outputs.remove(input_idx);
                            action.replace(PipelineAction::Nothing);
                        }
                        if ui.button("Ok").clicked(){
                            action.replace(PipelineAction::Nothing);
                        }
                    });
                    action
                }).unwrap_or(PipelineAction::OpenOutput { output_idx: input_idx })
            },
            PipelineAction::RemoveInput { input_idx } => {
                inputs.remove(input_idx);
                PipelineAction::Nothing
            },
            PipelineAction::RemoveOutput{ output_idx: input_idx } => {
                outputs.remove(input_idx);
                PipelineAction::Nothing
            },
            PipelineAction::Nothing => PipelineAction::Nothing,
        };

    }
}
