use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NonEmptyList<T>(Vec<T>);

impl<T> TryFrom<Vec<T>> for NonEmptyList<T> {
    type Error = Vec<T>;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(value)
        } else {
            Ok(Self(value))
        }
    }
}

impl<T> Borrow<[T]> for NonEmptyList<T> {
    fn borrow(&self) -> &[T] {
        return &self.0;
    }
}
