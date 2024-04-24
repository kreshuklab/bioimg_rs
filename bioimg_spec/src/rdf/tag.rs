use super::{bounded_string::BoundedStringParsingError, BoundedString};


#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Tag(BoundedString<1, {1024 -1}>);

impl From<Tag> for String{
    fn from(value: Tag) -> Self {
        value.0.into()
    }
}

impl TryFrom<String> for Tag{
    type Error = BoundedStringParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(BoundedString::try_from(value)?))
    }
}