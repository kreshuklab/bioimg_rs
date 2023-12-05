use std::{borrow::Borrow, error::Error};

pub struct Clamped<const MIN: usize, const MAX: usize, T>(T);

#[derive(thiserror::Error, Debug)]
pub enum ClampedValueParsingError{
    #[error("{source}")]
    BadValue{source: Box<dyn Error + 'static>},
    #[error("Value '{value}' is not in range [{min}, {max}]")]
    ValueNotInRange{value: usize, min: usize, max: usize},
}

impl<const MIN: usize, const MAX: usize, T: Borrow<usize>>
Borrow<usize> for Clamped<MIN, MAX, T>{
    fn borrow(&self) -> &usize {
        return self.0.borrow()
    }
}

impl<const MIN: usize, const MAX: usize, T, E>
TryFrom<usize> for Clamped<MIN, MAX, T>
where
E: Error + 'static,
T: TryFrom<usize, Error = E>,
T: Borrow<usize>,
{
    type Error = ClampedValueParsingError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let inner = match T::try_from(value){
            Err(err) => return Err(ClampedValueParsingError::BadValue { source: Box::new(err) }),
            Ok(inner_val) => inner_val,
        };
        let value = *inner.borrow();
        if !(MIN..=MAX).contains(&value){
            return Err(ClampedValueParsingError::ValueNotInRange { value, min: MIN, max: MAX })
        }
        Ok(Self(inner))
    }
}

impl<const MIN: usize, const MAX: usize, T: Borrow<usize>>
From<Clamped<MIN, MAX, T>> for usize{
    fn from(value: Clamped<MIN, MAX, T>) -> Self {
        return *value.borrow()
    }
}