use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Author{
    pub name: String, // (Name→String) Full name.
    pub affiliation: String, // (String) Affiliation.
    pub email: String, // FIXME: make a parser here (Email) E-Mail
    pub github_user: String, // (String) GitHub user name.
    pub orcid: String, // FIXME!! more string than string! orcid id in hyphenated groups of 4 digits, e.g. '0000-0001-2345-6789' (and valid as per ISO 7064 11,2.)
}
