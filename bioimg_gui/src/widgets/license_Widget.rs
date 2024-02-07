use bioimg_spec::rdf;
use strum::VariantArray;

use super::StatefulWidget;

pub struct LicenseWidget {
    value: rdf::SpdxLicense,
    search: String,
    popup_open: bool,
    display_names: Vec<String>,
    lower_case_display_names: Vec<String>,
}

impl Default for LicenseWidget {
    fn default() -> Self {
        let display_names: Vec<String> = rdf::SpdxLicense::VARIANTS.iter().map(|v| v.to_string()).collect();
        Self {
            value: rdf::SpdxLicense::Apache_2_0,
            search: String::with_capacity(64),
            popup_open: false,
            lower_case_display_names: display_names.iter().map(|dn| dn.to_lowercase()).collect(),
            display_names,
        }
    }
}

impl StatefulWidget for LicenseWidget {
    type Value<'p> = rdf::SpdxLicense;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        if ui.button(&self.display_names[self.value as usize]).clicked() {
            self.popup_open = !self.popup_open;
        }
        if !self.popup_open {
            return;
        }
        egui::containers::Area::new(id.with("License Popup"))
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
                        ui.heading("Pick a license for this model");
                        if ui.button("🗙").clicked() {
                            self.popup_open = false;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Filter: ");
                        ui.text_edit_singleline(&mut self.search);
                    });
                    ui.separator();
                    ui.add_space(10.0);

                    let lower_search = self.search.to_lowercase();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.lower_case_display_names
                            .iter()
                            .enumerate()
                            .filter(|(_, lower_variant_name)| lower_variant_name.contains(&lower_search))
                            .for_each(|(idx, _)| {
                                if ui.button(&self.display_names[idx]).clicked() {
                                    self.popup_open = false;
                                    self.value = rdf::SpdxLicense::from_repr(idx).unwrap();
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
