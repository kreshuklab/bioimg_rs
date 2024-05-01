pub mod scale_linear;
pub mod binarize;
pub mod clip;
pub mod sigmoid;
pub mod zero_mean_unit_variance;
pub mod scale_range;

pub use self::scale_linear::ScaleLinearDescr;
pub use self::binarize::BinarizeDescr;
pub use self::clip::ClipDescr;
pub use self::sigmoid::Sigmoid;
pub use self::zero_mean_unit_variance::ZeroMeanUnitVariance;

use crate::{rdf::non_empty_list::NonEmptyList, util::SingleOrMultiple};

use super::axes::NonBatchAxisId;



// //////////////

fn _default_to_1() -> f32{
    1.0
}

fn _default_to_single_1() -> SingleOrMultiple<f32>{
    SingleOrMultiple::Single(1.0)
}

fn _default_to_single_0() -> SingleOrMultiple<f32>{
    SingleOrMultiple::Single(0.0)
}

// //////////////////

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "name", content = "kwargs")]
pub enum PreprocessingDescr {
    #[serde(rename = "binarize")]
    Binarize(BinarizeDescr),
    #[serde(rename = "clip")]
    Clip(ClipDescr),
    #[serde(rename = "scale_linear")]
    ScaleLinear(ScaleLinearDescr),
    #[serde(rename = "sigmoid")]
    Sigmoid(Sigmoid),
    #[serde(rename = "zero_mean_unit_variance")]
    ZeroMeanUnitVariance(ZeroMeanUnitVariance),
    // #[serde(rename = "scale_range")]
    // ScaleRange {
    //     mode: ScaleRangeMode,
    //     // axes: AxisSequence,
    //     #[serde(default = "_default_eps")]
    //     eps: f64,
    //     #[serde(default = "_default_max_percentile")]
    //     max_percentile: f64,
    //     #[serde(default = "_default_min_percentile")]
    //     min_percentile: f64,
    // },
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
