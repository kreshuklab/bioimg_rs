use crate::rdf::{model::{axes::NonBatchAxisId, AxisId}, non_empty_list::NonEmptyList};

#[derive(thiserror::Error, Debug)]
pub enum ZeroMeanUnitvarianceParsingError{
    #[error("Standard deviation must be greater or equal to 1e-6. Provided {0}")]
    BadStandardDeviation(f32),
    #[error("std and mean must have the same lengths")]
    MismatchedStdAndMean,
    #[error("Empty list")] //FIXME: this should never happen if using NonEmptyList
    EmptyList,
}

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Debug)]
pub struct ZeroMeanUnitVarianceStdDeviation(f32);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ZeroMeanUnitVariance {
    /// The subset of axes to normalize jointly, i.e. axes to reduce to compute mean/std.
    /// For example to normalize 'batch', 'x' and 'y' jointly in a tensor ('batch', 'channel', 'y', 'x')
    /// resulting in a tensor of equal shape normalized per channel, specify `axes=('batch', 'x', 'y')`.
    /// To normalize each sample independently leave out the 'batch' axis.
    /// Default: Scale all axes jointly.
    pub axes: Option<NonEmptyList<AxisId>>,

    /// epsilon for numeric stability: `out = (tensor - mean) / (std + eps)`.
    #[serde(default = "_default_eps")]
    pub eps: f32,
}

const fn _default_eps() -> f32 {
    1e-6
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum FixedZeroMeanUnitVariance{
    Simple(SimpleFixedZeroMeanUnitVariance),
    AlongAxis(FixedZeroMeanUnitVarianceAlongAxis)
}

impl TryFrom<f32> for ZeroMeanUnitVarianceStdDeviation{
    type Error = ZeroMeanUnitvarianceParsingError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 1e-6{
            return Err(ZeroMeanUnitvarianceParsingError::BadStandardDeviation(value))
        }
        Ok(Self(value))
    }
}

///Normalize with fixed, precomputed values for mean and variance.
///See `zero_mean_unit_variance` for data dependent normalization.
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct SimpleFixedZeroMeanUnitVariance{
    ///The mean value to normalize with.
    mean: f32,
    ///The standard deviation value to normalize with.
    std: ZeroMeanUnitVarianceStdDeviation,
}


// Normalize with fixed, precomputed values for mean and variance.
// See `zero_mean_unit_variance` for data dependent normalization.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(try_from = "FixedZeroMeanUnitVarianceAlongAxisMsg")]
#[serde(into = "FixedZeroMeanUnitVarianceAlongAxisMsg")]
pub struct FixedZeroMeanUnitVarianceAlongAxis{
    pub mean_and_std: NonEmptyList<SimpleFixedZeroMeanUnitVariance>,
    pub axis: NonBatchAxisId,
}

impl TryFrom<FixedZeroMeanUnitVarianceAlongAxisMsg> for FixedZeroMeanUnitVarianceAlongAxis{
    type Error = ZeroMeanUnitvarianceParsingError;
    fn try_from(value: FixedZeroMeanUnitVarianceAlongAxisMsg) -> Result<Self, Self::Error> {
        if value.mean.len() != value.std.len(){
            return Err(ZeroMeanUnitvarianceParsingError::MismatchedStdAndMean)
        }
        return Ok(Self{
            mean_and_std: value.mean.into_inner().into_iter()
                .zip(value.std.into_inner().into_iter())
                .map(|(mean, std)| SimpleFixedZeroMeanUnitVariance{mean, std})
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| ZeroMeanUnitvarianceParsingError::EmptyList)?,
            axis: value.axis,
        })
    }
}

impl From<FixedZeroMeanUnitVarianceAlongAxis> for FixedZeroMeanUnitVarianceAlongAxisMsg{
    fn from(value: FixedZeroMeanUnitVarianceAlongAxis) -> Self {
        Self{
            mean: value.mean_and_std.map(|mean_and_std| mean_and_std.mean),
            std: value.mean_and_std.map(|mean_and_std| mean_and_std.std),
            axis: value.axis,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct FixedZeroMeanUnitVarianceAlongAxisMsg{
    /// The mean value(s) to normalize with.
    mean: NonEmptyList<f32>,
    /// The standard deviation value(s) to normalize with.
    /// Size must match `mean` values.
    std: NonEmptyList<ZeroMeanUnitVarianceStdDeviation>,
    /// The axis of the mean/std values to normalize each entry along that dimension
    /// separately.
    axis: NonBatchAxisId,
}
