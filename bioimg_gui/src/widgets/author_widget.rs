use bioimg_spec::rdf::{author::Author2, bounded_string::{BoundedString, BoundedStringParsingError}, orcid::{Orcid, OrcidParsingError}};

use super::{StagingString, DrawAndParse, StagingOpt};

pub type ConfString = BoundedString<1, 1023>;

#[derive(thiserror::Error, Debug)]
pub enum Author2ParsingError{
    #[error("{0}")]
    FieldError(#[from] #[source] BoundedStringParsingError),
    #[error("{0}")]
    BadOrcid(#[from] #[source] OrcidParsingError),
}

#[derive(Default)]
pub struct StagingAuthor2{
    staging_name: StagingString< ConfString >,                // (Name→String) Full name.
    staging_affiliation: StagingOpt<StagingString< ConfString >>, // (String) Affiliation.
    staging_email: StagingOpt<StagingString<ConfString>>,       // FIXME: make a parser here (Email) E-Mail
    staging_github_user: StagingOpt<StagingString<ConfString>>, // (String) GitHub user name.
    staging_orcid: StagingOpt<StagingString<Orcid>>,
}

impl DrawAndParse for StagingAuthor2{
    type Parsed<'p> = Author2;
    type Error = Author2ParsingError;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Author2, Author2ParsingError>{
        ui.scope(|ui|{
            egui::Grid::new(id).show(ui, |ui| {
                ui.strong("Name: ");
                let name = self.staging_name.draw_and_parse(ui, id.with("Name"));
                ui.end_row();

                ui.strong("Affiliation: ");
                let affiliation = self.staging_affiliation.draw_and_parse(ui, id.with("Affiliation"));
                ui.end_row();

                ui.strong("Email: ");
                let email = self.staging_email.draw_and_parse(ui, id.with("Email"));
                ui.end_row();

                ui.strong("Github User: ");
                let github_user = self.staging_github_user.draw_and_parse(ui, id.with("Github User"));
                ui.end_row();

                ui.strong("Orcid: ");
                let orcid = self.staging_orcid.draw_and_parse(ui, id.with("Orcid"));
                ui.end_row();

                Ok(Author2{
                    name: name?,
                    affiliation: affiliation?,
                    email: email?,
                    github_user: github_user?,
                    orcid: orcid?,
                })
            }).inner
        }).inner
    }
}
