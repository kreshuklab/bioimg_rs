use crate::result::Result;
use bioimg_spec::rdf::{
    bounded_string::{BoundedString, BoundedStringParsingError},
    cite_entry::CiteEntry2,
};

use super::{staging_opt::StagingOpt, staging_string::StagingString, staging_vec::ItemWidgetConf, url_widget::StagingUrl, StatefulWidget, ValueWidget};

pub type ConfString = BoundedString<1, 1023>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CiteEntry2ParsingError {
    #[error("{0}")]
    FieldError(
        #[from]
        #[source]
        BoundedStringParsingError,
    ),
    #[error("{0}")]
    BadUrl(#[from] url::ParseError),
}

pub struct StagingCiteEntry2 {
    pub staging_text: StagingString<ConfString>,
    pub staging_doi: StagingOpt<StagingString<ConfString>>,
    pub staging_url: StagingOpt<StagingUrl>,
}

impl ValueWidget for StagingCiteEntry2{
    type Value<'a> = CiteEntry2;
    fn set_value<'a>(&mut self, value: Self::Value<'a>) {
        self.staging_text.set_value(value.text);
        self.staging_doi.set_value(value.doi);
        self.staging_url.set_value(value.url);
    }
}

impl ItemWidgetConf for StagingCiteEntry2{
    const ITEM_NAME: &'static str = "Cite";
    const MIN_NUM_ITEMS: usize = 1;
}

impl Default for StagingCiteEntry2 {
    fn default() -> Self {
        Self {
            staging_text: Default::default(),
            staging_doi: Default::default(),
            staging_url: Default::default(),
        }
    }
}

impl StatefulWidget for StagingCiteEntry2 {
    type Value<'p> = Result<CiteEntry2>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        egui::Grid::new(id).show(ui, |ui| {
            ui.strong("Text: ");
            self.staging_text.draw_and_parse(ui, id.with("Text"));
            ui.end_row();

            ui.strong("Doi: ");
            self.staging_doi.draw_and_parse(ui, id.with("Doi"));
            ui.end_row();

            ui.strong("Url: ");
            self.staging_url.draw_and_parse(ui, id.with("Url"));
            ui.end_row();
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(CiteEntry2 {
            text: self.staging_text.state().to_owned()?,
            doi: self.staging_doi.state().to_owned().transpose()?,
            url: self.staging_url.state().to_owned().transpose()?,
        })
    }
}
