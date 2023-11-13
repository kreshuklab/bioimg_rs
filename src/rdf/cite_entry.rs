use serde::{Deserialize, Serialize};
use url::Url;

use crate::util::ConfigString;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CiteEntry {
    pub text: ConfigString, //(String) free text description
    pub doi: ConfigString, // FIXME: make it stricter (DOI→String) digital object identifier, see https://www.doi.org/ (alternatively specify url)
    pub url: Url,
}
