use serde::{Deserialize, Serialize};

use crate::rdf::BoundedString;

use super::orcid::Orcid;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: BoundedString<1, 1023>,        // (Name→String) Full name. FIXME: disallow / and \.
    pub affiliation: BoundedString<1, 1023>, // (String) Affiliation.
    pub email: BoundedString<1, 1023>,       // FIXME: make a parser here (Email) E-Mail
    pub github_user: BoundedString<1, 1023>, // (String) GitHub user name.
    pub orcid: Orcid,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Author2 {
    pub name: BoundedString<1, 1023>,                // (Name→String) Full name.
    pub affiliation: Option<BoundedString<1, 1023>>, // (String) Affiliation.
    pub email: Option<BoundedString<1, 1023>>,       // FIXME: make a parser here (Email) E-Mail
    pub github_user: Option<BoundedString<1, 1023>>, // (String) GitHub user name.
    pub orcid: Option<Orcid>,
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
