// use serde::{Deserialize, Serialize};


// use self::{
//     attachment::Attachments, author::Author, badge::Badge, cite_entry::CiteEntry, maintainer::Maintainer, version::Version_0_5_0,
// };

pub mod attachment;
pub mod author;
pub mod badge;
pub mod bounded_string;
pub mod cite_entry;
pub mod clamped;
pub mod file_reference;
pub mod file_description;
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
pub mod cover_image_source;

pub use bounded_string::BoundedString;
pub use icon::{EmojiIcon, Icon, IconParsingError};
pub use identifier::Identifier;
pub use license::LicenseId;
pub use literal::LiteralInt;
pub use version::Version;
pub use file_reference::{HttpUrl, FsPath, FileReference};

use self::{lowercase::Lowercase, slashless_string::SlashlessString};

pub type ResourceId = SlashlessString<Lowercase<BoundedString<1, 1023>>>;

// #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
// pub struct Rdf {
//     pub format_version: Version_0_5_0,
//     pub description: BoundedString<1, 1023>, //FIXME: double chekc lengrhs
//     pub name: BoundedString<1, 1023>,
//     pub attachments: Option<Attachments>,
//     pub authors: Option<Vec<Author>>,
//     pub badges: Option<Vec<Badge>>,
//     pub cite: Option<Vec<CiteEntry>>,
//     pub covers: Option<Vec<FileReference>>,
//     pub documentation: Option<FileReference>,
//     pub download_url: Option<FileReference>,
//     pub git_repo: Option<HttpUrl>,
//     pub icon: Option<BoundedString<1, 1023>>,
//     pub id: Option<BoundedString<1, 1023>>,
//     pub license: Option<SpdxLicense>,
//     pub links: Option<Vec<FileReference>>,
//     pub maintainers: Option<Vec<Maintainer>>,
//     pub rdf_source: Option<FileReference>,
//     pub source: Option<FileReference>,
//     pub tags: Option<Vec<BoundedString<1, 1023>>>,
//     pub version: Option<Version>,
// }

// #[test]
// fn test_model_rdf_serde() {
//     let raw = serde_json::json!({
//         "format_version": "0.5.0",
//         "description": "Some fantastic model",
//         "name": "my cool model",
//         // "attachments": [],
//         "authors": [
//             {
//                 "name": "John Doe",
//                 "affiliation": "Some University",
//                 "email": "john.doe@some_university.com" ,
//                 "github_user": "john_doe",
//                 "orcid": "0000-0001-7051-1197"
//             },
//         ],
//         "badges": [
//             {
//                 "label": "x",
//                 "icon": "http://some.icon/bla",
//                 "url": "http://some.url/to/icon"
//             }
//         ],
//         "cite": [
//             {
//                 "text": "Plz cite eme",
//                 "doi": "blabla",
//                 "url": "https://blas/bla",

//             }
//         ],
//         // "covers": [],
//         "documentation": "http://example.com/docs",
//         "download_url": "http://blas.blus/blis",
//         "git_repo": "https://github.com/blas/blus",
//         "icon": "x",
//         "id": "some_id_goes_here",
//         "license": "Adobe-Utopia",
//         "links": [],
//         "maintainers": [],
//         // "rdf_source": ,
//         // "source": ,
//         // "tags": ,
//         "version": "4.5.6",
//     });

//     let parsed_rdf: Rdf = serde_json::from_value(raw).unwrap();
//     let expected_rdf = Rdf {
//         format_version: Version_0_5_0::try_from(Version { major: 0, minor: 5, patch: 0 }).unwrap(),
//         description: "Some fantastic model".try_into().unwrap(),
//         name: "my cool model".try_into().unwrap(),

//         attachments: None,
//         authors: Some(vec![Author {
//             name: "John Doe".try_into().unwrap(),
//             affiliation: "Some University".try_into().unwrap(),
//             email: "john.doe@some_university.com".try_into().unwrap(),
//             github_user: "john_doe".try_into().unwrap(),
//             orcid: "0000-0001-7051-1197".to_owned().try_into().unwrap(), //FIXME
//         }]),
//         badges: Some(vec![Badge {
//             label: "x".try_into().unwrap(),
//             icon: HttpUrl::try_from("http://some.icon/bla".to_owned()).unwrap().into(),
//             url: HttpUrl::try_from("http://some.url/to/icon".to_owned()).unwrap().into(),
//         }]),
//         cite: Some(vec![CiteEntry {
//             text: "Plz cite eme".try_into().unwrap(),
//             doi: "blabla".try_into().unwrap(),
//             url: HttpUrl::try_from("https://blas/bla".to_owned()).unwrap(),
//         }]),
//         covers: None,
//         documentation: Some(HttpUrl::try_from("http://example.com/docs".to_owned()).unwrap().into()),
//         download_url: Some(HttpUrl::try_from("http://blas.blus/blis".to_owned()).unwrap().into()),
//         git_repo: Some(HttpUrl::try_from("https://github.com/blas/blus".to_owned()).unwrap()),
//         icon: Some("x".try_into().unwrap()),
//         id: Some("some_id_goes_here".try_into().unwrap()),
//         license: Some(SpdxLicense::Adobe_Utopia),
//         links: Some(vec![]),
//         maintainers: Some(vec![]),
//         rdf_source: None,
//         source: None,
//         tags: None,
//         version: Some(Version { major: 4, minor: 5, patch: 6 }),
//     };

//     assert_eq!(parsed_rdf, expected_rdf);
// }
