use std::{ops::Deref, sync::mpsc::{Receiver, Sender}, time::Instant};

use egui::InnerResponse;

pub trait DynamicImageExt {
    fn to_egui_texture_handle(&self, name: impl Into<String>, ctx: &egui::Context) -> egui::TextureHandle;
}

impl DynamicImageExt for image::DynamicImage {
    fn to_egui_texture_handle(&self, name: impl Into<String>, ctx: &egui::Context) -> egui::TextureHandle {
        let size = [self.width() as _, self.height() as _];
        let rgb_image = self.to_rgb8();
        let pixels = rgb_image.as_flat_samples();
        let texture_image = egui::ColorImage::from_rgb(size, pixels.as_slice());
        ctx.load_texture(
            name,
            texture_image,
            egui::TextureOptions {
                magnification: egui::TextureFilter::Linear,
                minification: egui::TextureFilter::Nearest,
                wrap_mode: egui::TextureWrapMode::ClampToEdge,
                mipmap_mode: None,
            },
        )
    }
}

pub fn group_frame<R>(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> InnerResponse<R> {
    let margin = egui::Margin { left: 20, ..Default::default() };
    let response = egui::Frame::new().inner_margin(margin).show(ui, add_contents);
    let response_rect = response.response.rect;
    let line_start = response_rect.min;
    let line_end = line_start + egui::Vec2 { x: 0.0, y: response_rect.height() };
    ui.painter().line_segment([line_start, line_end], ui.visuals().window_stroke);
    response
}

pub struct TaskChannel<T>{
    sender: Sender<T>,
    receiver: Receiver<T>
}

impl<T> TaskChannel<T>{
    pub fn sender(&self) -> &Sender<T>{
        &self.sender
    }
    pub fn receiver(&self) -> &Receiver<T>{
        &self.receiver
    }
}

impl<T> Default for TaskChannel<T>{
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self{sender, receiver}
    }
}


pub struct GenCell<T>{
    timestamp: Instant,
    data: T,
}

impl<T> GenCell<T>{
    pub fn new(data: T) -> Self{
        Self{timestamp: Instant::now(), data }
    }
    pub fn maybe_set(&mut self, timestamp: Instant, value: T){
        if timestamp > self.timestamp {
            self.data = value
        }
    }
}

impl<T> Deref for GenCell<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct Arrow{
    pub origin: egui::Pos2,
    pub target: egui::Pos2,
    pub color: egui::Color32,
    pub tip_angle_from_shaft: f32,
    pub tip_side_length: f32,
}

impl Arrow{
    pub fn new(origin: egui::Pos2, target: egui::Pos2) -> Self{
        Self{
            origin,
            target,
            color: egui::Color32::BLACK,
            tip_angle_from_shaft: std::f32::consts::PI / 9.0,
            tip_side_length: 10.0,
        }
    }
    pub fn color(mut self, color: egui::Color32) -> Self{
        self.color = color;
        self
    }
}

impl Arrow{
    pub fn draw(self, ui: &mut egui::Ui) {
        let arrow_dir = (self.target - self.origin).normalized();
        let reverse_arrow_dir = -arrow_dir;

        let rot = egui::emath::Rot2::from_angle(self.tip_angle_from_shaft);
        let tip_left_pt = self.target + (rot * reverse_arrow_dir * self.tip_side_length);
        let tip_right_pt = self.target + (rot.inverse() * reverse_arrow_dir * self.tip_side_length);

        let tip = egui::epaint::PathShape{
            points: vec![self.target, tip_left_pt, tip_right_pt],
            closed: true,
            fill: self.color,
            stroke: egui::Stroke{color: self.color, width: 2.0}.into(),
        };

        ui.painter().line_segment(
            [self.origin, self.target],
            egui::Stroke{color: self.color, width: 2.0},
        );
        ui.painter().add(egui::Shape::Path(tip));
    }
}

