use crate::result::Result;

pub trait Iconify{
    fn iconify(&self) -> Result<egui::WidgetText>;
}
