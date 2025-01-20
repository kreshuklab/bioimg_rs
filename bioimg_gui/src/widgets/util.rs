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
    let margin = egui::Margin { left: 20.0, ..Default::default() };
    let response = egui::Frame::none().inner_margin(margin).show(ui, add_contents);
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
