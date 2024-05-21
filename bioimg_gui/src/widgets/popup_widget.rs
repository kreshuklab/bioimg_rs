#[derive(Default)]
pub struct FullScreenPopupWidget{
    pub is_open: bool,
}

pub enum PopupResult<T>{
    Continued,
    Closed,
    Finished(T),
}

impl FullScreenPopupWidget{
    pub fn draw<R>(
        &mut self,
        ui: &mut egui::Ui,
        id: egui::Id,
        heading: &str,
        add_elements: impl FnOnce(&mut egui::Ui, egui::Id) -> PopupResult<R>
    ) -> PopupResult<R>{
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.is_open = false;
        }
        if !self.is_open{
            return PopupResult::Closed
        }
        let area = egui::containers::Area::new(id.with("full screen popup"))
            .movable(false)
            .order(egui::Order::Foreground)
            .constrain(true)
            .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO);
        area.show(ui.ctx(), |ui| {
            egui::Frame::popup(&ui.ctx().style()).show(ui, |ui| {
                ui.vertical(|ui|{
                    ui.horizontal(|ui| {
                        ui.heading(heading);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui.button("🗙").clicked() {
                                self.is_open = false;
                            }
                        });
                    });
                    let res = add_elements(ui, id.with("full screen popup add contents".as_ptr()));
                    ui.allocate_space(egui::Vec2{x: ui.available_width(), y: ui.available_height()});
                    if matches!(&res, PopupResult::Closed | PopupResult::Finished(_)){
                        self.is_open = false
                    }
                    res
                }).inner
            }).inner
        }).inner
    }
}
