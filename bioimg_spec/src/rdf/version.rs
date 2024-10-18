use std::str::FromStr;


#[derive(thiserror::Error, Debug)]
#[error("Error parsing version: {reason}")]
pub struct VersionParsingError {
    reason: String,
}

#[derive(
    PartialOrd, Ord, Clone, Debug, PartialEq, Eq,
    serde::Deserialize, serde::Serialize,
    derive_more::Display, derive_more::Deref, derive_more::FromStr,
)]
#[serde(try_from="VersionMsg")]
#[serde(into="String")]
pub struct Version(versions::Version);

impl Version{
    pub fn major_minor_patch(major: u32, minor: u32, patch: u32) -> Self{
        Version(versions::Version{
            chunks: versions::Chunks(vec![
                versions::Chunk::Numeric(major),
                versions::Chunk::Numeric(minor),
                versions::Chunk::Numeric(patch),
            ]),
            ..Default::default()
        })
    }
    pub fn version_0_5_3() -> Version{
        Self::major_minor_patch(0, 5, 3)
    }
    pub fn version_0_5_0() -> Version{
        Self::major_minor_patch(0, 5, 0)
    }
    pub fn version_0_6_0() -> Version{
        Self::major_minor_patch(0, 6, 0)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum VersionMsg{
    Text(String),
    Float(f32),
    Int(u32),
}

impl TryFrom<String> for Version{
    type Error = VersionParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match versions::Version::from_str(&value){
            Err(e) => Err(VersionParsingError{reason: e.to_string()}),
            Ok(v) => Ok(Version(v))
        }
    }
}

impl TryFrom<VersionMsg> for Version{
    type Error = VersionParsingError;
    fn try_from(value: VersionMsg) -> Result<Self, Self::Error> {
        match value{
            VersionMsg::Text(s) => Self::try_from(s.to_owned()),
            VersionMsg::Float(f) => Self::try_from(f.to_string()),
            VersionMsg::Int(i) => Self::try_from(i.to_string()),
        }
    }
}

impl From<Version> for String{
    fn from(value: Version) -> Self {
        value.0.to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(try_from="Version")]
pub struct Version_0_5_x(Version);

impl Version_0_5_x{
    pub fn new() -> Self{
        Self(Version::version_0_5_3())
    }
}

impl TryFrom<Version> for Version_0_5_x {
    type Error = VersionParsingError;
    fn try_from(version: Version) -> Result<Self, Self::Error> {
        if  version < Version::version_0_5_0() {
            return Err(VersionParsingError { reason: format!("Version is too low: {version}") })
        }
        if  version >= Version::version_0_6_0() {
            return Err(VersionParsingError { reason: format!("Version is too high: {version}") })
        }
        Ok(Self(version))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, derive_more::Display)]
#[serde(try_from = "Version")]
pub struct FutureRdfVersion(Version);

#[derive(thiserror::Error, Debug)]
pub enum FutureRdfVersionParsingError{
    #[error("Version '{found}' is too low")]
    VersionTooLow{found: Version}
}

impl TryFrom<Version> for FutureRdfVersion{
    type Error = FutureRdfVersionParsingError;
    fn try_from(value: Version) -> Result<Self, Self::Error> {
        match value.cmp(&Version::version_0_5_3()){
            std::cmp::Ordering::Greater => Ok(Self(value)),
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => Err(Self::Error::VersionTooLow { found: value })
        }
    }
}

#[test]
fn test_version_parsing() {
    use serde_json::Value as JsonValue;

    let raw_version = JsonValue::String("1.2.3".into());

    assert_eq!(
        serde_json::from_value::<Version>(raw_version).unwrap(),
        Version::major_minor_patch(1, 2, 3)
    );
    Version::try_from("1.2.bla".to_owned()).expect_err("Should have failed to parse this verison");
}
