use std::{borrow::Borrow, fmt::Display};

use super::{lowercase::Lowercase, BoundedString, EnvironmentFile, FileReference};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct FileDescription<R: Borrow<FileReference>>{
    pub source: R,
    pub sha256: Option<Sha256>,
}

impl<R: Borrow<FileReference>> Display for FileDescription<R>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source.borrow())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Sha256(Lowercase<BoundedString<64, 0>>);


pub type EnvironmentFileDescr = FileDescription<EnvironmentFile>;