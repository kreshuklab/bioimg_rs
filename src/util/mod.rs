use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::ops::RangeInclusive;

#[derive(thiserror::Error, PartialEq, Eq, Debug)]
pub enum PeggedStringParsingError {
    #[error("Expected a string with length in {allowed:?}")]
    BadLength { value: String, allowed: RangeInclusive<usize> },
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct PeggedString<const MIN_CHARS: usize, const EXTRA_CHARS: usize>(String);

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> TryFrom<String> for PeggedString<MIN_CHARS, EXTRA_CHARS> {
    type Error = PeggedStringParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let allowed = MIN_CHARS..=MIN_CHARS + EXTRA_CHARS;
        if allowed.contains(&value.len()) {
            Ok(PeggedString(value))
        } else {
            Err(PeggedStringParsingError::BadLength { value, allowed })
        }
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> Display for PeggedString<MIN_CHARS, EXTRA_CHARS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> Borrow<str> for PeggedString<MIN_CHARS, EXTRA_CHARS> {
    fn borrow(&self) -> &str {
        return &self.0;
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> PeggedString<MIN_CHARS, EXTRA_CHARS> {
    pub fn as_str(&self) -> &str {
        return &self.0;
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> Into<String> for PeggedString<MIN_CHARS, EXTRA_CHARS> {
    fn into(self) -> String {
        return self.0;
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> TryFrom<&str> for PeggedString<MIN_CHARS, EXTRA_CHARS> {
    type Error = PeggedStringParsingError;

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
pub struct LiteralInt<const VAL: usize>;

#[derive(thiserror::Error, Debug)]
pub enum LiteralIntParsingError {
    #[error("Expected number {expected}, found '{found}'")]
    ExpectedNumberOne { expected: usize, found: usize },
}

impl<const VAL: usize> TryFrom<usize> for LiteralInt<VAL> {
    type Error = LiteralIntParsingError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == VAL {
            Ok(Self)
        } else {
            Err(LiteralIntParsingError::ExpectedNumberOne {
                expected: VAL,
                found: value,
            })
        }
    }
}

impl<const VAL: usize> Into<usize> for LiteralInt<VAL> {
    fn into(self) -> usize {
        VAL
    }
}
