use bioimg_spec::rdf::{bounded_string::{BoundedString, BoundedStringParsingError}, cite_entry::CiteEntry2};

use super::{error_display::show_if_error, url_widget::StagingUrl, StatefulWidget, StagingOpt, StagingString};

pub type ConfString = BoundedString<1, 1023>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CiteEntry2ParsingError{
    #[error("Empty")]
    Empty,
    #[error("{0}")]
    FieldError(#[from] #[source] BoundedStringParsingError),
    #[error("{0}")]
    BadUrl(#[from] url::ParseError),
}



pub struct StagingCiteEntry2{
    staging_text: StagingString<ConfString>,
    staging_doi: StagingOpt<StagingString<ConfString>>,
    staging_url: StagingOpt<StagingUrl>,
    parsed: Result<CiteEntry2, CiteEntry2ParsingError>,
}

impl Default for StagingCiteEntry2{
    fn default() -> Self {
        Self{
            staging_text: Default::default(),
            staging_doi: Default::default(),
            staging_url: Default::default(),
            parsed: Err(CiteEntry2ParsingError::Empty), //FIXME: could we eliminate "Empty"
        }
    }
}

impl StagingCiteEntry2{
    fn do_draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<CiteEntry2, CiteEntry2ParsingError>{
        ui.scope(|ui|{
            egui::Grid::new(id).show(ui, |ui| {
                ui.strong("Text: ");
                self.staging_text.draw_and_parse(ui, id.with("Text"));
                let text_res = self.staging_text.state();
                show_if_error(ui, &text_res);
                ui.end_row();

                ui.strong("Doi: ");
                self.staging_doi.draw_and_parse(ui, id.with("Doi"));
                let doi_res = self.staging_doi.state();
                if let Some(res) = &doi_res{
                    show_if_error(ui, &res);
                }
                ui.end_row();

                ui.strong("Url: ");
                self.staging_url.draw_and_parse(ui, id.with("Url"));
                let url_res = self.staging_url.state();
                if let Some(res) = &url_res{
                    show_if_error(ui, res);
                }
                ui.end_row();

                Ok(CiteEntry2{
                    text: text_res?,
                    doi: doi_res.transpose()?,
                    url: url_res.transpose()?,
                })
            }).inner
        }).inner
    }
}

impl StatefulWidget for StagingCiteEntry2{
    type Value<'p> = Result<CiteEntry2, CiteEntry2ParsingError>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        self.parsed = self.do_draw_and_parse(ui, id)
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}
