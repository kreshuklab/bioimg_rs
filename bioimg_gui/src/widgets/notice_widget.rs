use std::time::{Duration, Instant};

pub struct NoticeWidget{
    stop_at: Instant,
    message: String,
}

impl NoticeWidget{
    pub const FADE_TIME: Duration = Duration::from_secs(5);

    pub fn new_hidden() -> Self{
        Self {
            stop_at: Instant::now() - Duration::from_secs(10),
            message: "".into()
        }
    }

    pub fn update_message(&mut self, message: String){
        self.message = message;
        self.stop_at = Instant::now() + Self::FADE_TIME;
    }

    pub fn draw(&self, ui: &mut egui::Ui, now: std::time::Instant){
        if now > self.stop_at{
            return
        }
        let start_time = self.stop_at - Self::FADE_TIME;
        let delta = now - start_time;
        let progress = delta.as_millis() as f32 / Self::FADE_TIME.as_millis() as f32;

        let alpha = 255 - ( progress * 255.0 ) as u8;
        let color = egui::Color32::from_rgba_unmultiplied(255, 0, 0, alpha);
        ui.label(egui::RichText::new(&self.message).color(color));
        ui.ctx().request_repaint();
    }
}