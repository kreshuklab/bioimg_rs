use serde::{Serialize, Deserialize};
use url::Url;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CiteEntry{
    pub text: String, //(String) free text description
    pub doi: String, // FIXME: make it stricter (DOI→String) digital object identifier, see https://www.doi.org/ (alternatively specify url)
    pub url: Url,
}