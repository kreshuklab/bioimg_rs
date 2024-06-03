use std::fmt::Display;

use super::{StatefulWidget, ValueWidget};

pub struct EnumWidget<E> {
    pub value: E,
    search: String,
    lower_case_display_names: Vec<String>,
}

impl<E> EnumWidget<E>{
    pub fn new(value: E) -> Self
    where
        E: strum::VariantNames
    {
        Self {
            value,
            search: String::with_capacity(64),
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
        }

        let vert_space_above_button = button_min.y;
        let vert_space_under_button = ui.ctx().screen_rect().max.y - button_max.y;

        let above_or_below = if vert_space_under_button > vert_space_above_button {
            egui::AboveOrBelow::Below
        } else {
            egui::AboveOrBelow::Above
        };
        egui::popup::popup_above_or_below_widget(ui, popup_id, &button_response, above_or_below, |ui| {
            ui.set_min_width(200.0); // if you want to control the size
            ui.vertical(|ui|{
                let header_rect = ui.vertical(|ui|{
                    ui.horizontal(|ui| {
                        ui.label("🔎 ");
                        let search_resp = ui.text_edit_singleline(&mut self.search);
                        search_resp.request_focus();
                    });
                    ui.add_space(10.0);
                }).response.rect;
                let header_height = header_rect.max.y - header_rect.min.y;

                let lower_search = self.search.to_lowercase();
                let scroll_area = egui::ScrollArea::vertical()
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .max_height(vert_space_above_button.max(vert_space_under_button) - header_height);
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
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.value.clone()
    }
}
