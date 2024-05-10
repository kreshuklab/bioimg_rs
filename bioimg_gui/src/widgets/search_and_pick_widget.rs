use std::fmt::Display;

use super::{StatefulWidget, ValueWidget};

pub struct SearchableEntry<T>{
    lowercase_display: String,
    display: String,
    value: T,
}

pub struct SearchAndPickWidget<T> {
    pub value: T,
    pub search: String,
    pub popup_open: bool,
    pub entries: Vec<SearchableEntry<T>>,
}

impl<T: PartialEq> SearchAndPickWidget<T>{
    pub fn contains(&self, value: &T) -> bool{
        self.entries.iter().find(|entry| entry.value == *value).is_some()
    }
}

impl<T: Display> SearchAndPickWidget<T>{
    pub fn new(value: T, entries: Vec<T>) -> Self{
        Self{
            value,
            search: String::with_capacity(64),
            popup_open: false,
            entries: entries.into_iter()
                .map(|e| SearchableEntry{
                    lowercase_display: e.to_string().to_lowercase(),
                    display: e.to_string(),
                    value: e,
                })
                .collect(),
        }
    }
}


impl<T> ValueWidget for SearchAndPickWidget<T>{
    type Value<'v> = T;
    fn set_value(&mut self, value: T){
        self.value = value
    }
}

impl<T> StatefulWidget for SearchAndPickWidget<T>
where
    T: Display + Clone
{
    type Value<'p> = T where T: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        if ui.button(&self.value.to_string()).clicked() {
            self.popup_open = !self.popup_open;
        }
        if !self.popup_open {
            return;
        }
        egui::containers::Area::new(id.with("Enum Popup"))
            .movable(false)
            .order(egui::Order::Foreground)
            .constrain(true)
            .show(ui.ctx(), |ui| {
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.popup_open = false;
                    self.search.clear();
                    return;
                }
                egui::Frame::popup(&ui.ctx().style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Pick one");
                        if ui.button("🗙").clicked() {
                            self.popup_open = false;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("🔎 ");
                        ui.text_edit_singleline(&mut self.search);
                    });
                    ui.separator();
                    ui.add_space(10.0);

                    let lower_search = self.search.to_lowercase();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.entries
                            .iter()
                            .filter(|entry| entry.lowercase_display.contains(&lower_search))
                            .for_each(|entry| {
                                if ui.button(&entry.display).clicked() {
                                    self.popup_open = false;
                                    self.value = entry.value.clone();
                                    self.search.clear();
                                }
                            });
                    })
                });
            });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.value.clone()
    }
}
