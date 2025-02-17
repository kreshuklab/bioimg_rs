use std::{collections::VecDeque, time::{Duration, Instant}};

use egui::NumExt;

use crate::result::GuiError;

const FADE_TIME: Duration = Duration::from_secs(5);

struct Message{
    spawn_instant: Instant,
    text: String,
    color: egui::Color32,
    link_target: Option<egui::Rect>
}

impl Message{
    fn progress(&self) -> f32{
        let elapsed = Instant::now() - self.spawn_instant;
        (elapsed.as_millis() as f32 / FADE_TIME.as_millis() as f32).at_most(1.0)
    }
    fn is_done(&self) -> bool{
        self.spawn_instant + FADE_TIME < Instant::now()
    }
}

#[derive(Default)]
pub struct NotificationsWidget{
    messages: VecDeque<Message>,
    stop_fade: bool,
}

impl NotificationsWidget{
    pub fn new() -> Self{
        Self::default()
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
            link_target: None,
        });
    }
    pub fn push_gui_error(&mut self, error: GuiError){
        self.messages.push_back(Message{
            spawn_instant: Instant::now(),
            text: error.to_string(),
            color: egui::Color32::RED,
            link_target: error.failed_widget_rect,
        });
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, id: egui::Id) -> Option<egui::Rect>{
        let mut scroll_to: Option<egui::Rect> = None;
        if self.messages.len() == 0{
            return scroll_to
        }
        let now = Instant::now();
        let area = egui::Window::new("Notifications")
            .id(id)
            .title_bar(false)
            .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::ZERO)
            .order(egui::Order::Foreground)
            .movable(false)
            .collapsible(false)
            .resizable(true)
            .interactable(true);
        let area_resp = area.show(ui.ctx(), |ui| {
            let frame = egui::Frame::popup(&ui.ctx().style())
                .corner_radius(egui::CornerRadius::default())
                .outer_margin(0.0);
            frame.show(ui, |ui| {
                self.messages.retain_mut(|msg|{
                    if self.stop_fade{
                        msg.spawn_instant = now;
                    }
                    if msg.is_done(){
                        false
                    } else {
                        let alpha = 1.0 - msg.progress();
                        let rich_text = egui::RichText::new(&msg.text).color(msg.color.gamma_multiply(alpha));
                        match msg.link_target{
                            Some(rect) => {
                                let rich_text = rich_text.underline();
                                if ui.link(rich_text).clicked(){
                                    scroll_to.replace(rect);
                                }
                            },
                            None => {
                                ui.label(rich_text);
                            },
                        }
                        ui.ctx().request_repaint();
                        true
                    }
                });
            });
        });
        if let Some(inner_response) = area_resp{
            self.stop_fade = inner_response.response.contains_pointer();
        }
        scroll_to
    }
}
