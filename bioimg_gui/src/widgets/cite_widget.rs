use crate::result::Result;
use bioimg_spec::rdf::{
    bounded_string::{BoundedString, BoundedStringParsingError},
    cite_entry::CiteEntry2,
};

use super::{staging_opt::StagingOpt, staging_string::StagingString, url_widget::StagingUrl, StatefulWidget};

pub type ConfString = BoundedString<1, 1023>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CiteEntry2ParsingError {
    #[error("Empty")]
    Empty,
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
    staging_text: StagingString<ConfString>,
    staging_doi: StagingOpt<StagingString<ConfString>>,
    staging_url: StagingOpt<StagingUrl>,
    parsed: Result<CiteEntry2, CiteEntry2ParsingError>,
}

impl Default for StagingCiteEntry2 {
    fn default() -> Self {
        Self {
            staging_text: Default::default(),
            staging_doi: Default::default(),
            staging_url: Default::default(),
            parsed: Err(CiteEntry2ParsingError::Empty), //FIXME: could we eliminate "Empty"
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
