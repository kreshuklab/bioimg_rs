use super::bounded_string::{BoundedString, BoundedStringParsingError};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct SlashlessString<const MIN_CHARS: usize, const EXTRA_CHARS: usize>(BoundedString<MIN_CHARS, EXTRA_CHARS>);

#[derive(thiserror::Error, Debug, Clone)]
pub enum SlashlessStringError {
    #[error("{0}")]
    BoundedStringParsingError(#[from] BoundedStringParsingError),
    #[error("String has slashes: {0}")]
    ContainsSlashes(String),
}

impl<const MIN_CHARS: usize, const EXTRA_CHARS: usize> TryFrom<String> for SlashlessString<MIN_CHARS, EXTRA_CHARS> {
    type Error = SlashlessStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bs: BoundedString<MIN_CHARS, EXTRA_CHARS> = TryFrom::try_from(value)?;
        if bs.as_str().chars().find(|c| *c == '/' || *c == '\\').is_some() {
            return Err(SlashlessStringError::ContainsSlashes(bs.into()));
        }
        Ok(Self(bs))
    }
}
