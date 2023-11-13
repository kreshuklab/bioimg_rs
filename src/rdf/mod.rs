use serde::{Serialize, Deserialize};
use url::Url;

use self::{format_version::Version, author::Author, file_reference::FileReference, attachment::Attachments, badge::Badge, cite_entry::CiteEntry, license::SpdxLicense, maintainer::Maintainer};

pub mod format_version;
pub mod file_reference;
pub mod author;
pub mod badge;
pub mod cite_entry;
pub mod input_tensor;
pub mod axes;
pub mod attachment;
pub mod license;
pub mod maintainer;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(transparent)]
pub struct Name(String);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(transparent)]
pub struct Description(String);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ModelRDF{
    pub format_version: Version,
    pub description: Description,
    pub name: Name,
    pub attachments: Option<Attachments>,
    pub authors: Option<Vec<Author>>,
    pub badges: Option<Vec<Badge>>,
    pub cite: Option<Vec<CiteEntry>>,
    pub covers: Option<Vec<FileReference>>,
    pub documentation: Option<FileReference>,
    pub download_url: Option<FileReference>,
    pub git_repo: Option<Url>,
    pub icon: Option<String>,
    pub id: Option<String>,
    pub license: Option<SpdxLicense>,
    pub links: Option<Vec<FileReference>>,
    pub maintainers: Option<Vec<Maintainer>>,
    pub rdf_source: Option<FileReference>,
    pub source: Option<FileReference>,
    pub tags: Option<Vec<String>>,
    pub version: Option<Version>,
}

#[test]
fn test_model_rdf_serde(){
    use url::Url;

    let raw = serde_json::json!({
        "format_version": "1.2.3",
        "description": "Some fantastic model",
        "name": "my cool model",
        "authors": [
            {
                "name": "John Doe",
                "affiliation": "Some University",
                "email": "john.doe@some_university.com" ,
                "github_user": "john_doe",
                "orcid": "111-111-111", //FIXME
            },
        ],
        "badges": [
            {
                "label": "x",
                "icon": "http://some.icon/bla",
                "url": "http://some.url/to/icon"
            }
        ],
        "cite": [
            {
                "text": "Plz cit eme",
                "doi": "blabla",
                "url": "https://blas/bla",

            }
        ],
        // "covers": [],
        "documentation": "http://example.com/docs",
        "download_url": "http://blas.blus/blis",
        "git_repo": "https://github.com/blas/blus",
        "icon": "x",
        "id": "some_id_goes_here",
        "license": "Adobe-Utopia",
        "links": [],
        "maintainers": [],
        "rdf_source": ,
        // "source": ,
        // "tags": ,
        // "version": ,
    });

    let parsed_rdf: ModelRDF = serde_json::from_value(raw).unwrap();
    let expected_rdf = ModelRDF{
        format_version: Version { major: 1, minor: 2, patch: 3 },
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