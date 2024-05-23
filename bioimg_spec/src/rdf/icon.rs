use std::borrow::Borrow;

use super::file_reference::FileReference;

#[derive(thiserror::Error, Debug, Clone)]
pub enum IconParsingError {
    #[error("Not emoji: '{0}'")]
    NotEmoji(String),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Icon {
    Emoji(EmojiIcon),
    FileRef(FileReference),
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum EmojiParsingError {
    #[error("Bad string: {0}")]
    BadString(String),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct EmojiIcon(String);

impl Borrow<str> for EmojiIcon{
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl TryFrom<String> for EmojiIcon {
    type Error = IconParsingError;
    //FIXME: check that characters/glyphs,graphemes/whatever are emoji
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !(1..=2).contains(&value.chars().count()) {
            return Err(IconParsingError::NotEmoji(value));
        }
        return Ok(Self(value));
    }
}

impl TryFrom<String> for Icon {
    type Error = IconParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Icon::Emoji(EmojiIcon::try_from(value)?))
    }
}

impl From<EmojiIcon> for String {
    fn from(value: EmojiIcon) -> Self {
        return value.0;
    }
}
