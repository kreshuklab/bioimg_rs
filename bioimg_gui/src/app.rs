use crate::widgets::{author_widget::StagingAuthor2, StagingVec, DrawAndParse, StagingOpt};



/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct TemplateApp {
    staging_authors: StagingOpt<StagingVec<StagingAuthor2>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            staging_authors: StagingOpt::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            let authors_result = egui::Grid::new("app").show(ui, |ui|{
                ui.strong("Authors: ");
                let authors_result = self.staging_authors.draw_and_parse(ui, egui::Id::from("authors"));
                ui.end_row();
                authors_result
            }).inner;

            match authors_result{
                Err(err) => ui.label(
                    egui::RichText::new(
                        format!("Bad authors: {err}")
                    ).color(egui::Color32::from_rgb(110, 0, 0))
                ),
                Ok(_) => ui.label(egui::RichText::new("Authors validate!").color(egui::Color32::from_rgb(0, 110, 0))),
            };

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
