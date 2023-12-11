use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Shape {
    Explicit(Vec<usize>),
    Parameterized { min: Vec<usize>, step: Vec<usize> },
}
