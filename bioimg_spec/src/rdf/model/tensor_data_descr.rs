use serde::{Deserialize, Serialize};

use crate::rdf::{literal::StrMarker, non_empty_list::NonEmptyList, si_units::SiUnit, LitStr};

use super::data_type::DataType;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TensorDataDescr {
    NominalOrOrdinal(NominalOrOrdinalDataDescr),
    IntervalOrRatio(IntervalOrRatioDataDescr),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TVs {
    Ints(NonEmptyList<i64>),
    Floats(NonEmptyList<f32>),
    Bools(NonEmptyList<bool>),
    Strings(NonEmptyList<String>),
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ArbitraryUnit;

impl StrMarker for ArbitraryUnit{
    const NAME: &'static str = "arbitrary unit";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TensorDataUnit {
    ArbitraryUnit(LitStr<ArbitraryUnit>),
    Si(SiUnit),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NominalOrOrdinalDataDescr {
    /// A fixed set of nominal or an ascending sequence of ordinal values.
    /// In this case `data_type` is required to be an unsigend integer type, e.g. 'uint8'.
    /// String `values` are interpreted as labels for tensor values 0, ..., N.
    /// Note: as YAML 1.2 does not natively support a "set" datatype,
    /// nominal values should be given as a sequence (aka list/array) as well.
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
