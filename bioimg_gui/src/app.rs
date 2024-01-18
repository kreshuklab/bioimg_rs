use std::fmt::Display;

use bioimg_spec::rdf::bounded_string::BoundedString;

use crate::widgets::{DrawAndParse, StagingString, InputLines, StagingVec, StagingOpt, author_widget::StagingAuthor2, file_widget::FileWidget};



pub struct TemplateApp {
    staging_name: StagingString<BoundedString<1, 127>>,
    test_file_vec: StagingVec<FileWidget>,
    test_opt: StagingOpt<StagingString<BoundedString<1, 127>>>,
    // staging_description: StagingString<BoundedString<1, 1023>>,
    staging_authors: StagingOpt<StagingVec<StagingAuthor2>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            staging_name: StagingString::new(InputLines::SingleLine),
            test_file_vec: Default::default(),
            test_opt: StagingOpt::new(),
            staging_authors: StagingOpt::default(),
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
            fn show_error(ui: &mut egui::Ui, message: impl Display){
                ui.label(egui::RichText::new(message.to_string()).color(egui::Color32::RED));
            }
            fn show_if_error<T, E: Display>(ui: &mut egui::Ui, result: &Result<T, E>){
                if let Err(ref err) = result{
                    show_error(ui, err)
                }
            }

            egui::Grid::new("app").num_columns(2).striped(true).show(ui, |ui|{
                ui.strong("Name: ");
                let name_result = self.staging_name.draw_and_parse(ui, egui::Id::from("Name"));
                show_if_error(ui, &name_result);
                ui.end_row();

                ui.strong("Pick some images: ");
                let _images = self.test_file_vec.draw_and_parse(ui, egui::Id::from("pick some images"));
                ui.end_row();

                ui.strong("Bla: ");
                let test_opt_result = self.test_opt.draw_and_parse(ui, egui::Id::from("Bla"));
                show_if_error(ui, &test_opt_result);
                ui.end_row();

                ui.strong("Test vec: ");
                let _test_auth_vec_result = ui.horizontal_top(|ui|{
                    self.staging_authors.draw_and_parse(ui, egui::Id::from("test_vec"))
                }).inner;
                ui.end_row();

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
