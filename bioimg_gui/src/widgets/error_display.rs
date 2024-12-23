use std::fmt::Display;

pub fn show_error(ui: &mut egui::Ui, message: impl Display){
    ui.label(egui::RichText::new(message.to_string()).color(egui::Color32::RED));
}
pub fn show_warning(ui: &mut egui::Ui, message: impl Display){
    ui.label(egui::RichText::new(message.to_string()).color(egui::Color32::YELLOW));
}
pub fn show_if_error<T, E: Display>(ui: &mut egui::Ui, result: &Result<T, E>){
    if let Err(ref err) = result{
        show_error(ui, err)
    }
}
