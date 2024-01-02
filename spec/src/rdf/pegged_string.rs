use std::{ops::RangeInclusive, fmt::Display, borrow::Borrow};

use serde::{Serialize, Deserialize};

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
