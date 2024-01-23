use bioimg_spec::rdf::{bounded_string::{BoundedString, BoundedStringParsingError}, cite_entry::CiteEntry2};

use super::{url_widget::StagingUrl, DrawAndParse, StagingOpt, StagingString};

pub type ConfString = BoundedString<1, 1023>;

#[derive(thiserror::Error, Debug)]
pub enum CiteEntry2ParsingError{
    #[error("{0}")]
    FieldError(#[from] #[source] BoundedStringParsingError),
    #[error("{0}")]
    BadUrl(#[from] url::ParseError),
}



#[derive(Default)]
pub struct StagingCiteEntry2{
    staging_text: StagingString<ConfString>,
    staging_doi: StagingOpt<StagingString<ConfString>>,
    staging_url: StagingOpt<StagingUrl>,
}

impl DrawAndParse for StagingCiteEntry2{
    type Parsed<'p> = CiteEntry2;
    type Error = CiteEntry2ParsingError;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<CiteEntry2, CiteEntry2ParsingError>{
        ui.scope(|ui|{
            egui::Grid::new(id).show(ui, |ui| {
                ui.strong("Text: ");
                let text = self.staging_text.draw_and_parse(ui, id.with("Text"));
                ui.end_row();

                ui.strong("Doi: ");
                let doi = self.staging_doi.draw_and_parse(ui, id.with("Doi"));
                ui.end_row();

                ui.strong("Url: ");
                let url = self.staging_url.draw_and_parse(ui, id.with("Url"));
                ui.end_row();

                Ok(CiteEntry2{
                    text: text?,
                    doi: doi?,
                    url: url?
                })
            }).inner
        }).inner
    }
}
