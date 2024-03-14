use super::{lowercase::Lowercase, BoundedString, FileReference};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct FileDescription{
    pub source: FileReference,
    pub sha256: Option<Sha256>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Sha256(Lowercase<BoundedString<64, 0>>);