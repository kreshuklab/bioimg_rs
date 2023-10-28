use std::path::PathBuf;

use url::Url;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum FileReference{
    Url(Url),
    Path(PathBuf),
}

#[test]
fn test_file_reference(){
    use serde_json::json;

    let raw_url = "http://bla/ble?lalala";
    let deserialized_url: FileReference= serde_json::from_value(json!(raw_url)).unwrap();
    assert_eq!(
        FileReference::Url(Url::parse(raw_url).unwrap()),
        deserialized_url,
    );

    let raw_path = "lalala/lelele";
    let deserialized_path: FileReference= serde_json::from_value(json!(raw_path)).unwrap();
    assert_eq!(
        FileReference::Path(PathBuf::from(raw_path)),
        deserialized_path,
    );
}