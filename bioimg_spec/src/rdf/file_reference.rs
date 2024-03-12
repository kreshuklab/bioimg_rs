use serde::{Deserialize, Serialize};
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum FsPathParsingError{
    #[error("Bad character in file path component '{raw}'")]
    BadComponentChar{raw: String},
    #[error("Empty path components")]
    EmptyComponent{raw: String},
    #[error("Empty path")]
    EmptyPath,
    #[error("Path is not absolute: {0}")]
    PathNotAbsolute(String)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct FsPathComponent(String);

impl FsPathComponent{
    pub fn unique() -> Self{
        Self( uuid::Uuid::new_v4().to_string() )
    }
}

impl TryFrom<String> for FsPathComponent{
    type Error = FsPathParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains(['/', '\\']){
            return Err(FsPathParsingError::BadComponentChar { raw: value })
        }
        if value.len() == 0{
            return Err(FsPathParsingError::EmptyComponent { raw: value })
        }
        Ok(Self(value))
    }
}

impl From<FsPathComponent> for String{
    fn from(value: FsPathComponent) -> Self {
        value.0
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct FsPath{
    components: Vec<FsPathComponent>
}

impl FsPath{
    pub fn unique() -> Self{
        Self{ components: vec![ FsPathComponent::unique() ] }
    }
}

impl TryFrom<String> for FsPath{
    type Error = FsPathParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.starts_with("/"){
            return Err(FsPathParsingError::PathNotAbsolute(value))
        }
        let components = value.split("/")
            .skip(1)
            .map(|comp| FsPathComponent::try_from(comp.to_owned()))
            .collect::<Result<Vec<_>, _>>()?;
        if components.len() == 0{
            return Err(FsPathParsingError::EmptyPath)
        }
        Ok(Self{components})
    }
}

impl From<FsPath> for String{
    fn from(value: FsPath) -> Self {
        let initial = String::with_capacity(value.components.iter().map(|comp| comp.0.len()).sum());
        return value.components.iter().fold(initial, |mut acc, comp|{
            acc += &comp.0;
            acc
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UrlParsingError{
    #[error("{0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Url is not http: {url}")]
    NotHttp{url: url::Url},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(into = "String")]
#[serde(try_from = "String")]
pub struct HttpUrl(url::Url);

impl TryFrom<String> for HttpUrl{
    type Error = UrlParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let url = url::Url::parse(&value)?;
        return Self::try_from(url)
    }
}

impl TryFrom<Url> for HttpUrl {
    type Error = UrlParsingError;
    fn try_from(url: Url) -> Result<Self, Self::Error> {
        if url.scheme() != "http" && url.scheme() != "https"{
            return Err(UrlParsingError::NotHttp { url })
        }
        Ok(Self(url))
    }
}

impl From<HttpUrl> for String{
    fn from(value: HttpUrl) -> Self {
        value.0.into()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(untagged)]
pub enum FileReference {
    Url(HttpUrl),
    Path(FsPath),
}

impl From<HttpUrl> for FileReference{
    fn from(value: HttpUrl) -> Self {
        Self::Url(value)
    }
}
impl From<FsPath> for FileReference{
    fn from(value: FsPath) -> Self {
        Self::Path(value)
    }
}

#[test]
fn test_file_reference() {
    use serde_json::json;

    let raw_url = "http://bla/ble?lalala";
    let deserialized_url: FileReference = serde_json::from_value(json!(raw_url)).unwrap();
    assert_eq!(FileReference::Url(HttpUrl::try_from(raw_url.to_owned()).unwrap()), deserialized_url,);

    let raw_path = "/lalala/lelele";
    dbg!(FsPath::try_from(raw_path.to_owned()).unwrap());
    let deserialized_path: FileReference = serde_json::from_value(json!(raw_path)).unwrap();
    assert_eq!(FileReference::Path(FsPath::try_from(raw_path.to_owned()).unwrap()), deserialized_path,);
}
