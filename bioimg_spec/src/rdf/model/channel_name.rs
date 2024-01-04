use serde::{Deserialize, Serialize};

use crate::rdf::{bounded_string::BoundedStringParsingError, identifier::Identifier};


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
    BadConfigString { source: BoundedStringParsingError },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelNames {
    Dynamic(DynamicChannelName),
    Shorthand(Identifier<String>),
    Fixed(Vec<Identifier<String>>),
}

impl Default for ChannelNames {
    fn default() -> Self {
        ChannelNames::Dynamic(DynamicChannelName::default())
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
    use std::borrow::Borrow;

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
        ChannelNames::Shorthand(ident) => {
            let ident_name: &str = ident.borrow();
            assert_eq!(ident_name, "blas")
        }
        bad_match => panic!("Expected shorthand, found {bad_match:?}"),
    }

    let val = serde_json::json!(["blas", "bles", "blis"]);
    let channel_names: ChannelNames = serde_json::from_value(val).unwrap();
    match channel_names {
        ChannelNames::Fixed(names) => {
            let expected: Vec<_> = vec![
                Identifier::try_from(String::from("blas")).unwrap(),
                Identifier::try_from(String::from("bles")).unwrap(),
                Identifier::try_from(String::from("blis")).unwrap(),
            ];
            assert_eq!(names, expected);
        }
        bad_match => panic!("Expected shorthand, found {bad_match:?}"),
    }
}
