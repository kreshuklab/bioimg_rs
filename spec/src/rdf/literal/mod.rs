use serde::{Serialize, Deserialize};

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

impl<const VAL: usize> From<LiteralInt<VAL>> for usize {
    fn from(_value: LiteralInt<VAL>) -> Self {
        return VAL
    }
}
