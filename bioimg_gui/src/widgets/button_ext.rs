pub trait ButtonExt{
    fn draw_as_label(self, ui: &mut egui::Ui) -> egui::Response;
}

impl ButtonExt for egui::Button<'_>{
    fn draw_as_label(self, ui: &mut egui::Ui) -> egui::Response{
        let resp = ui.add(self.frame(false));
        if resp.hovered(){
            ui.ctx().output_mut(|out| out.cursor_icon = egui::CursorIcon::PointingHand);
        }
        resp
    }
}
