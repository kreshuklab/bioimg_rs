use bioimg_spec::rdf::bounded_string::BoundedString;

use crate::widgets::{author_widget::StagingAuthor2, StagingVec, DrawAndParse, StagingOpt, StagingString};



pub struct TemplateApp {
    staging_name: StagingString<BoundedString<1, 127>>,
    staging_authors: StagingOpt<StagingVec<StagingAuthor2>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            staging_name: Default::default(),
            staging_authors: StagingOpt::default(),
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            // egui::Grid::new("app")
            //     .num_columns(2)
            //     .with_row_color(|idx, _| {
            //         if idx % 2 == 0{
            //             Some(egui::Color32::RED)
            //         }else{
            //             Some(egui::Color32::GREEN)
            //         }
            //     })
            //     .show(ui, |ui|{
            //         ui.label("Outer label 1"); _ = ui.button("outer button 1");
            //         ui.end_row();

            //         ui.label("Outer label 2");
            //         ui.horizontal(|ui|{

            //         });
            //         egui::Grid::new("inner grid")
            //             .num_columns(3)
            //             .show(ui, |ui|{
            //                 ui.label("Inner label 1"); ui.label("Inner label 3"); ui.label("Inner label 2");
            //                 ui.end_row();
            //                 ui.label("Inner label 4"); ui.label("Inner label 5"); ui.label("Inner label 6");
            //                 ui.end_row();
            //                 ui.label("Inner label 7"); ui.label("Inner label 8"); ui.label("Inner label 9");
            //                 ui.end_row();
            //             });
            //         ui.end_row();

            //         ui.label("Outer label 3"); _ = ui.button("outer button 3");
            //         ui.end_row();
            //     })


            egui::Grid::new("app").num_columns(2).striped(true).show(ui, |ui|{
                ui.strong("Name: ");
                let _name_result = self.staging_name.draw_and_parse(ui, egui::Id::from("Name"));
                ui.end_row();

                ui.strong("Authors: ");
                let _authors_result = self.staging_authors.draw_and_parse(ui, egui::Id::from("Authors"));
                ui.end_row();

                ui.end_row();
            }).inner;
        });
    }
}
