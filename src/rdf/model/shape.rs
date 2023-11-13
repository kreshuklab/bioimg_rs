use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Explicit(Vec<usize>),
    Parameterized { min: Vec<usize>, step: Vec<usize> },
}
