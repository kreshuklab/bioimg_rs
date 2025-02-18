use eframe::glow::MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS;
use egui::Widget;

use super::collapsible_widget::CollapsibleWidget;
use super::error_display::show_error;
use super::inout_tensor_widget::InputTensorWidget;
use super::preprocessing_widget::{PreprocessingWidget, PreprocessingWidgetMode, ShowPreprocTypePicker};
use super::util::{Arrow, EnumeratedItem};
use super::StatefulWidget;



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

fn input_frame<R, F>(ui: &mut egui::Ui, f: F) -> egui::InnerResponse<R>
where
    F: FnOnce(&mut egui::Ui) -> R
{
    let tip_length = 20.0;
    let frame_resp = ui.horizontal(|ui| {
        let resp = egui::Frame::new().inner_margin(egui::Margin::same(10)).show(ui, f);
        println!("Inner resp: {:?}", resp.response.rect);
        ui.add_space(tip_length);
        resp.inner
    });
    println!("Outter resp: {:?}", frame_resp.response.rect);

    let inp_rect = frame_resp.response.rect;
    let width = egui::Vec2{x: inp_rect.width() - tip_length, y: 0.0};
    let height = egui::Vec2{x: 0.0, y: inp_rect.height()};
    let top_left = inp_rect.min;
    let top_right = top_left + width; 
    let bottom_left = top_left + height;
    let bottom_right = top_right + height;
    let tip = egui::Pos2{x: inp_rect.max.x, y: inp_rect.center().y};
    
    let stroke = egui::Stroke{width: 2.0, color: egui::Color32::RED};

    ui.painter().line_segment([top_right, top_left], stroke);
    ui.painter().line_segment([top_left, bottom_left], stroke);
    ui.painter().line_segment([bottom_left, bottom_right], stroke);

    ui.painter().line_segment([bottom_right, tip], stroke);
    ui.painter().line_segment([tip, top_right], stroke);

    frame_resp
}

#[derive(Default,Clone)]
enum PipelineAction{
    #[default]
    Nothing,
    OpenInput{input_idx: usize},
    RemoveInput{input_idx: usize},
    OpenPreproc{input_idx: usize, preproc_idx: usize},
    RemovePreproc{input_idx: usize, preproc_idx: usize},
}

impl PipelineWidget{
    pub fn draw(
        &mut self,
        ui: &mut egui::Ui,
        id: egui::Id,
        inputs: &mut Vec<CollapsibleWidget<InputTensorWidget>>,
    ){

        let margin_width = 10;
        let margin = egui::Margin::same(10);
        let red_stroke = egui::Stroke{color: egui::Color32::RED, width: 2.0};

        let mut pipeline_action = self.action.clone();

        let (input_tips, weights_rect, _output_rects) = ui.horizontal(|ui|{
            let mut input_tips = Vec::<egui::Pos2>::new();

            ui.vertical(|ui| {
                let id = id.with("inputs".as_ptr());
                for (input_idx, cw) in inputs.iter_mut().enumerate(){
                    let inp = &mut cw.inner;
                    let id = id.with(input_idx);

                    let input_resp = input_frame(ui, |ui|{
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

                            let response = egui_dnd::dnd(ui, id.with("dnd".as_ptr()))
                            .with_animation_time(0.0)
                            .show(
                                inp.preprocessing_widget
                                    .iter_mut()
                                    .enumerate()
                                    .map(|(i, item)| EnumeratedItem { item, index: i }),
                                |ui, item, handle, _state| {
                                    handle.ui(ui, |ui| {
                                        if draw_preproc_button(ui, item.item).clicked(){
                                            pipeline_action = PipelineAction::OpenPreproc { input_idx, preproc_idx: item.index };
                                        }
                                    });
                                },
                            );

                            if response.is_drag_finished() {
                                response.update_vec(&mut inp.preprocessing_widget);
                            }

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
            self.action = pipeline_action;

            self.action = match std::mem::take(&mut self.action) {
                PipelineAction::OpenPreproc { input_idx, preproc_idx } => {
                    let id = id.with("modal".as_ptr());
                    let mut out = PipelineAction::OpenPreproc { input_idx, preproc_idx };
                    egui::Modal::new(id).show(ui.ctx(), |ui| {
                        ui.vertical(|ui|{
                            ui.with_layout(egui::Layout::right_to_left(Default::default()), |ui|{
                                if ui.button("🗙").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)){
                                    out = PipelineAction::Nothing;
                                }
                            });
                            inputs[input_idx].inner.preprocessing_widget[preproc_idx].draw_and_parse(
                                ui, ShowPreprocTypePicker::Show, id.with("widget".as_ptr())
                            );
                            ui.separator();
                            ui.horizontal(|ui|{
                                if ui.button("Remove").clicked(){
                                    inputs[input_idx].inner.preprocessing_widget.remove(preproc_idx);
                                    out = PipelineAction::Nothing;
                                }
                                if ui.button("Ok").clicked(){
                                    out = PipelineAction::Nothing;
                                }
                            });
                        })
                    });
                    out
                },
                PipelineAction::RemovePreproc { input_idx, preproc_idx } => {
                    inputs[input_idx].inner.preprocessing_widget.remove(preproc_idx);
                    PipelineAction::Nothing
                },
                PipelineAction::OpenInput { input_idx } => {
                    let id = id.with(input_idx).with("modal".as_ptr());
                    let mut out = PipelineAction::OpenInput { input_idx };
                    egui::Modal::new(id).show(ui.ctx(), |ui| {
                        ui.vertical(|ui|{
                            ui.with_layout(egui::Layout::right_to_left(Default::default()), |ui|{
                                if ui.button("🗙").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)){
                                    out = PipelineAction::Nothing;
                                }
                            });
                            inputs[input_idx].inner.draw_and_parse(ui, id.with("input widget".as_ptr()));
                            ui.separator();
                            ui.horizontal(|ui|{
                                if ui.button("Remove").clicked(){
                                    inputs.remove(input_idx);
                                    out = PipelineAction::Nothing;
                                }
                                if ui.button("Ok").clicked(){
                                    out = PipelineAction::Nothing;
                                }
                            });
                        })
                    });
                    out
                },
                PipelineAction::RemoveInput { input_idx } => {
                    inputs.remove(input_idx);
                    PipelineAction::Nothing
                },
                PipelineAction::Nothing => PipelineAction::Nothing,
            };

            ui.add_space(30.0);

            let weights_rect = ui.vertical(|ui|{
                egui::Frame::new()
                .inner_margin(egui::Margin::same(margin_width))
                .fill(egui::Color32::from_rgb(0, 255, 0))
                .show(ui, |ui|{
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                    ui.label("Weights go ehre xxxxxxxxxxxxxxxxxxxxxx");
                });
            }).response.rect;

            let output_rects = ui.vertical(|ui|{
                egui::Frame::new()
                .inner_margin(egui::Margin::same(margin_width))
                .fill(egui::Color32::from_rgb(0, 0, 255))
                .show(ui, |ui|{
                    let output_rects: Vec<_> = [1,2].iter()
                        .map(|idx| ui.label(format!("Input #{idx}")).rect)
                        .collect();
                    output_rects
                }).inner
            }).inner;

            (input_tips, weights_rect, output_rects)
        }).inner;

        let input_height_incr = weights_rect.height() / (input_tips.len() + 1) as f32;
        let color = egui::Color32::GRAY;

        for (idx, inp_tip) in input_tips.iter().enumerate(){
            let target = egui::Pos2{
                x: weights_rect.min.x,
                y: weights_rect.min.y + ((idx + 1) as f32 * input_height_incr),
            };

            // let control1 = egui::Pos2{x: origin.x + self.c1_x_offset, y: origin.y + self.c1_y_offset};
            // let control2 = egui::Pos2{x: target.x + self.c2_x_offset, y: target.y + self.c2_y_offset};
            let control1 = egui::Pos2{x: inp_tip.x + 20.0, y: inp_tip.y};
            let control2 = egui::Pos2{x: target.x + -20.0, y: target.y};

            // ui.painter().circle_filled(control1, 3.0, egui::Color32::YELLOW);
            // ui.painter().circle_filled(control2, 3.0, egui::Color32::LIGHT_BLUE);

            ui.painter().add(egui::epaint::CubicBezierShape{
                points: [
                    *inp_tip,
                    control1,
                    control2,
                    target,
                ],
                closed: false,
                fill: egui::Color32::TRANSPARENT,
                stroke: egui::Stroke{color, width: 2.0}.into(),
            });
            ui.painter().circle_filled(*inp_tip, 5.0, color);
            Arrow::new(target, egui::Pos2{x: target.x + 10.0, y: target.y}).color(color).draw(ui);
        }
    }
}
