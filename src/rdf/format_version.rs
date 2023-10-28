use std::num::ParseIntError;

use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum VersionParsingError{
    #[error("Expected 3 fields, found {found}")]
    WrongNumberOfComponents{found: usize},
    #[error("Could not parse version field: {0}")]
    ParseIntError(ParseIntError)
}
impl From<ParseIntError> for VersionParsingError{
    fn from(value: ParseIntError) -> Self {
        return Self::ParseIntError(value)
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct FormatVersion{
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
impl TryFrom<&str> for FormatVersion{
    type Error = VersionParsingError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(".")
            .map(|comp| comp.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        if parts.len() != 3{
            return Err(VersionParsingError::WrongNumberOfComponents { found: parts.len() })
        }
        return Ok(FormatVersion{major: parts[0], minor: parts[1], patch: parts[2]})
    }
}
impl TryFrom<String> for FormatVersion{
    type Error = VersionParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        return <Self as TryFrom<&str>>::try_from(&value)
    }
}

impl Into<String> for FormatVersion{
    fn into(self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[test]
fn test_version_parsing() {
    use serde_json::Value as JsonValue;

    let raw_version = JsonValue::String("1.2.3".into());

    assert_eq!(
        serde_json::from_value::<FormatVersion>(raw_version).unwrap(),
        FormatVersion{major: 1, minor: 2, patch: 3}
    );
    assert_eq!(
        FormatVersion::try_from("1.2"),
        Err(VersionParsingError::WrongNumberOfComponents { found: 2 })
    );
    assert_eq!(
        FormatVersion::try_from("1.2.bla"),
        Err(VersionParsingError::ParseIntError("bla".parse::<u32>().expect_err("should fail parsing")))
    );
}

