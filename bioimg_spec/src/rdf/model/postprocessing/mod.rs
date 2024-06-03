use crate::rdf::NonEmptyList;

use super::{preprocessing::{BinarizeDescr, ClipDescr, EnsureDtype, FixedZmuv, PreprocessingEpsilon, ScaleLinearDescr, ScaleRangeDescr, Sigmoid, Zmuv}, AxisId, TensorId};


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "id", content = "kwargs")]
pub enum PostprocessingDescr {
    #[serde(rename = "binarize")]
    Binarize(BinarizeDescr),
    #[serde(rename = "clip")]
    Clip(ClipDescr),
    #[serde(rename = "ensure_dtype")]
    EnsureDtype(EnsureDtype),
    #[serde(rename = "scale_linear")]
    ScaleLinear(ScaleLinearDescr),
    #[serde(rename = "sigmoid")]
    Sigmoid(Sigmoid),
    #[serde(rename = "fixed_zero_mean_unit_variance")]
    FixedZeroMeanUnitVariance(FixedZmuv),
    #[serde(rename = "zero_mean_unit_variance")]
    ZeroMeanUnitVariance(Zmuv),
    #[serde(rename = "scale_range")]
    ScaleRange(ScaleRangeDescr),
    #[serde(rename = "scale_mean_variance")]
    ScaleMeanVarianceDescr(ScaleMeanVarianceDescr),
}

/// Scale a tensor's data distribution to match another tensor's mean/std.
/// `out  = (tensor - mean) / (std + eps) * (ref_std + eps) + ref_mean.`
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ScaleMeanVarianceDescr{
    /// Name of tensor to match.
    pub reference_tensor: TensorId,

    /// The subset of axes to normalize jointly, i.e. axes to reduce to compute mean/std.
    /// For example to normalize 'batch', 'x' and 'y' jointly in a tensor ('batch', 'channel', 'y', 'x')
    /// resulting in a tensor of equal shape normalized per channel, specify `axes=('batch', 'x', 'y')`.
    /// To normalize samples independently, leave out the 'batch' axis.
    /// default: Scale all axes jointly.
    pub axes: Option<NonEmptyList<AxisId>>,

    /// Epsilon for numeric stability:
    /// `out  = (tensor - mean) / (std + eps) * (ref_std + eps) + ref_mean.`
    pub eps: PreprocessingEpsilon,

}
