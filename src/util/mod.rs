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

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(into = "usize")]
#[serde(try_from = "usize")]
pub struct ConstOne;

#[derive(thiserror::Error, Debug)]
pub enum ConstOneParsingError {
    #[error("Expected number 1, found '{found}'")]
    ExpectedNumberOne { found: usize },
}

impl TryFrom<usize> for ConstOne {
    type Error = ConstOneParsingError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 1 {
            Ok(ConstOne)
        } else {
            Err(ConstOneParsingError::ExpectedNumberOne { found: value })
        }
    }
}

impl Into<usize> for ConstOne {
    fn into(self) -> usize {
        1
    }
}
