use serde::{Serialize, Deserialize};

use self::{format_version::FormatVersion, author::Author, file_reference::FileReference};

pub mod format_version;
pub mod file_reference;
pub mod author;
pub mod badge;
pub mod cite_entry;
pub mod input_tensor;
pub mod axes;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(transparent)]
pub struct Name(String);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(transparent)]
pub struct Description(String);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ModelRDF{
    pub format_version: FormatVersion,
    pub authors: Vec<Author>,
    pub description: Description,
    pub documentation: FileReference,
}

#[test]
fn test_model_rdf_serde(){
    use url::Url;

    let raw = serde_json::json!({
        "format_version": "1.2.3",
        "authors": [
            {
                "name": "John Doe",
                "affiliation": "Some University",
                "email": "john.doe@some_university.com" ,
                "github_user": "john_doe",
                "orcid": "111-111-111", //FIXME
            },
        ],
        "description": "Some fantastic model",
        "documentation": "http://example.com/docs"
    });

    let parsed_rdf: ModelRDF = serde_json::from_value(raw).unwrap();
    let expected_rdf = ModelRDF{
        format_version: FormatVersion { major: 1, minor: 2, patch: 3 },
        authors: vec![Author{
            name: "John Doe".into(),
            affiliation: "Some University".into(),
            email: "john.doe@some_university.com".into(),
            github_user: "john_doe".into(),
            orcid: "111-111-111".into(), //FIXME
        }],
        description: Description("Some fantastic model".into()),
        documentation: FileReference::Url(Url::parse("http://example.com/docs").unwrap()),
    };

    assert_eq!(parsed_rdf, expected_rdf);
}