use std::marker::PhantomData;

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

#[derive(thiserror::Error, Debug)]
pub enum MarkerParsingError{
    #[error("Expected '{expected}', found '{found}'")]
    UnexpectedString{expected: &'static str, found: String}
}

pub trait Marker: Clone{
    const NAME: &'static str;
}

macro_rules! declare_lowercase_marker {($name:ident) => { paste!{
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct $name;
    impl Marker for $name {
        const NAME: &'static str = stringify!( [<$name:lower>] );
    }
}};}

pub(crate) use declare_lowercase_marker;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct LitStrMarker<M: Marker>(PhantomData<M>);

impl<M: Marker> LitStrMarker<M>{
    pub fn new() -> Self{ Self(PhantomData) }
}

impl<N: Marker> From<LitStrMarker<N>> for String{
    fn from(value: LitStrMarker<N>) -> Self {
        N::NAME.to_owned()
    }
}

impl<N: Marker> TryFrom<String> for LitStrMarker<N>{
    type Error = MarkerParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == N::NAME{
            Ok(Self(PhantomData))
        }else{
            Err(MarkerParsingError::UnexpectedString { expected: N::NAME, found: value })
        }
    }
}
