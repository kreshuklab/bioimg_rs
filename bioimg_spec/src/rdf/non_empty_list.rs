use std::{
    borrow::Borrow,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct NonEmptyList<T>(Vec<T>);

impl<T> Deref for NonEmptyList<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for NonEmptyList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

impl<T> NonEmptyList<T> {
    pub fn map<F, Out>(&self, f: F) -> NonEmptyList<Out>
    where
        F: Fn(&T) -> Out,
    {
        NonEmptyList(self.iter().map(f).collect())
    }
}
