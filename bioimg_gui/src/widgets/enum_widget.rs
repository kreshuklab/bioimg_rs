use std::fmt::Display;

use super::{popup_widget::{FullScreenPopupWidget, PopupResult}, StatefulWidget, ValueWidget};

pub struct EnumWidget<E> {
    pub value: E,
    search: String,
    lower_case_display_names: Vec<String>,
    popup_widget: FullScreenPopupWidget,
}

impl<E> EnumWidget<E>{
    pub fn new(value: E) -> Self
    where
        E: strum::VariantNames
    {
        Self {
            value,
            search: String::with_capacity(64),
            popup_widget: Default::default(),
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
        if ui.button(&self.value.to_string()).clicked() {
            self.popup_widget.is_open = !self.popup_widget.is_open;
        }
        let new_value = self.popup_widget.draw(ui, id.with("pick variant".as_ptr()), "Pick One", |ui, _| {
            ui.horizontal(|ui| {
                ui.label("🔎 ");
                ui.text_edit_singleline(&mut self.search);
            });
            ui.separator();
            ui.add_space(10.0);

            let lower_search = self.search.to_lowercase();
            let mut out: PopupResult<E> = PopupResult::Continued;
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.lower_case_display_names
                    .iter()
                    .enumerate()
                    .filter(|(_, lower_variant_name)| lower_variant_name.contains(&lower_search))
                    .for_each(|(idx, _)| {
                        if ui.button(<E as strum::VariantNames>::VARIANTS[idx]).clicked() {
                            out = PopupResult::Finished(<E as strum::VariantArray>::VARIANTS[idx].clone());
                        }
                    });
            });
            out
        });
        match new_value{
            PopupResult::Finished(value) => {
                self.search.clear();
                self.value = value;
            },
            PopupResult::Closed => self.search.clear(),
            PopupResult::Continued => (),
        }
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.value.clone()
    }
}
