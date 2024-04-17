use std::{borrow::Borrow, error::Error};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct SlashlessString<T>(T)
where
    T: Borrow<str> + TryFrom<String>,
    <T as TryFrom<String>>::Error: Error + 'static
;

#[derive(thiserror::Error, Debug)]
pub enum SlashlessStringError {
    #[error("{0}")]
    BadString(Box<dyn Error + 'static>),
    #[error("String has slashes: {0}")]
    ContainsSlashes(String),
}

impl<T> From<SlashlessString<T>> for String
where
    T: Borrow<str> + TryFrom<String>,
    <T as TryFrom<String>>::Error: Error + 'static
{
    fn from(value: SlashlessString<T>) -> Self {
        value.0.borrow().to_owned()
    }
}

impl<T> TryFrom<String> for SlashlessString<T>
where
    T: Borrow<str> + TryFrom<String>,
    <T as TryFrom<String>>::Error: Error + 'static
{
    type Error = SlashlessStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let t = match T::try_from(value) {
            Ok(t) => t,
            Err(err) => return Err(SlashlessStringError::BadString(Box::new(err)))
        };
        if t.borrow().chars().find(|c| *c == '/' || *c == '\\').is_some() {
            return Err(SlashlessStringError::ContainsSlashes(t.borrow().into()));
        }
        Ok(Self(t))
    }
}
