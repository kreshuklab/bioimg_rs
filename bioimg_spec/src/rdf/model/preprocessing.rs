// use super::axes::AxisSequence;
use serde::{Deserialize, Serialize};

use crate::util::SingleOrMultiple;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", content = "kwargs")]
pub enum Preprocessing {
    #[serde(rename = "binarize")]
    Binarize { threshold: f64 },
    #[serde(rename = "clip")]
    Clip { min: f64, max: f64 },
    #[serde(rename = "scale_linear")]
    ScaleLinear {
        // axes: AxisSequence,
        gain: SingleOrMultiple<f64>,
        offset: SingleOrMultiple<f64>,
    },
    #[serde(rename = "scale_range")]
    ScaleRange {
        mode: ScaleRangeMode,
        // axes: AxisSequence,
        #[serde(default = "_default_eps")]
        eps: f64,
        #[serde(default = "_default_max_percentile")]
        max_percentile: f64,
        #[serde(default = "_default_min_percentile")]
        min_percentile: f64,
    },
    #[serde(rename = "sigmoid")]
    Sigmoid,
    #[serde(rename = "zero_mean_unit_variance")]
    ZeroMeanUnitVariance(ZeroMeanUnitVariance),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "mode")]
pub enum ZeroMeanUnitVariance {
    #[serde(rename = "fixed")]
    Fixed {
        // axes: AxisSequence,
        #[serde(default = "_default_eps")]
        eps: f64,
        mean: Vec<f64>,
        std: Vec<f64>,
    },
    #[serde(rename = "per_dataset")]
    PerDataset {
        // axes: AxisSequence,
        #[serde(default = "_default_eps")]
        eps: f64,
    },
    #[serde(rename = "per_sample")]
    PerSample {
        // axes: AxisSequence,
        #[serde(default = "_default_eps")]
        eps: f64,
    },
}

const fn _default_eps() -> f64 {
    10E-6
}

const fn _default_min_percentile() -> f64 {
    0f64
}

const fn _default_max_percentile() -> f64 {
    100f64
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ScaleRangeMode {
    #[serde(rename = "per_dataset")]
    PerDataset,
    #[serde(rename = "per_sample")]
    PerSample,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ZeroMeanUnitVarianceMode {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "per_dataset")]
    PerDataset,
    #[serde(rename = "per_sample")]
    PerSample,
}
