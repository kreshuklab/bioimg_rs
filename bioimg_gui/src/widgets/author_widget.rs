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

#[derive(Default, Clone)]
pub struct StagingAuthor2{
    staging_name: StagingString< ConfString >,                // (Name→String) Full name.
    staging_affiliation: StagingOpt<StagingString< ConfString >>, // (String) Affiliation.
    staging_email: StagingOpt<StagingString<ConfString>>,       // FIXME: make a parser here (Email) E-Mail
    staging_github_user: StagingOpt<StagingString<ConfString>>, // (String) GitHub user name.
    staging_orcid: StagingOpt<StagingString<Orcid>>,
}

impl DrawAndParse for StagingAuthor2{
    type Error = Author2ParsingError;
    type Parsed = Author2;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui) -> Result<Author2, Author2ParsingError>{
        let name = ui.horizontal(|ui|{
            ui.label("Name");
            self.staging_name.draw_and_parse(ui)
        }).inner;
        let affiliation = ui.horizontal(|ui|{
            ui.label("Affiliation");
            self.staging_affiliation.draw_and_parse(ui)
        }).inner;
        let email = ui.horizontal(|ui|{
            ui.label("Email");
            self.staging_email.draw_and_parse(ui)
        }).inner;
        let github_user = ui.horizontal(|ui|{
            ui.label("Github User");
            self.staging_github_user.draw_and_parse(ui)
        }).inner;
        let orcid = ui.horizontal(|ui|{
            ui.label("Orcid");
            self.staging_orcid.draw_and_parse(ui)
        }).inner;

        Ok(Author2{
            name: name?,
            affiliation: affiliation?,
            email: email?,
            github_user: github_user?,
            orcid: orcid?,
        })
    }
}
