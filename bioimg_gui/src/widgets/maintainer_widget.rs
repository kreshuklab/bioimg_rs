use bioimg_spec::rdf::{bounded_string::BoundedString, maintainer::Maintainer, orcid::Orcid, slashless_string::SlashlessString};

use super::{error_display::show_if_error, StagingOpt, StagingString, StatefulWidget};

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
        ui.vertical(|ui| {
            let github_user = ui
                .horizontal(|ui| {
                    ui.strong("Github User: ");
                    self.github_user.draw_and_parse(ui, id.with("github_user"));
                    let github_user = self.github_user.state();
                    github_user
                })
                .inner;

            let affiliation = ui
                .horizontal(|ui| {
                    ui.strong("Affiliation: ");
                    self.affiliation.draw_and_parse(ui, id.with("affiliation"));
                    let affiliation = self.affiliation.state();
                    affiliation
                })
                .inner;

            let email = ui
                .horizontal(|ui| {
                    ui.strong("Email: ");
                    self.email.draw_and_parse(ui, id.with("email"));
                    let email = self.email.state();
                    email
                })
                .inner;

            let orcid = ui
                .horizontal(|ui| {
                    ui.strong("Orcid: ");
                    self.orcid.draw_and_parse(ui, id.with("orcid"));
                    let orcid = self.orcid.state();
                    orcid
                })
                .inner;

            let name = ui
                .horizontal(|ui| {
                    ui.strong("Name: ");
                    self.name.draw_and_parse(ui, id.with("name"));
                    let name = self.name.state();
                    name
                })
                .inner;

            Ok(Maintainer {
                github_user: github_user?,
                name: name.transpose()?,
                affiliation: affiliation.transpose()?,
                email: email.transpose()?,
                orcid: orcid.transpose()?,
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
