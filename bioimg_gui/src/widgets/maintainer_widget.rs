use bioimg_spec::rdf::{bounded_string::BoundedString, maintainer::Maintainer, orcid::Orcid, slashless_string::SlashlessString};

use super::{StagingOpt, StagingString, StatefulWidget};

pub struct StagingMaintainer {
    github_user: StagingString<BoundedString<1, 1023>>, //FIXME validate this somehow}
    affiliation: StagingOpt<StagingString<BoundedString<1, 1023>>>,
    email: StagingOpt<StagingString<BoundedString<1, 1023>>>, //FIXME
    orcid: StagingOpt<StagingString<Orcid>>,
    name: StagingOpt<StagingString<SlashlessString<1, 1023>>>,
    parsed: anyhow::Result<Maintainer>,
}

impl Default for StagingMaintainer {
    fn default() -> Self {
        Self {
            github_user: Default::default(),
            affiliation: Default::default(),
            email: Default::default(),
            orcid: Default::default(),
            name: Default::default(),
            parsed: Err(anyhow::anyhow!("Empty")),
        }
    }
}

impl StagingMaintainer {
    fn do_render_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) -> anyhow::Result<Maintainer> {
        egui::Grid::new(id)
            .num_columns(2)
            .show(ui, |ui| {
                ui.strong("Github User: ");
                self.github_user.draw_and_parse(ui, id.with("github_user"));
                ui.end_row();

                ui.strong("Affiliation: ");
                self.affiliation.draw_and_parse(ui, id.with("affiliation"));
                ui.end_row();

                ui.strong("Email: ");
                self.email.draw_and_parse(ui, id.with("email"));
                ui.end_row();

                ui.strong("Orcid: ");
                self.orcid.draw_and_parse(ui, id.with("orcid"));
                ui.end_row();

                ui.strong("Name: ");
                self.name.draw_and_parse(ui, id.with("name"));
                ui.end_row();

                Ok(Maintainer {
                    github_user: self.github_user.state()?,
                    name: self.name.state().transpose()?,
                    affiliation: self.affiliation.state().transpose()?,
                    email: self.email.state().transpose()?,
                    orcid: self.orcid.state().transpose()?,
                })
            })
            .inner
    }
}

impl StatefulWidget for StagingMaintainer {
    type Value<'p> = &'p anyhow::Result<Maintainer>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.parsed = self.do_render_and_parse(ui, id);
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}
