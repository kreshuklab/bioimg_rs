use std::{fmt::Display, num::ParseIntError};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum VersionParsingError {
    #[error("Expected 3 fields, found {found}")]
    WrongNumberOfComponents { found: usize },
    #[error("Could not parse version field: {0}")]
    ParseIntError(ParseIntError),
    #[error("Expected version '{expected}', found '{found}'")]
    UnexpectedVersion { expected: Version, found: Version },
    #[error("Unexpected version number {0}")]
    UnexpectedVersionNumber(Version),
}
impl From<ParseIntError> for VersionParsingError {
    fn from(value: ParseIntError) -> Self {
        return Self::ParseIntError(value);
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_string: String = self.clone().into();
        write!(f, "{self_string}",)
    }
}
impl TryFrom<&str> for Version {
    type Error = VersionParsingError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value
            .split(".")
            .map(|comp| comp.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        let three_parts: [usize; 3] = parts
            .try_into()
            .map_err(|parts: Vec<usize>| VersionParsingError::WrongNumberOfComponents { found: parts.len() })?;
        return Ok(Version { major: three_parts[0], minor: three_parts[1], patch: three_parts[2] });
    }
}
impl TryFrom<String> for Version {
    type Error = VersionParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        return <Self as TryFrom<&str>>::try_from(&value);
    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Version_0_5_0(Version);

impl Version_0_5_0{
    pub fn new() -> Self{
        Self(Version{major: 0, minor: 5, patch: 0})
    }
}

impl TryFrom<Version> for Version_0_5_0 {
    type Error = VersionParsingError;
    fn try_from(version: Version) -> Result<Self, Self::Error> {
        if version.major == 0 && version.minor == 5 && version.patch == 0 {
            Ok(Self(version))
        } else {
            Err(VersionParsingError::UnexpectedVersionNumber(version))
        }
    }
}

#[test]
fn test_version_parsing() {
    use serde_json::Value as JsonValue;

    let raw_version = JsonValue::String("1.2.3".into());

    assert_eq!(
        serde_json::from_value::<Version>(raw_version).unwrap(),
        Version { major: 1, minor: 2, patch: 3 }
    );
    assert_eq!(
        Version::try_from("1.2"),
        Err(VersionParsingError::WrongNumberOfComponents { found: 2 })
    );
    assert_eq!(
        Version::try_from("1.2.bla"),
        Err(VersionParsingError::ParseIntError(
            "bla".parse::<u32>().expect_err("should fail parsing")
        ))
    );
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(try_from = "Version")]
#[serde(into = "Version")]
pub struct LiteralVersion<const MAJOR: usize, const MINOR: usize, const PATCH: usize>;

impl<const MAJOR: usize, const MINOR: usize, const PATCH: usize> Into<Version> for LiteralVersion<MAJOR, MINOR, PATCH> {
    fn into(self) -> Version {
        return Version { major: MAJOR, minor: MINOR, patch: PATCH };
    }
}

impl<const MAJOR: usize, const MINOR: usize, const PATCH: usize> TryFrom<Version> for LiteralVersion<MAJOR, MINOR, PATCH> {
    type Error = VersionParsingError;

    fn try_from(value: Version) -> Result<Self, Self::Error> {
        if value.major == MAJOR && value.minor == MINOR && value.patch == PATCH {
            Ok(Self)
        } else {
            Err(VersionParsingError::UnexpectedVersion { expected: Self.into(), found: value })
        }
    }
}
