use std::fmt::Display;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Sigmoid;

impl Display for Sigmoid{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sigmoid")
    }
}
