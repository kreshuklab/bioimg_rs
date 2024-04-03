use bioimg_spec::rdf::{author::Author2, bounded_string::BoundedString, orcid::Orcid};

use super::{staging_opt::StagingOpt, staging_string::StagingString, staging_vec::ItemWidgetConf, StatefulWidget};
use crate::result::Result;

pub type ConfString = BoundedString<1, 1023>;

pub struct StagingAuthor2 {
    pub staging_name: StagingString<ConfString>,                    // (Name→String) Full name.
    pub staging_affiliation: StagingOpt<StagingString<ConfString>>, // (String) Affiliation.
    pub staging_email: StagingOpt<StagingString<ConfString>>,       // FIXME: make a parser here (Email) E-Mail
    pub staging_github_user: StagingOpt<StagingString<ConfString>>, // (String) GitHub user name.
    pub staging_orcid: StagingOpt<StagingString<Orcid>>,
}

impl ItemWidgetConf for StagingAuthor2{
    const ITEM_NAME: &'static str = "Author";
    const MIN_NUM_ITEMS: usize = 1;
}

impl Default for StagingAuthor2 {
    fn default() -> Self {
        Self {
            staging_name: Default::default(),
            staging_affiliation: Default::default(),
            staging_email: Default::default(),
            staging_github_user: Default::default(),
            staging_orcid: Default::default(),
        }
    }
}

impl StatefulWidget for StagingAuthor2 {
    type Value<'p> = Result<Author2>;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        egui::Grid::new(id).num_columns(2).show(ui, |ui| {
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
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(Author2 {
            name: self.staging_name.state().to_owned()?,
            affiliation: self.staging_affiliation.state().transpose().to_owned()?,
            email: self.staging_email.state().transpose().to_owned()?,
            github_user: self.staging_github_user.state().transpose().to_owned()?,
            orcid: self.staging_orcid.state().transpose().to_owned()?,
        })
    }
}
