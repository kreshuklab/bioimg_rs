use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SpaceUnit{
    #[serde(rename = "attometer")]
    Attometer,
    #[serde(rename = "angstrom")]
    Angstrom,
    #[serde(rename = "centimeter")]
    Centimeter,
    #[serde(rename = "decimeter")]
    Decimeter,
    #[serde(rename = "exameter")]
    Exameter,
    #[serde(rename = "femtometer")]
    Femtometer,
    #[serde(rename = "foot")]
    Foot,
    #[serde(rename = "gigameter")]
    Gigameter,
    #[serde(rename = "hectometer")]
    Hectometer,
    #[serde(rename = "inch")]
    Inch,
    #[serde(rename = "kilometer")]
    Kilometer,
    #[serde(rename = "megameter")]
    Megameter,
    #[serde(rename = "meter")]
    Meter,
    #[serde(rename = "micrometer")]
    Micrometer,
    #[serde(rename = "mile")]
    Mile,
    #[serde(rename = "millimeter")]
    Millimeter,
    #[serde(rename = "nanometer")]
    Nanometer,
    #[serde(rename = "parsec")]
    Parsec,
    #[serde(rename = "petameter")]
    Petameter,
    #[serde(rename = "picometer")]
    Picometer,
    #[serde(rename = "terameter")]
    Terameter,
    #[serde(rename = "yard")]
    Yard,
    #[serde(rename = "yoctometer")]
    Yoctometer,
    #[serde(rename = "yottameter")]
    Yottameter,
    #[serde(rename = "zeptometer")]
    Zeptometer,
    #[serde(rename = "zettameter")]
    Zettameter,
}