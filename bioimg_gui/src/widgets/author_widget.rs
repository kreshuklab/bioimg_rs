use bioimg_spec::rdf::{author::Author2, bounded_string::{BoundedString, BoundedStringParsingError}, orcid::{Orcid, OrcidParsingError}};

use super::{error_display::show_if_error, DrawAndParse, StagingOpt, StagingString};

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
    type Value<'p> = Result<Author2, Author2ParsingError>;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) -> Self::Value<'p>{
        ui.scope(|ui|{
            egui::Grid::new(id).show(ui, |ui| {
                ui.strong("Name: ");
                let name_res = self.staging_name.draw_and_parse(ui, id.with("Name"));
                show_if_error(ui, &name_res);
                ui.end_row();

                ui.strong("Affiliation: ");
                let affiliation_res = self.staging_affiliation.draw_and_parse(ui, id.with("Affiliation"));
                if let Some(res) = &affiliation_res{
                    show_if_error(ui, res);
                }
                ui.end_row();

                ui.strong("Email: ");
                let email_res = self.staging_email.draw_and_parse(ui, id.with("Email"));
                if let Some(res) = &email_res{
                    show_if_error(ui, res);
                }
                ui.end_row();

                ui.strong("Github User: ");
                let github_user_res = self.staging_github_user.draw_and_parse(ui, id.with("Github User"));
                if let Some(res) = &github_user_res{
                    show_if_error(ui, res);
                }
                ui.end_row();

                ui.strong("Orcid: ");
                let orcid_res = self.staging_orcid.draw_and_parse(ui, id.with("Orcid"));
                if let Some(res) = &orcid_res{
                    show_if_error(ui, res);
                }
                ui.end_row();

                Ok(Author2{
                    name: name_res?,
                    affiliation: affiliation_res.transpose()?,
                    email: email_res.transpose()?,
                    github_user: github_user_res.transpose()?,
                    orcid: orcid_res.transpose()?,
                })
            }).inner
        }).inner
    }
}
