use serde::{Deserialize, Serialize};
use url::Url;

use self::{attachment::Attachments, author::Author, badge::Badge, cite_entry::CiteEntry, maintainer::Maintainer};

pub mod attachment;
pub mod author;
pub mod badge;
pub mod bounded_string;
pub mod cite_entry;
pub mod clamped;
pub mod file_reference;
pub mod icon;
pub mod identifier;
pub mod license;
pub mod literal;
pub mod lowercase;
pub mod maintainer;
pub mod model;
pub mod non_empty_list;
pub mod orcid;
pub mod si_units;
pub mod slashless_string;
pub mod version;

pub use bounded_string::BoundedString;
pub use file_reference::FileReference;
pub use icon::{EmojiIcon, Icon, IconParsingError};
pub use identifier::Identifier;
pub use license::SpdxLicense;
pub use literal::LiteralInt;
pub use version::Version;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Rdf {
    pub format_version: Version,
    pub description: BoundedString<1, 1023>, //FIXME: double chekc lengrhs
    pub name: BoundedString<1, 1023>,
    pub attachments: Option<Attachments>,
    pub authors: Option<Vec<Author>>,
    pub badges: Option<Vec<Badge>>,
    pub cite: Option<Vec<CiteEntry>>,
    pub covers: Option<Vec<FileReference>>,
    pub documentation: Option<FileReference>,
    pub download_url: Option<FileReference>,
    pub git_repo: Option<Url>,
    pub icon: Option<BoundedString<1, 1023>>,
    pub id: Option<BoundedString<1, 1023>>,
    pub license: Option<SpdxLicense>,
    pub links: Option<Vec<FileReference>>,
    pub maintainers: Option<Vec<Maintainer>>,
    pub rdf_source: Option<FileReference>,
    pub source: Option<FileReference>,
    pub tags: Option<Vec<BoundedString<1, 1023>>>,
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

    let parsed_rdf: Rdf = serde_json::from_value(raw).unwrap();
    let expected_rdf = Rdf {
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
            orcid: "0000-0002-8205-121X".to_owned().try_into().unwrap(), //FIXME
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
