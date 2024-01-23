use std::fmt::Display;

use bioimg_spec::rdf::bounded_string::BoundedString;

use crate::widgets::{author_widget::StagingAuthor2, cite_widget::StagingCiteEntry2, cover_image_widget::CoverImageWidget, DrawAndParse, InputLines, StagingString, StagingVec};



pub struct TemplateApp {
    staging_name: StagingString<BoundedString<1, 127>>,
    staging_description: StagingString<BoundedString<1, 1023>>,
    cover_image: StagingVec<CoverImageWidget>,
    // id?
    staging_authors: StagingVec<StagingAuthor2>,
    //attachments
    staging_citations: StagingVec<StagingCiteEntry2>,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            staging_name: StagingString::new(InputLines::SingleLine),
            staging_description: StagingString::new(InputLines::Multiline),
            cover_image: Default::default(),
            staging_authors: Default::default(),
            staging_citations: StagingVec::default(),
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

                ui.strong("Description: ");
                let name_result = self.staging_description.draw_and_parse(ui, egui::Id::from("Name"));
                show_if_error(ui, &name_result);
                ui.end_row();

                ui.strong("Cover Images: ");
                let cover_img_result = self.cover_image.draw_and_parse(ui, egui::Id::from("Cover Images"));
                show_if_error(ui, &cover_img_result);
                ui.end_row();

                ui.strong("Authors: ");
                let authors_result = self.staging_authors.draw_and_parse(ui, egui::Id::from("Authors"));
                show_if_error(ui, &authors_result);
                ui.end_row();

                ui.strong("Cite: ");
                let citations_result = self.staging_citations.draw_and_parse(ui, egui::Id::from("Cite"));
                show_if_error(ui, &citations_result);
                ui.end_row();
            }).inner;
        });
    }
}
