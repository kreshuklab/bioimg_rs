use serde::{Deserialize, Serialize};

use crate::rdf::{non_empty_list::NonEmptyList, si_units::SiUnit};

use super::data_type::DataType;

pub enum TensorDataDescr {
    NominalOrOrdinal,
    IntervalOrRatio,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TVs {
    Ints(NonEmptyList<i64>),
    Floats(NonEmptyList<f32>),
    Bools(NonEmptyList<bool>),
    Strings(NonEmptyList<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TensorDataUnit {
    ArbitraryUnit,
    Si(SiUnit),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NominalOrOrdinalDataDescr {
    pub values: TVs,
    #[serde(rename = "type")]
    #[serde(default = "_default_data_type")]
    pub data_type: DataType,
    #[serde(default)]
    pub unit: Option<TensorDataUnit>,
}

fn _default_data_type() -> DataType {
    DataType::Uint8
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntervalOrRatioDataDescr {
    #[serde(rename = "type")]
    data_type: DataType,
    range: (Option<f32>, Option<f32>),
    unit: TensorDataUnit,
    #[serde(default = "_default_scale")]
    scale: f32,
    #[serde(default)]
    offset: Option<f32>,
}

fn _default_scale() -> f32 {
    1.0
}
