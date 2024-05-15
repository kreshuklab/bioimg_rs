use crate::rdf::model::data_type::DataType;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct EnsureDtype{
    pub dtype: DataType
}