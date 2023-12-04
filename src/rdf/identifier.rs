use std::{borrow::Borrow, error::Error};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Identifier<T>(T);

impl<T: Borrow<str>> Borrow<str> for Identifier<T>{
    fn borrow(&self) -> &str {
        return self.0.borrow()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum IdentifierParsingError {
    #[error("Bad identifier: {source}")]
    BadString { source: Box<dyn Error + 'static> },
    #[error("Empty string can't be an identifier")]
    EmptyString,
    #[error("Expected first character to be alphabetic: '{value}'")]
    MustStartWithAlphabeticalCharacter{value: String},
    #[error("Identifiers cannot contain whitespace: '{value}'")]
    ContainsbadCharacter{value: String, position: usize}
}

impl<T, E> TryFrom<String> for Identifier<T>
where
    T: Borrow<str>,
    E: Error + 'static,
    T : TryFrom<String, Error = E>
{
    type Error = IdentifierParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner = match T::try_from(value){
            Err(err) => return Err(IdentifierParsingError::BadString { source: Box::new(err) }),
            Ok(inner_val) => inner_val,
        };
        let inner_str: &str = inner.borrow();
        let Some(first_char) = inner_str.chars().next() else {
            return Err(IdentifierParsingError::EmptyString)
        };
        if !first_char.is_alphabetic(){
            return Err(IdentifierParsingError::MustStartWithAlphabeticalCharacter { value: inner_str.into() })
        }
        for (idx, c) in inner_str.char_indices(){
            if !c.is_alphanumeric() && c != '_'{
                return Err(IdentifierParsingError::ContainsbadCharacter { value: inner_str.into(), position: idx })
            }
        }
        Ok(Self(inner))
    }
}

impl<T: Into<String>> From<Identifier<T>> for String{
    fn from(value: Identifier<T>) -> Self {
        return value.0.into()
    }
}