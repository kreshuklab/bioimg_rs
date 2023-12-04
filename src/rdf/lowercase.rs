use std::{borrow::Borrow, error::Error, ops::Deref};

use serde::{Serialize, Deserialize};

#[derive(thiserror::Error, Debug)]
pub enum LowercaseParsingError{
    #[error("{source}")]
    BadString{source: Box<dyn Error + 'static>},
    #[error("Character at {idx} is not lowercase: {value}")]
    IsNotLowercase{value: String, idx: usize}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lowercase<T: Borrow<str>>(T);

impl<T: Borrow<str>> Borrow<str> for Lowercase<T>{
    fn borrow(&self) -> &str {
        return self.0.borrow()
    }
}

impl<T: Borrow<str>> Deref for Lowercase<T>{
    type Target = str;
    fn deref(&self) -> &Self::Target {
        return self.borrow()
    }
}

impl<T, E> TryFrom<String> for Lowercase<T>
where
    E: Error + 'static,
    T: TryFrom<String, Error = E>,
    T: Borrow<str>,
{
    type Error = LowercaseParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner = match T::try_from(value){
            Err(err) => return Err(LowercaseParsingError::BadString { source: Box::new(err) }),
            Ok(inner_val) => inner_val,
        };
        let inner_str: &str = inner.borrow();
        if let Some(uppercase_idx) = inner_str.chars().position(|c| c.is_uppercase()){
            return Err(LowercaseParsingError::IsNotLowercase { value: inner_str.into(), idx: uppercase_idx })
        }
        Ok(Self(inner))
    }
}