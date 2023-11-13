use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Maintainer{
    pub github_user: String,
    pub affiliation: String,
    pub email: String, //FIXME
    pub name: String, //FIXME?
    pub orcid: String, //FIXME
}