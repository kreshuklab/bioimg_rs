use std::time::{Duration, Instant};

pub struct NoticeWidget{
    show_instant: Instant,
    show_duration: Duration,
    message: String,
}

impl NoticeWidget{
    pub fn new_hidden() -> Self{
        Self {
            show_instant: Instant::now() - Duration::from_secs(1),
            show_duration: Duration::from_micros(1),
            message: String::new()
        }
    }

    pub fn update_message(&mut self, message: String){
        self.message = message;
        self.show_duration = Duration::from_secs(5);
    }

    pub fn hide(&mut self){
        self.show_instant = Instant::now() - Duration::from_secs(1);
        self.show_duration = Duration::from_micros(1);
    }

    pub fn draw(&self, ui: &mut egui::Ui, now: std::time::Instant){
        let delta = now - self.show_instant;
        if delta > self.show_duration{
            return
        }
        let progress = delta.as_millis() as f32 / self.show_duration.as_millis() as f32;

        ui.label(&self.message);
        let alpha = 255 - ( progress * 255.0 ) as u8;
        let color = egui::Color32::from_rgba_unmultiplied(255, 0, 0, alpha);
        ui.label(egui::RichText::new(&self.message).color(color));
    }
}