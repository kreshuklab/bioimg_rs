use serde::{Deserialize, Serialize};

use crate::rdf::PeggedString;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: PeggedString<1, 1023>,        // (Name→String) Full name. FIXME: disallow / and \.
    pub affiliation: PeggedString<1, 1023>, // (String) Affiliation.
    pub email: PeggedString<1, 1023>,       // FIXME: make a parser here (Email) E-Mail
    pub github_user: PeggedString<1, 1023>, // (String) GitHub user name.
    pub orcid: PeggedString<1, 1023>, // FIXME!! more string than string! orcid id in hyphenated groups of 4 digits, e.g. '0000-0001-2345-6789' (and valid as per ISO 7064 11,2.)
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Author2 {
    pub name: PeggedString<1, 1023>,                // (Name→String) Full name.
    pub affiliation: Option<PeggedString<1, 1023>>, // (String) Affiliation.
    pub email: Option<PeggedString<1, 1023>>,       // FIXME: make a parser here (Email) E-Mail
    pub github_user: Option<PeggedString<1, 1023>>, // (String) GitHub user name.
    pub orcid: Option<PeggedString<1, 1023>>, // FIXME!! more string than string! orcid id in hyphenated groups of 4 digits, e.g. '0000-0001-2345-6789' (and valid as per ISO 7064 11,2.)
}

impl From<Author> for Author2 {
    fn from(value: Author) -> Self {
        Author2 {
            name: value.name,
            affiliation: Some(value.affiliation),
            email: Some(value.email),
            github_user: Some(value.github_user),
            orcid: Some(value.orcid),
        }
    }
}
