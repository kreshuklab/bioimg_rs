use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, PartialEq, Eq, Debug)]
pub enum AxisLabelParsingError{
    #[error("Expected 1 character, found {0}")]
    WrongNumberOfChars(usize),
    #[error("Can't parse {0} as an axis. Should be one of b, i, t, c, z, y, x")]
    BadValue(String),
}



#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
#[repr(u8)]
#[serde(into = "char")]
#[serde(try_from = "char")]
pub enum AxisLabel{
    B, 	//batch (groups multiple samples)
    I, 	//instance/index/element
    T, 	//time
    C, 	//channel
    Z, 	//spatial dimension z
    Y, 	//spatial dimension y
    X, 	//spatial dimension x
}

impl Into<char> for &AxisLabel{
    fn into(self) -> char {
        match self {
            AxisLabel::B => 'b',
            AxisLabel::I => 'i',
            AxisLabel::T => 't',
            AxisLabel::C => 'c',
            AxisLabel::Z => 'z',
            AxisLabel::Y => 'y',
            AxisLabel::X => 'x',
        }.into()
    }
}
impl Into<char> for AxisLabel{
    fn into(self) -> char {
        return (&self).into()
    }
}


impl TryFrom<char> for AxisLabel{
    type Error = AxisLabelParsingError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value{
            'b' => Ok(Self::B),
            'i' => Ok(Self::I),
            't' => Ok(Self::T),
            'c' => Ok(Self::C),
            'z' => Ok(Self::Z),
            'y' => Ok(Self::Y),
            'x' => Ok(Self::X),
            _ => Err(AxisLabelParsingError::BadValue(value.into()))
        }
    }
}

#[derive(Error, PartialEq, Eq, Debug)]
pub enum AxisSequenceParsingError{
    #[error("Bad number of axes. Expected 1 to 7, found {0}")]
    BadNumberOfAxes(usize),
    #[error("Found bad axis: {0}")]
    AxisLabelParsingError(AxisLabelParsingError)
}

impl From<AxisLabelParsingError> for AxisSequenceParsingError{
    fn from(value: AxisLabelParsingError) -> Self {
        return Self::AxisLabelParsingError(value)
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct AxisSequence(Vec<AxisLabel>);

impl Into<String> for AxisSequence{
    fn into(self) -> String {
        return self.0.iter()
            .fold(String::with_capacity(self.0.len()), |mut acc, ch| {
                acc.push(ch.into());
                acc
            })
    }
}

impl TryFrom<String> for AxisSequence{
    type Error = AxisSequenceParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 1 || value.len() > 7{
            return Err(AxisSequenceParsingError::BadNumberOfAxes(value.len()));
        }
        let labels = value.chars()
            .map(|c| AxisLabel::try_from(c))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(Self(labels))
    }
}

#[test]
fn test_axis_label_serialization(){
    let raw_axes = serde_json::Value::String("xyzib".into());
    let axis_sequence: AxisSequence = serde_json::from_value(raw_axes).unwrap();

    assert_eq!(
        axis_sequence,
        AxisSequence(vec![AxisLabel::X, AxisLabel::Y, AxisLabel::Z, AxisLabel::I, AxisLabel::B])
    )
}