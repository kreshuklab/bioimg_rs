use crate::rdf::{model::AxisId, non_empty_list::NonEmptyList};

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