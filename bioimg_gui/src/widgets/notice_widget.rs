use std::{collections::VecDeque, time::{Duration, Instant}};

use egui::NumExt;

const FADE_TIME: Duration = Duration::from_secs(5);

struct Message{
    spawn_instant: Instant,
    text: String,
    color: egui::Color32,
}

impl Message{
    fn progress(&self) -> f32{
        let elapsed = Instant::now() - self.spawn_instant;
        (elapsed.as_millis() as f32 / FADE_TIME.as_millis() as f32).at_most(1.0)
    }
    fn is_done(&self) -> bool{
        self.spawn_instant + FADE_TIME < Instant::now()
    }
    fn draw(&self, ui: &mut egui::Ui) -> egui::Response{
        let alpha = 1.0 - self.progress();
        let rich_text = egui::RichText::new(&self.text).color(self.color.gamma_multiply(alpha));
        ui.label(rich_text)
    }
}

#[derive(Default)]
pub struct NotificationsWidget{
    messages: VecDeque<Message>,
    stop_fade: bool,
}

impl NotificationsWidget{
    pub fn new() -> Self{
        Self{messages: VecDeque::new(), stop_fade: false}
    }
    pub fn push_message(&mut self, message_text: Result<String, String>){
        let (text, color) = match message_text{
            Ok(text) => (text, egui::Color32::GREEN),
            Err(text) => (text, egui::Color32::RED),
        };
        self.messages.push_back(Message{
            spawn_instant: Instant::now(),
            text,
            color,
        });
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, id: egui::Id){
        if self.messages.len() == 0{
            return
        }
        let now = Instant::now();
        let area = egui::containers::Area::new(id)
            .movable(false)
            .order(egui::Order::Foreground)
            .constrain(true)
            .movable(false)
            .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO);
            // .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::ZERO);
        let area_resp = area.show(ui.ctx(), |ui| {
            let frame = egui::Frame::popup(&ui.ctx().style())
                .rounding(egui::Rounding::default())
                .outer_margin(0.0);
            frame.show(ui, |ui| {
                self.messages.retain_mut(|msg|{
                    if self.stop_fade{
                        msg.spawn_instant = now;
                    }
                    if msg.is_done(){
                        false
                    } else {
                        msg.draw(ui);
                        ui.ctx().request_repaint();
                        true
                    }
                });
            });
        });
        self.stop_fade = area_resp.response.contains_pointer();
    }
}
