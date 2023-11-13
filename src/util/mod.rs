use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ConfigString(String);

impl Display for ConfigString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for ConfigString {
    fn into(self) -> String {
        return self.0;
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigStringParsingError {
    #[error("Configuration string cannot be empty")]
    EmptyString,
}

impl TryFrom<String> for ConfigString {
    type Error = ConfigStringParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            Err(ConfigStringParsingError::EmptyString)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<&str> for ConfigString {
    type Error = ConfigStringParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        String::from(value).try_into()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum SingleOrMultiple<T> {
    Single(T),
    Multiple(Vec<T>),
}

impl<T> SingleOrMultiple<T> {
    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::Single(t) => std::slice::from_ref(t),
            Self::Multiple(ts) => ts,
        }
    }
}
