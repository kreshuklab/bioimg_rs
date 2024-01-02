use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TimeUnit{
    #[serde(rename = "attosecond")]
    Attosecond,
    #[serde(rename = "centisecond")]
    Centisecond,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "decisecond")]
    Decisecond,
    #[serde(rename = "exasecond")]
    Exasecond,
    #[serde(rename = "femtosecond")]
    Femtosecond,
    #[serde(rename = "gigasecond")]
    Gigasecond,
    #[serde(rename = "hectosecond")]
    Hectosecond,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "kilosecond")]
    Kilosecond,
    #[serde(rename = "megasecond")]
    Megasecond,
    #[serde(rename = "microsecond")]
    Microsecond,
    #[serde(rename = "millisecond")]
    Millisecond,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "nanosecond")]
    Nanosecond,
    #[serde(rename = "petasecond")]
    Petasecond,
    #[serde(rename = "picosecond")]
    Picosecond,
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "terasecond")]
    Terasecond,
    #[serde(rename = "yoctosecond")]
    Yoctosecond,
    #[serde(rename = "yottasecond")]
    Yottasecond,
    #[serde(rename = "zeptosecond")]
    Zeptosecond,
    #[serde(rename = "zettasecond")]
    Zettasecond,
}
