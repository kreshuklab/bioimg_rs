use std::{borrow::Borrow, error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

const PYTHON_KEYWORDS: [&'static str; 35] = [
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else",
    "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise",
    "return", "try", "while", "with", "yield",
];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Identifier<T>(T);

impl Identifier<String> {
    pub fn appended_with(&self, suffix: &str) -> Self {
        return Self(format!("{}{suffix}", self.0));
    }
}

impl<T: Borrow<str>> Borrow<str> for Identifier<T> {
    fn borrow(&self) -> &str {
        return self.0.borrow();
    }
}

impl<T: Display> Display for Identifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum IdentifierParsingError {
    #[error("Bad identifier: {source}")]
    BadString { source: Box<dyn Error + 'static> },
    #[error("Empty string can't be an identifier")]
    EmptyString,
    #[error("Expected first character to be alphabetic or _: '{value}'")]
    MustStartWithAlphabeticalOrUnderscore { value: String },
    #[error("Identifiers cannot contain whitespace: '{value}'")]
    ContainsbadCharacter { value: String, position: usize },
    #[error("Value '{value}' is a python keyword")]
    IsPythonKeyword { value: String },
}

impl<T, E> TryFrom<String> for Identifier<T>
where
    T: Borrow<str>,
    E: Error + 'static,
    T: TryFrom<String, Error = E>,
{
    type Error = IdentifierParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner = match T::try_from(value) {
            Err(err) => return Err(IdentifierParsingError::BadString { source: Box::new(err) }),
            Ok(inner_val) => inner_val,
        };
        let inner_str: &str = inner.borrow();
        let Some(first_char) = inner_str.chars().next() else {
            return Err(IdentifierParsingError::EmptyString);
        };
        if !first_char.is_alphabetic() && first_char != '_' {
            return Err(IdentifierParsingError::MustStartWithAlphabeticalOrUnderscore { value: inner_str.into() });
        }
        for (idx, c) in inner_str.char_indices() {
            if !c.is_alphanumeric() && c != '_' {
                return Err(IdentifierParsingError::ContainsbadCharacter {
                    value: inner_str.into(),
                    position: idx,
                });
            }
        }
        if PYTHON_KEYWORDS.iter().copied().position(|kw| kw == inner_str).is_some() {
            return Err(IdentifierParsingError::IsPythonKeyword { value: inner_str.into() });
        }
        Ok(Self(inner))
    }
}

impl<T: Into<String>> From<Identifier<T>> for String {
    fn from(value: Identifier<T>) -> Self {
        return value.0.into();
    }
}
