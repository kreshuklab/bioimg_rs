use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(untagged)]
pub enum FileReference {
    Url(Url),
    Path(PathBuf),
}

impl From<Url> for FileReference {
    fn from(value: Url) -> Self {
        Self::Url(value)
    }
}

impl From<PathBuf> for FileReference {
    fn from(value: PathBuf) -> Self {
        Self::Path(value)
    }
}

#[test]
fn test_file_reference() {
    use serde_json::json;

    let raw_url = "http://bla/ble?lalala";
    let deserialized_url: FileReference = serde_json::from_value(json!(raw_url)).unwrap();
    assert_eq!(FileReference::Url(Url::parse(raw_url).unwrap()), deserialized_url,);

    let raw_path = "lalala/lelele";
    let deserialized_path: FileReference = serde_json::from_value(json!(raw_path)).unwrap();
    assert_eq!(FileReference::Path(PathBuf::from(raw_path)), deserialized_path,);
}
