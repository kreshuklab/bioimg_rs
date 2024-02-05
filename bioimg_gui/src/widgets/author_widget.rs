use bioimg_spec::rdf::{
    author::Author2,
    bounded_string::{BoundedString, BoundedStringParsingError},
    orcid::{Orcid, OrcidParsingError},
};

use super::{StagingOpt, StagingString, StatefulWidget};

pub type ConfString = BoundedString<1, 1023>;

#[derive(thiserror::Error, Debug)]
pub enum Author2ParsingError {
    #[error("Missing field: {field_name}")]
    MissingField { field_name: String },
    #[error("{0}")]
    FieldError(
        #[from]
        #[source]
        BoundedStringParsingError,
    ),
    #[error("{0}")]
    BadOrcid(
        #[from]
        #[source]
        OrcidParsingError,
    ),
}

pub struct StagingAuthor2 {
    staging_name: StagingString<ConfString>,                    // (Name→String) Full name.
    staging_affiliation: StagingOpt<StagingString<ConfString>>, // (String) Affiliation.
    staging_email: StagingOpt<StagingString<ConfString>>,       // FIXME: make a parser here (Email) E-Mail
    staging_github_user: StagingOpt<StagingString<ConfString>>, // (String) GitHub user name.
    staging_orcid: StagingOpt<StagingString<Orcid>>,
    parsed: Result<Author2, Author2ParsingError>,
}

impl Default for StagingAuthor2 {
    fn default() -> Self {
        Self {
            staging_name: Default::default(),
            staging_affiliation: Default::default(),
            staging_email: Default::default(),
            staging_github_user: Default::default(),
            staging_orcid: Default::default(),
            parsed: Err(Author2ParsingError::MissingField {
                field_name: "Name".to_owned(),
            }), //FIXME: what?
        }
    }
}

impl StagingAuthor2 {
    fn do_draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Author2, Author2ParsingError> {
        egui::Grid::new(id)
            .num_columns(2)
            .show(ui, |ui| {
                ui.strong("Name: ");
                self.staging_name.draw_and_parse(ui, id.with("Name"));
                ui.end_row();

                ui.strong("Affiliation: ");
                self.staging_affiliation.draw_and_parse(ui, id.with("Affiliation"));
                ui.end_row();

                ui.strong("Email: ");
                self.staging_email.draw_and_parse(ui, id.with("Email"));
                ui.end_row();

                ui.strong("Github User: ");
                self.staging_github_user.draw_and_parse(ui, id.with("Github User"));
                ui.end_row();

                ui.strong("Orcid: ");
                self.staging_orcid.draw_and_parse(ui, id.with("Orcid"));
                ui.end_row();

                Ok(Author2 {
                    name: self.staging_name.state().clone()?,
                    affiliation: self.staging_affiliation.state().transpose()?,
                    email: self.staging_email.state().transpose()?,
                    github_user: self.staging_github_user.state().transpose()?,
                    orcid: self.staging_orcid.state().transpose()?,
                })
            })
            .inner
    }
}

impl StatefulWidget for StagingAuthor2 {
    type Value<'p> = &'p Result<Author2, Author2ParsingError>;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.parsed = self.do_draw_and_parse(ui, id)
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}
