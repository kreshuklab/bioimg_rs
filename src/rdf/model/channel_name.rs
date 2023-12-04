use serde::{Deserialize, Serialize};

use crate::util::{PeggedString, PeggedStringParsingError};

#[derive(thiserror::Error, Debug)]
pub enum ChannelNameParsingError {
    #[error("Value '{value}' has bad size: {length}")]
    BadLength { value: String, length: usize },
    #[error("Value '{value}' is a python keyword")]
    IsPythonKeyword { value: String },
    #[error("Unexpected character '{char}' in channel name '{value}'")]
    UnexpectedCharacter { value: String, char: char },
    #[error("Expected a string like 'prefix{{i}}suffix, found {value}")]
    BadDynamicChannelname { value: String },
    #[error("Bad configuration string: {source}")]
    BadConfigString { source: PeggedStringParsingError },
}

impl From<PeggedStringParsingError> for ChannelNameParsingError {
    fn from(source: PeggedStringParsingError) -> Self {
        Self::BadConfigString { source }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelNames {
    Dynamic(DynamicChannelName),
    Shorthand(FixedChannelName),
    Fixed(Vec<FixedChannelName>),
}

impl Default for ChannelNames {
    fn default() -> Self {
        ChannelNames::Dynamic(DynamicChannelName::default())
    }
}

impl ChannelNames {
    const MIN_LENGTH: usize = 1;
    const MAX_LENGTH: usize = 16;
    const PYTHON_KEYWORDS: [&'static str; 35] = [
        "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif",
        "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or",
        "pass", "raise", "return", "try", "while", "with", "yield",
    ];

    fn validate_basic(value: String) -> Result<String, ChannelNameParsingError> {
        if !(Self::MIN_LENGTH..=Self::MAX_LENGTH).contains(&value.len()) {
            return Err(ChannelNameParsingError::BadLength {
                length: value.len(),
                value,
            });
        }

        if Self::PYTHON_KEYWORDS.iter().find(|kw| **kw == value.as_str()).is_some() {
            return Err(ChannelNameParsingError::IsPythonKeyword { value });
        }
        return Ok(value);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FixedChannelName(PeggedString<1, 1023>);
impl TryFrom<String> for FixedChannelName {
    type Error = ChannelNameParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let val = ChannelNames::validate_basic(value)?;
        if let Some(unexpected_char) = val.chars().find(|c| *c == '{' || *c == '}') {
            return Err(ChannelNameParsingError::UnexpectedCharacter {
                value: val,
                char: unexpected_char,
            });
        }
        Ok(Self(val.try_into()?))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct DynamicChannelName {
    pub prefix: String,
    pub suffix: String,
}

impl Default for DynamicChannelName {
    fn default() -> Self {
        Self {
            prefix: "channel".into(),
            suffix: String::new(),
        }
    }
}

impl From<DynamicChannelName> for String {
    fn from(value: DynamicChannelName) -> Self {
        return format!("{}{{i}}{}", value.prefix, value.suffix);
    }
}

impl TryFrom<String> for DynamicChannelName {
    type Error = ChannelNameParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (prefix, suffix) = value
            .split_once("{i}")
            .ok_or(ChannelNameParsingError::BadDynamicChannelname { value: value.clone() })?;
        Ok(Self {
            prefix: prefix.into(),
            suffix: suffix.into(),
        })
    }
}

#[test]
fn test_channel_names_serialization() {
    let val = serde_json::json!("some_prefix{i}some_suffix");
    let channel_names: ChannelNames = serde_json::from_value(val).unwrap();
    match channel_names {
        ChannelNames::Dynamic(DynamicChannelName { prefix, suffix }) => {
            assert_eq!(prefix.as_str(), "some_prefix");
            assert_eq!(suffix.as_str(), "some_suffix");
        }
        bad_match => panic!("Expected dynamic channle name, found {bad_match:?}"),
    }

    let val = serde_json::json!("blas");
    let channel_names: ChannelNames = serde_json::from_value(val).unwrap();
    match channel_names {
        ChannelNames::Shorthand(FixedChannelName(name)) => {
            assert_eq!(name.as_str(), "blas")
        }
        bad_match => panic!("Expected shorthand, found {bad_match:?}"),
    }

    let val = serde_json::json!(["blas", "bles", "blis"]);
    let channel_names: ChannelNames = serde_json::from_value(val).unwrap();
    match channel_names {
        ChannelNames::Fixed(names) => {
            let expected: Vec<_> = vec![
                FixedChannelName(String::from("blas").try_into().unwrap()),
                FixedChannelName(String::from("bles").try_into().unwrap()),
                FixedChannelName(String::from("blis").try_into().unwrap()),
            ];
            assert_eq!(names, expected);
        }
        bad_match => panic!("Expected shorthand, found {bad_match:?}"),
    }
}
