use serde::{Deserialize, Serialize};

use crate::util::ConfigString;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: ConfigString,        // (Name→String) Full name.
    pub affiliation: ConfigString, // (String) Affiliation.
    pub email: ConfigString,       // FIXME: make a parser here (Email) E-Mail
    pub github_user: ConfigString, // (String) GitHub user name.
    pub orcid: ConfigString, // FIXME!! more string than string! orcid id in hyphenated groups of 4 digits, e.g. '0000-0001-2345-6789' (and valid as per ISO 7064 11,2.)
}
