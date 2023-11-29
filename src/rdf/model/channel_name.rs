#[derive(thiserror::Error, Debug)]
pub enum ChannelNameParsingError {
    #[error("Value '{value}' has bad size: {length}")]
    BadLength { value: String, length: usize },
    #[error("Value '{value}' is a python keyword")]
    IsPythonKeyword { value: String },
    #[error("Unexpected character '{char}' in channel name '{value}'")]
    UnexpectedCharacter { value: String, char: char },
}

// Union[Sequence[str*], str*]

// Union of

//     Sequence of str (
//     MinLen(min_length=1);
//     Predicate(islower);
//     AfterValidator(validate_identifier);
//     AfterValidator(validate_is_not_keyword);
//     MaxLen(max_length=16)
// )

//     str (
//     StringConstraints(
//         strip_whitespace=None,
//         to_upper=None,
//         to_lower=None,
//         strict=None,
//         min_length=3,
//         max_length=16,
//         pattern='^.\{i\}.$'
//     )
// )

pub enum ChannelNames {
    Fixed(Vec<FixedChannelName>),
    Dynamic(DynamicChannelName),
}
impl ChannelNames {
    const MIN_LENGTH: usize = 1;
    const MAX_LENGTH: usize = 16;
    const PYTHON_KEYWORDS: [&'static str; 35] = [
        "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class",
        "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global",
        "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return",
        "try", "while", "with", "yield",
    ];

    fn validate_basic(value: String) -> Result<String, ChannelNameParsingError> {
        if !(Self::MIN_LENGTH..=Self::MAX_LENGTH).contains(&value.len()) {
            return Err(ChannelNameParsingError::BadLength {
                value,
                length: value.len(),
            });
        }

        if Self::PYTHON_KEYWORDS
            .iter()
            .find(|kw| **kw == value.as_str())
            .is_some()
        {
            return Err(ChannelNameParsingError::IsPythonKeyword { value });
        }
        return Ok(value);
    }
}

pub struct FixedChannelName(String);
impl TryFrom<String> for FixedChannelName {
    type Error = ChannelNameParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let val = ChannelNames::validate_basic(value)?;
        if let Some(unexpected_char) = val.chars().find(|c| *c == '{' || *c == '}'){
            return Err(ChannelNameParsingError::UnexpectedCharacter { value, char: unexpected_char })
        }
        Ok(Self(value))
    }
}

// pub struct DynamicChannelName
