use bioimg_spec::rdf::{author::Author2, bounded_string::BoundedString, orcid::Orcid};

use super::{collapsible_widget::{CollapsibleWidget, SummarizableWidget}, staging_opt::StagingOpt, staging_string::StagingString, staging_vec::ItemWidgetConf, StatefulWidget, ValueWidget};
use crate::{project_data::AuthorWidgetProjectData1, result::Result};

pub type ConfString = BoundedString<1, 1024>;

pub struct AuthorWidget {
    pub name_widget: StagingString<ConfString>,                    // (Name→String) Full name.
    pub affiliation_widget: StagingOpt<StagingString<ConfString>>, // (String) Affiliation.
    pub email_widget: StagingOpt<StagingString<ConfString>>,       // FIXME: make a parser here (Email) E-Mail
    pub github_user_widget: StagingOpt<StagingString<ConfString>>, // (String) GitHub user name.
    pub orcid_widget: StagingOpt<StagingString<Orcid>>,
}

impl AuthorWidget{
    pub fn get_proj_data(&self) -> AuthorWidgetProjectData1{
        AuthorWidgetProjectData1 {
            name: self.name_widget.raw.clone(),
            affiliation: self.affiliation_widget.0.as_ref().map(|val| val.raw.clone()),
            email: self.email_widget.0.as_ref().map(|val| val.raw.clone()),
            github_user: self.github_user_widget.0.as_ref().map(|val| val.raw.clone()),
            orcid: self.orcid_widget.0.as_ref().map(|val| val.raw.clone()),
        }
    }
    pub fn restor_from_proj_data(&mut self, _proj_data: AuthorWidgetProjectData1){
        // self.name_widget.set_value(proj_data.name);
    }
}

impl ValueWidget for AuthorWidget{
    type Value<'a> = Author2;
    fn set_value<'a>(&mut self, value: Self::Value<'a>) {
        self.name_widget.set_value(value.name);
        self.affiliation_widget.set_value(value.affiliation);
        self.email_widget.set_value(value.email);
        self.github_user_widget.set_value(value.github_user);
        self.orcid_widget.set_value(value.orcid);
    }
}

impl ItemWidgetConf for AuthorWidget{
    const ITEM_NAME: &'static str = "Author";
    const MIN_NUM_ITEMS: usize = 1;
}

impl ItemWidgetConf for CollapsibleWidget<AuthorWidget>{
    const ITEM_NAME: &'static str = "Author";
    const MIN_NUM_ITEMS: usize = 1;
    const GROUP_FRAME: bool = false;
}

impl SummarizableWidget for AuthorWidget{
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

impl Default for AuthorWidget {
    fn default() -> Self {
        Self {
            name_widget: Default::default(),
            affiliation_widget: Default::default(),
            email_widget: Default::default(),
            github_user_widget: Default::default(),
            orcid_widget: Default::default(),
        }
    }
}


impl StatefulWidget for AuthorWidget {
    type Value<'p> = Result<Author2>;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        egui::Grid::new(id).num_columns(2).show(ui, |ui| {
            ui.strong("Name: ");
            self.name_widget.draw_and_parse(ui, id.with("Name"));
            ui.end_row();

            ui.strong("Affiliation: ");
            self.affiliation_widget.draw_and_parse(ui, id.with("Affiliation"));
            ui.end_row();

            ui.strong("Email: ");
            self.email_widget.draw_and_parse(ui, id.with("Email"));
            ui.end_row();

            ui.strong("Github User: ");
            self.github_user_widget.draw_and_parse(ui, id.with("Github User"));
            ui.end_row();

            ui.strong("Orcid: ");
            self.orcid_widget.draw_and_parse(ui, id.with("Orcid"));
            ui.end_row();
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(Author2 { //FIXME: maybe check everything before cloning?
            name: self.name_widget.state().cloned()?,
            affiliation: self.affiliation_widget.state().transpose()?.cloned(),
            email: self.email_widget.state().transpose()?.cloned(),
            github_user: self.github_user_widget.state().transpose()?.cloned(),
            orcid: self.orcid_widget.state().transpose()?.cloned(),
        })
    }
}
