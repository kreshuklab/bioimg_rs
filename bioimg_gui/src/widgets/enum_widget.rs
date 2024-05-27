use std::fmt::Display;

use super::{popup_widget::{draw_fullscreen_popup, PopupResult}, StatefulWidget, ValueWidget};

pub struct EnumWidget<E> {
    pub value: E,
    search: String,
    lower_case_display_names: Vec<String>,
    first_frame: bool,
}

impl<E> EnumWidget<E>{
    pub fn new(value: E) -> Self
    where
        E: strum::VariantNames
    {
        Self {
            value,
            search: String::with_capacity(64),
            first_frame: true,
            lower_case_display_names: <E as strum::VariantNames>::VARIANTS.iter().map(|dn| dn.to_lowercase()).collect(),
        }
    }

}

impl<E> ValueWidget for EnumWidget<E>{
    type Value<'v> = E;
    fn set_value(&mut self, value: E){
        self.value = value
    }
}

impl<E> Default for EnumWidget<E>
where
    E: Default + strum::VariantNames
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<E> StatefulWidget for EnumWidget<E>
where
    E: strum::VariantArray + strum::VariantNames + Display + Clone
{
    type Value<'p> = E where E: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        let popup_id = id;
        let button_response = ui.button(&self.value.to_string());
        let button_min = button_response.rect.min;
        let button_max = button_response.rect.max;
        if button_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
            if ui.memory(|mem| mem.is_popup_open(popup_id)){
                self.first_frame = true;
            }
        }
        if !ui.memory(|mem| mem.is_popup_open(popup_id)){
            return;
        }
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
            return
        }

        let mut area = egui::containers::Area::new(popup_id)
            .movable(false)
            .order(egui::Order::Foreground)
            .constrain(true)
            .movable(false);

        let vert_space_above_button = button_min.y;
        let vert_space_under_button = ui.ctx().screen_rect().max.y - button_max.y;

        if vert_space_under_button > vert_space_above_button {
            area = area.fixed_pos(egui::Pos2{ x: button_min.x, y: button_max.y });
        } else {
            area = area.anchor(
                egui::Align2::LEFT_BOTTOM,
                egui::Vec2{
                    x: button_response.rect.min.x,
                    y: - (ui.ctx().screen_rect().max.y - button_response.rect.min.y),
                },
            );
        };
        let area_resp = area.show(ui.ctx(), |ui| egui::Frame::popup(&ui.ctx().style())
            .shadow(egui::epaint::Shadow::NONE)
            .rounding(egui::Rounding::default())
            .outer_margin(0.0)
            .show(ui, |ui| {
                ui.vertical(|ui|{
                    ui.horizontal(|ui| {
                        ui.label("🔎 ");
                        let search_resp = ui.text_edit_singleline(&mut self.search);
                        if self.first_frame{
                            search_resp.request_focus();
                        }
                    });
                    // ui.separator();
                    ui.add_space(10.0);

                    let lower_search = self.search.to_lowercase();
                    let scroll_area = egui::ScrollArea::vertical().max_height(100.0).scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible);
                    scroll_area.show(ui, |ui| {
                        self.lower_case_display_names
                            .iter()
                            .enumerate()
                            .filter(|(_, lower_variant_name)| lower_variant_name.contains(&lower_search))
                            .for_each(|(idx, _)| {
                                if ui.button(<E as strum::VariantNames>::VARIANTS[idx]).clicked() {
                                    self.value = <E as strum::VariantArray>::VARIANTS[idx].clone();
                                    ui.memory_mut(|mem| mem.toggle_popup(popup_id))
                                }
                            });
                    });
                });
            }));
        if !self.first_frame && area_resp.response.clicked_elsewhere(){
            ui.ctx().memory_mut(|mem| mem.close_popup());
        }
        if !ui.ctx().memory(|mem| mem.is_popup_open(popup_id)){
            self.first_frame = true;
        }
        self.first_frame = false;
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.value.clone()
    }
}
