use serde::{Deserialize, Serialize};
use url::Url;

use crate::util::ConfigString;

use self::{
    attachment::Attachments, author::Author, badge::Badge, cite_entry::CiteEntry,
    file_reference::FileReference, license::SpdxLicense, maintainer::Maintainer, version::Version,
};

pub mod attachment;
pub mod author;
pub mod axes;
pub mod badge;
pub mod cite_entry;
pub mod file_reference;
pub mod input_tensor;
pub mod license;
pub mod maintainer;
pub mod version;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ModelRDF {
    pub format_version: Version,
    pub description: ConfigString,
    pub name: ConfigString,
    pub attachments: Option<Attachments>,
    pub authors: Option<Vec<Author>>,
    pub badges: Option<Vec<Badge>>,
    pub cite: Option<Vec<CiteEntry>>,
    pub covers: Option<Vec<FileReference>>,
    pub documentation: Option<FileReference>,
    pub download_url: Option<FileReference>,
    pub git_repo: Option<Url>,
    pub icon: Option<ConfigString>,
    pub id: Option<ConfigString>,
    pub license: Option<SpdxLicense>,
    pub links: Option<Vec<FileReference>>,
    pub maintainers: Option<Vec<Maintainer>>,
    pub rdf_source: Option<FileReference>,
    pub source: Option<FileReference>,
    pub tags: Option<Vec<ConfigString>>,
    pub version: Option<Version>,
}

#[test]
fn test_model_rdf_serde() {
    use url::Url;

    let raw = serde_json::json!({
        "format_version": "1.2.3",
        "description": "Some fantastic model",
        "name": "my cool model",
        // "attachments": [],
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
                "text": "Plz cite eme",
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
        // "rdf_source": ,
        // "source": ,
        // "tags": ,
        "version": "4.5.6",
    });

    let parsed_rdf: ModelRDF = serde_json::from_value(raw).unwrap();
    let expected_rdf = ModelRDF {
        format_version: Version {
            major: 1,
            minor: 2,
            patch: 3,
        },
        description: "Some fantastic model".try_into().unwrap(),
        name: "my cool model".try_into().unwrap(),

        attachments: None,
        authors: Some(vec![Author {
            name: "John Doe".try_into().unwrap(),
            affiliation: "Some University".try_into().unwrap(),
            email: "john.doe@some_university.com".try_into().unwrap(),
            github_user: "john_doe".try_into().unwrap(),
            orcid: "111-111-111".try_into().unwrap(), //FIXME
        }]),
        badges: Some(vec![Badge {
            label: "x".try_into().unwrap(),
            icon: Url::parse("http://some.icon/bla").unwrap().into(),
            url: Url::parse("http://some.url/to/icon").unwrap().into(),
        }]),
        cite: Some(vec![CiteEntry {
            text: "Plz cite eme".try_into().unwrap(),
            doi: "blabla".try_into().unwrap(),
            url: Url::parse("https://blas/bla").unwrap(),
        }]),
        covers: None,
        documentation: Some(Url::parse("http://example.com/docs").unwrap().into()),
        download_url: Some(Url::parse("http://blas.blus/blis").unwrap().into()),
        git_repo: Some(Url::parse("https://github.com/blas/blus").unwrap()),
        icon: Some("x".try_into().unwrap()),
        id: Some("some_id_goes_here".try_into().unwrap()),
        license: Some(SpdxLicense::Adobe_Utopia),
        links: Some(vec![]),
        maintainers: Some(vec![]),
        rdf_source: None,
        source: None,
        tags: None,
        version: Some(Version {
            major: 4,
            minor: 5,
            patch: 6,
        }),
    };

    assert_eq!(parsed_rdf, expected_rdf);
}
