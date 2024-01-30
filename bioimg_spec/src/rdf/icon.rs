use super::file_reference::FileReference;

#[derive(thiserror::Error, Debug)]
pub enum IconParsingError{
    #[error("{0}")]
    EmojiParsingError(EmojiParsingError)
}

pub enum Icon{
    FileReference(FileReference),
    Emoji(String),
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum EmojiParsingError{
    #[error("Bad string: {0}")]
    BadString(String),
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(try_from="String")]
#[serde(into="String")]
pub struct EmojiIcon(String);

impl TryFrom<String> for EmojiIcon{
    type Error = EmojiParsingError;
    //FIXME: check that characters/glyphs,graphemes/whatever are emoji
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !(1..=2).contains(&value.chars().count()){
            return Err(EmojiParsingError::BadString(value))
        }
        return Ok(Self(value))
    }
}

impl From<EmojiIcon> for String{
    fn from(value: EmojiIcon) -> Self {
        return value.0
    }
}