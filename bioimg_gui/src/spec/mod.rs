use std::{ops::RangeInclusive, fmt::Display, borrow::Borrow};

use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct StrictString<const MIN_CHARS: usize, const EXTRA_CHARS: usize>(String);

#[derive(thiserror::Error, PartialEq, Eq, Debug)]
pub enum StrictStringParsingError {
    #[error("Expected a string with length in {allowed:?}")]
    BadLength { value: String, allowed: RangeInclusive<usize> },
}
impl StrictStringParsingError{
    pub fn value(self) -> String{
        match self{
            Self::BadLength { value, .. } => value
        }
    }
}


impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> TryFrom<String> for StrictString<MIN_CHARS, EXTRA_CHARS> {
    type Error = StrictStringParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let allowed = MIN_CHARS..=MIN_CHARS + EXTRA_CHARS;
        if allowed.contains(&value.len()) {
            Ok(StrictString(value))
        } else {
            Err(StrictStringParsingError::BadLength { value, allowed })
        }
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> Display for StrictString<MIN_CHARS, EXTRA_CHARS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> Borrow<str> for StrictString<MIN_CHARS, EXTRA_CHARS> {
    fn borrow(&self) -> &str {
        return &self.0;
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> StrictString<MIN_CHARS, EXTRA_CHARS> {
    pub fn as_str(&self) -> &str {
        return &self.0;
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> Into<String> for StrictString<MIN_CHARS, EXTRA_CHARS> {
    fn into(self) -> String {
        return self.0;
    }
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> TryFrom<&str> for StrictString<MIN_CHARS, EXTRA_CHARS> {
    type Error = StrictStringParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        String::from(value).try_into()
    }
}


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Author2 {
    pub name: StrictString<1, 1023>,                // (Name→String) Full name.
    pub affiliation: Option<StrictString<1, 1023>>, // (String) Affiliation.
    pub email: Option<StrictString<1, 1023>>,       // FIXME: make a parser here (Email) E-Mail
    pub github_user: Option<StrictString<1, 1023>>, // (String) GitHub user name.
    pub orcid: Option<StrictString<1, 1023>>, // FIXME!! more string than string! orcid id in hyphenated groups of 4 digits, e.g. '0000-0001-2345-6789' (and valid as per ISO 7064 11,2.)
}