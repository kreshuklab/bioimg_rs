use crate::rdf::model::TensorId;

use super::{PreprocessingEpsilon, PreprocessingEpsilonParsingError, _default_to_0f32, _default_to_100f32};


#[derive(thiserror::Error, Debug)]
pub enum ScaleRangeParsingError{
    #[error("Min Percentile must be in open interval [0, 100[, found {0}")]
    BadMinPercentile(f32),
    #[error("Max percentile must be in open interval ]1, 100], found {0}")]
    BadMaxPercentile(f32),
    #[error("Max percentile '{max}' not greater than min percentile '{min}'")]
    MaxNotGreaterThanMinPercentile{max: f32, min: f32},
    #[error("{0}")]
    BadEpsilon(PreprocessingEpsilonParsingError),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ScaleRangeMode {
    #[serde(rename = "per_dataset")]
    PerDataset,
    #[serde(rename = "per_sample")]
    PerSample,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(try_from = "ScaleRangePercentileMessage")]
#[serde(into = "ScaleRangePercentileMessage")]
pub struct ScaleRangePercentile{
    /// The lower percentile used for normalization.
    min_percentile: f32,

    /// """The upper percentile used for normalization
    /// Has to be bigger than `min_percentile`.
    /// The range is 1 to 100 instead of 0 to 100 to avoid mistakenly
    /// accepting percentiles specified in the range 0.0 to 1.0.
    max_percentile: f32,
}

impl TryFrom<ScaleRangePercentileMessage> for ScaleRangePercentile{
    type Error = ScaleRangeParsingError;
    fn try_from(value: ScaleRangePercentileMessage) -> Result<Self, Self::Error> {
        if value.min_percentile < 0.0 || value.min_percentile >= 100.0{
            return Err(ScaleRangeParsingError::BadMinPercentile(value.min_percentile))
        }
        if value.max_percentile <= 1.0 || value.max_percentile > 100.0{
            return Err(ScaleRangeParsingError::BadMaxPercentile(value.max_percentile))
        }
        if value.min_percentile >= value.max_percentile{
            return Err(ScaleRangeParsingError::MaxNotGreaterThanMinPercentile {
                max: value.max_percentile , min: value.min_percentile
            })
        }
        Ok(Self{min_percentile: value.min_percentile, max_percentile: value.max_percentile})
    }
}

impl From<ScaleRangePercentile> for ScaleRangePercentileMessage{
    fn from(value: ScaleRangePercentile) -> Self {
        Self{min_percentile: value.min_percentile, max_percentile: value.max_percentile}
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ScaleRangePercentileMessage{
    #[serde(default="_default_to_0f32")]
    pub min_percentile: f32,
    #[serde(default="_default_to_100f32")]
    pub max_percentile: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ScaleRangeDescr{
    /// Mode for computing percentiles.
    /// |     mode    |             description              |
    /// | ----------- | ------------------------------------ |
    /// | per_dataset | compute for the entire dataset       |
    /// | per_sample  | compute for each sample individually |
    mode: ScaleRangeMode,

    /// The subset of axes to normalize jointly.
    /// For example xy to normalize the two image axes for 2d data jointly
    // FIXME: axes: Annotated[AxesInCZYX, Field(examples=["xy"])]

    #[serde(flatten)]
    percentiles: ScaleRangePercentile,

    /// Epsilon for numeric stability.
    /// `out = (tensor - v_lower) / (v_upper - v_lower + eps)`;
    /// with `v_lower,v_upper` values at the respective percentiles.
    eps: PreprocessingEpsilon,

    // Tensor name to compute the percentiles from. Default: The tensor itself.
    // For any tensor in `inputs` only input tensor references are allowed.
    // For a tensor in `outputs` only input tensor refereences are allowed if `mode: per_dataset`
    #[serde(default)]
    reference_tensor: Option<TensorId>,
}

const fn _default_min_percentile() -> f32 {
    0f32
}

const fn _default_max_percentile() -> f32 {
    100f32
}
