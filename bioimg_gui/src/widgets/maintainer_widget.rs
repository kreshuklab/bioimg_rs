use bioimg_spec::rdf::{self, bounded_string::BoundedString, orcid::Orcid};

use super::{collapsible_widget::{CollapsibleWidget, SummarizableWidget}, staging_opt::StagingOpt, staging_string::StagingString, staging_vec::ItemWidgetConf, StatefulWidget, ValueWidget};
use crate::result::Result;

pub struct StagingMaintainer {
    pub github_user_widget: StagingString<BoundedString<1, 1023>>, //FIXME validate this somehow}
    pub affiliation_widget: StagingOpt<StagingString<BoundedString<1, 1023>>>,
    pub email_widget: StagingOpt<StagingString<BoundedString<1, 1023>>>, //FIXME
    pub orcid_widget: StagingOpt<StagingString<Orcid>>,
    pub name_widget: StagingOpt<StagingString<rdf::MaintainerName>>,
}

impl ValueWidget for StagingMaintainer{
    type Value<'a> = rdf::Maintainer;
    fn set_value<'a>(&mut self, value: Self::Value<'a>) {
        self.github_user_widget.set_value(value.github_user);
        self.affiliation_widget.set_value(value.affiliation);
        self.email_widget.set_value(value.email);
        self.orcid_widget.set_value(value.orcid);
        self.name_widget.set_value(value.name);
    }
}

impl ItemWidgetConf for StagingMaintainer{
    const ITEM_NAME: &'static str = "Maintainer";
}

impl ItemWidgetConf for CollapsibleWidget<StagingMaintainer>{
    const ITEM_NAME: &'static str = "Maintainer";
    const GROUP_FRAME: bool = false;
}

impl SummarizableWidget for StagingMaintainer{
    fn summarize(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        match self.state(){
            Ok(author) => {
                ui.label(author.to_string());
            },
            Err(err) => {
                let rich_text = egui::RichText::new(err.to_string()).color(egui::Color32::RED);
                ui.label(rich_text);
            }
        }
    }
}

impl Default for StagingMaintainer {
    fn default() -> Self {
        Self {
            github_user_widget: Default::default(),
            affiliation_widget: Default::default(),
            email_widget: Default::default(),
            orcid_widget: Default::default(),
            name_widget: Default::default(),
        }
    }
}

impl StatefulWidget for StagingMaintainer {
    type Value<'p> = Result<rdf::Maintainer>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        egui::Grid::new(id).num_columns(2).show(ui, |ui| {
            ui.strong("Github User: ");
            self.github_user_widget.draw_and_parse(ui, id.with("github_user"));
            ui.end_row();

            ui.strong("Affiliation: ");
            self.affiliation_widget.draw_and_parse(ui, id.with("affiliation"));
            ui.end_row();

            ui.strong("Email: ");
            self.email_widget.draw_and_parse(ui, id.with("email"));
            ui.end_row();

            ui.strong("Orcid: ");
            self.orcid_widget.draw_and_parse(ui, id.with("orcid"));
            ui.end_row();

            ui.strong("Name: ");
            self.name_widget.draw_and_parse(ui, id.with("name"));
            ui.end_row();
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(rdf::Maintainer {
            github_user: self.github_user_widget.state()?,
            name: self.name_widget.state().transpose()?,
            affiliation: self.affiliation_widget.state().transpose()?,
            email: self.email_widget.state().transpose()?,
            orcid: self.orcid_widget.state().transpose()?,
        })
    }
}
