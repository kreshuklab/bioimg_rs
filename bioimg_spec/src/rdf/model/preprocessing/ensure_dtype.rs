use std::fmt::Display;

use crate::rdf::model::data_type::DataType;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct EnsureDtype{
    pub dtype: DataType
}

impl Display for EnsureDtype{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ensure {}", self.dtype)
    }
}
