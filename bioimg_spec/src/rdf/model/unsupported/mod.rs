use std::fmt::Display;

use crate::rdf::Version;

#[derive(thiserror::Error, Debug)]
pub enum LegacyVersionParsingError{
    #[error("Version '{found}' is too high")]
    VersionTooHigh{found: Version}
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[allow(non_camel_case_types)]
#[serde(try_from = "Version")]
pub struct Version_0_4_X_OrEarlier(Version);

impl Display for Version_0_4_X_OrEarlier{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<Version> for Version_0_4_X_OrEarlier{
    type Error = LegacyVersionParsingError;
    fn try_from(value: Version) -> Result<Self, Self::Error> {
        if value < Version::version_0_5_0() {
            return Ok(Self(value))
        }
        return Err(LegacyVersionParsingError::VersionTooHigh { found: value })
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UnsupportedLegacyModel{
    /// Version of the bioimage.io model description specification used.
    /// When creating a new model always use the latest micro/patch version described here.
    /// The `format_version` is important for any consumer software to understand how to parse the fields.
    pub format_version: Version_0_4_X_OrEarlier,
}

