use egui::Widget;

use super::collapsible_widget::CollapsibleWidget;
use super::error_display::show_error;
use super::inout_tensor_widget::InputTensorWidget;
use super::preprocessing_widget::{PreprocessingWidget, PreprocessingWidgetMode, ShowPreprocTypePicker};
use super::util::Arrow;



#[derive(Default)]
pub struct PipelineWidget{
    popup_id: Option<egui::Id>,
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

enum PipelineAction{
    Nothing,
    Remove{index: usize}
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

        let inputs_base_id = id.with("inputs".as_ptr());

        let (input_rects, weights_rect, _output_rects) = ui.horizontal(|ui|{
            let mut input_rects = Vec::<egui::Rect>::new();
            ui.vertical(|ui| {
                for (idx, cw) in inputs.iter_mut().enumerate(){
                    let inp = &mut cw.inner;
                    let inp_id = inputs_base_id.with(idx);

                    let frame_resp = egui::Frame::new().inner_margin(margin).stroke(red_stroke).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.strong(&inp.id_widget.raw);
                            ui.spacing_mut().item_spacing.x = 1.0;

                            let mut preproc_action = PipelineAction::Nothing;
                            for (idx, preproc) in inp.preprocessing_widget.iter_mut().enumerate(){
                                let preproc_id = inp_id.with(idx);
                                if draw_preproc_button(ui, preproc).clicked(){
                                    self.popup_id = Some(preproc_id);
                                }
                                let Some(id) = self.popup_id else{
                                    continue
                                };
                                if id != preproc_id{
                                    continue
                                }
                                egui::Modal::new(id.with("modal".as_ptr())).show(ui.ctx(), |ui| {
                                    ui.vertical(|ui|{
                                        ui.with_layout(egui::Layout::right_to_left(Default::default()), |ui|{
                                            if ui.button("🗙").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)){
                                                self.popup_id = None;
                                            }
                                        });
                                        preproc.draw_and_parse(ui, ShowPreprocTypePicker::Show, id.with("widget".as_ptr()));
                                        ui.separator();
                                        ui.horizontal(|ui|{
                                            if ui.button("Remove").clicked(){
                                                preproc_action = PipelineAction::Remove { index: idx };
                                                self.popup_id = None;
                                            }
                                            if ui.button("Ok").clicked(){
                                                self.popup_id = None;
                                            }
                                        });
                                    })
                                });
                            }
                            if let PipelineAction::Remove { index } = preproc_action{
                                inp.preprocessing_widget.remove(index);
                            }
                        });
                    });
                    input_rects.push(frame_resp.response.rect);
                }
            });

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

            (input_rects, weights_rect, output_rects)
        }).inner;

        let input_height_incr = weights_rect.height() / (input_rects.len() + 1) as f32;
        let color = egui::Color32::GRAY;

        for (idx, inp_rec) in input_rects.iter().enumerate(){
            let origin = egui::Pos2{
                x: inp_rec.max.x,
                y: inp_rec.center().y,
            };
            let target = egui::Pos2{
                x: weights_rect.min.x,
                y: weights_rect.min.y + ((idx + 1) as f32 * input_height_incr),
            };

            // let control1 = egui::Pos2{x: origin.x + self.c1_x_offset, y: origin.y + self.c1_y_offset};
            // let control2 = egui::Pos2{x: target.x + self.c2_x_offset, y: target.y + self.c2_y_offset};
            let control1 = egui::Pos2{x: origin.x + 20.0, y: origin.y};
            let control2 = egui::Pos2{x: target.x + -20.0, y: target.y};

            // ui.painter().circle_filled(control1, 3.0, egui::Color32::YELLOW);
            // ui.painter().circle_filled(control2, 3.0, egui::Color32::LIGHT_BLUE);

            ui.painter().add(egui::epaint::CubicBezierShape{
                points: [
                    origin,
                    control1,
                    control2,
                    target,
                ],
                closed: false,
                fill: egui::Color32::TRANSPARENT,
                stroke: egui::Stroke{color, width: 2.0}.into(),
            });
            ui.painter().circle_filled(origin, 5.0, color);
            Arrow::new(target, egui::Pos2{x: target.x + 10.0, y: target.y}).color(color).draw(ui);
        }
    }
}
