use serde::{Deserialize, Serialize};
use url::Url;

use crate::util::PeggedString;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CiteEntry {
    pub text: PeggedString<1, 1023>, //(String) free text description
    pub doi: PeggedString<1, 1023>, // FIXME: make it stricter (DOI→String) digital object identifier, see https://www.doi.org/ (alternatively specify url)
    pub url: Url,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CiteEntry2 {
    pub text: PeggedString<1, 1023>,        //(String) free text description
    pub doi: Option<PeggedString<1, 1023>>, // FIXME: make it stricter (DOI→String) digital object identifier, see https://www.doi.org/ (alternatively specify url)
    pub url: Option<Url>,
}

impl From<CiteEntry> for CiteEntry2 {
    fn from(entry1: CiteEntry) -> Self {
        CiteEntry2 {
            text: entry1.text,
            doi: Some(entry1.doi),
            url: Some(entry1.url),
        }
    }
}
