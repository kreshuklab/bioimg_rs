use bioimg_spec::rdf::model as modelrdf;

use super::collapsible_widget::{CollapsibleWidget, SummarizableWidget};
use super::inout_tensor_widget::InputTensorWidget;
use super::staging_string::StagingString;
use super::util::Arrow;
use super::StatefulWidget;





#[derive(Default)]
pub struct PipelineWidget{
}


impl PipelineWidget{
    pub fn draw(
        &mut self,
        ui: &mut egui::Ui,
        id: egui::Id,
        inputs: &mut Vec<CollapsibleWidget<InputTensorWidget>>,
    ){

        let margin_width = 10.0;
        let (input_rects, weights_rect, output_rects) = ui.horizontal(|ui|{
            let inp_id = id.with("inputs".as_ptr());
            let inp_margin = egui::Margin::same(margin_width);
            let input_rects: Vec<egui::Rect> = ui.vertical(|ui| inputs.iter_mut()
                .map(|collapsible| &mut collapsible.inner)
                .enumerate()
                .map(|(idx, inp)| egui::Frame::none()
                    .inner_margin(inp_margin)
                    .stroke(egui::Stroke{color: egui::Color32::RED, width: 2.0})
                    .show(ui, |ui|{
                        let id = inp_id.with(idx);
                        inp.summarize2(ui, id);
                    }).response.rect
                )
                .collect()
            ).inner;

            ui.add_space(30.0);

            let weights_rect = ui.vertical(|ui|{
                egui::Frame::none()
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
                egui::Frame::none()
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
