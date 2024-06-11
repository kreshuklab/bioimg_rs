use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::rdf::BoundedString;

use super::HttpUrl;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CiteEntry {
    pub text: BoundedString<1, 1023>, //(String) free text description
    pub doi: BoundedString<1, 1023>, // FIXME: make it stricter (DOI→String) digital object identifier, see https://www.doi.org/ (alternatively specify url)
    pub url: HttpUrl,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct CiteEntry2 {
    pub text: BoundedString<1, 1023>,        //(String) free text description
    pub doi: Option<BoundedString<1, 1023>>, // FIXME: make it stricter (DOI→String) digital object identifier, see https://www.doi.org/ (alternatively specify url)
    pub url: Option<HttpUrl>,
}

impl Display for CiteEntry2{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
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
