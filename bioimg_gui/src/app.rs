use bioimg_spec::rdf::bounded_string::BoundedString;

use crate::widgets::{
    author_widget::StagingAuthor2, cite_widget::StagingCiteEntry2, cover_image_widget::CoverImageWidget, error_display::show_if_error, url_widget::StagingUrl, InputLines, StagingOpt, StagingString, StagingVec, StatefulWidget
};

pub struct TemplateApp {
    staging_name: StagingString<BoundedString<1, 127>>,
    staging_description: StagingString<BoundedString<1, 1023>>,
    cover_image: StagingVec<CoverImageWidget>,
    // id?
    staging_authors: StagingVec<StagingAuthor2>,
    //attachments
    staging_citations: StagingVec<StagingCiteEntry2>,
    //config
    staging_git_repo: StagingOpt<StagingUrl>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            staging_name: StagingString::new(InputLines::SingleLine),
            staging_description: StagingString::new(InputLines::Multiline),
            cover_image: StagingVec::new("Cover Image"),
            staging_authors: StagingVec::new("Author"),
            staging_citations: StagingVec::new("Cite"),
            staging_git_repo: Default::default(),
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
            egui::Grid::new("app").num_columns(2).striped(true).show(ui, |ui|{
                ui.strong("Name: ");
                self.staging_name.draw_and_parse(ui, egui::Id::from("Name"));
                let name_result = self.staging_name.state();
                show_if_error(ui, &name_result);
                ui.end_row();

                ui.strong("Description: ");
                self.staging_description.draw_and_parse(ui, egui::Id::from("Name"));
                let name_result = self.staging_description.state();
                show_if_error(ui, &name_result);
                ui.end_row();

                ui.strong("Cover Images: ");
                self.cover_image.draw_and_parse(ui, egui::Id::from("Cover Images"));
                let cover_img_results = self.cover_image.state();
                ui.end_row();

                ui.strong("Authors: ");
                self.staging_authors.draw_and_parse(ui, egui::Id::from("Authors"));
                let author_results = self.staging_authors.state();
                ui.end_row();

                ui.strong("Cite: ");
                self.staging_citations.draw_and_parse(ui, egui::Id::from("Cite"));
                let citation_results = self.staging_citations.state();
                ui.end_row();

                ui.strong("Git Repo: ");
                self.staging_git_repo.draw_and_parse(ui, egui::Id::from("Git Repo"));
                let git_repo_result = self.staging_git_repo.state();
                ui.end_row();
            }).inner;
        });
    }
}
