use bioimg_spec::rdf::bounded_string::BoundedString;

use crate::widgets::{author_widget::StagingAuthor2, /*StagingVec,*/ DrawAndParse, StagingOpt/*, StagingString*/};



pub struct TemplateApp {
    // staging_name: StagingString<BoundedString<1, 127>>,
    // staging_description: StagingString<BoundedString<1, 1023>>,
    // staging_authors: StagingOpt<StagingVec<StagingAuthor2>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // staging_name: Default::default(),
            // staging_description: StagingString::multiline(),
            // staging_authors: StagingOpt::default(),
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
            ui.heading("Bioimage model description");

            egui::Grid::new("app").num_columns(2).striped(true).show(ui, |ui|{
                // ui.strong("Name: ");
                // let _name_result = self.staging_name.draw_and_parse(ui, egui::Id::from("Name"));
                // ui.end_row();

                // ui.strong("Description: ");
                // let _description_result = self.staging_description.draw_and_parse(ui, egui::Id::from("Description"));
                // ui.end_row();

                // ui.strong("Authors: ");
                // let _authors_result = self.staging_authors.draw_and_parse(ui, egui::Id::from("Authors"));
                // ui.end_row();

                // ui.end_row();
            }).inner;
        });
    }
}
