use std::{borrow::Borrow, fmt::Display, ops::RangeInclusive, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, PartialEq, Eq, Debug, Clone)]
pub enum BoundedStringParsingError {
    #[error("Expected a string with length in {allowed:?}")]
    BadLength { allowed: RangeInclusive<usize> },
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Hash, PartialOrd, Ord)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct BoundedString<const MIN_CHARS: usize, const EXTRA_CHARS: usize>(Arc<str>);

impl<const EXTRA_CHARS: usize> Default for BoundedString<0, EXTRA_CHARS> {
    fn default() -> Self {
        Self(Arc::from(""))
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> TryFrom<&str> for BoundedString<MIN_CHARS, EXTRA_CHARS> {
    type Error = BoundedStringParsingError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let allowed = MIN_CHARS..=MIN_CHARS + EXTRA_CHARS;
        if allowed.contains(&value.len()) {
            Ok(BoundedString(Arc::from(value)))
        } else {
            Err(BoundedStringParsingError::BadLength { allowed })
        }
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> TryFrom<String> for BoundedString<MIN_CHARS, EXTRA_CHARS> {
    type Error = BoundedStringParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> Display for BoundedString<MIN_CHARS, EXTRA_CHARS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> Borrow<str> for BoundedString<MIN_CHARS, EXTRA_CHARS> {
    fn borrow(&self) -> &str {
        return &self.0;
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> BoundedString<MIN_CHARS, EXTRA_CHARS> {
    pub fn as_str(&self) -> &str {
        return &self.0;
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> From<BoundedString<MIN_CHARS, EXTRA_CHARS>> for String {
    fn from(value: BoundedString<MIN_CHARS, EXTRA_CHARS>) -> Self {
        return value.0.as_ref().to_owned();
    }
}
