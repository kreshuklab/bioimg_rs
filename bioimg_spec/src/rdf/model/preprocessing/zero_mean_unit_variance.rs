use crate::rdf::{model::AxisId, non_empty_list::NonEmptyList};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "mode")]
pub enum ZeroMeanUnitVariance {
    #[serde(rename = "fixed")]
    Fixed {
        axes: Option<NonEmptyList<AxisId>>,
        #[serde(default = "_default_eps")]
        eps: f32,
        mean: Vec<f32>,
        std: Vec<f32>,
    },
    #[serde(rename = "per_dataset")]
    PerDataset {
        axes: Option<NonEmptyList<AxisId>>,
        #[serde(default = "_default_eps")]
        eps: f32,
    },
    #[serde(rename = "per_sample")]
    PerSample {
        axes: Option<NonEmptyList<AxisId>>,
        #[serde(default = "_default_eps")]
        eps: f32,
    },
}

const fn _default_eps() -> f32 {
    10E-6
}

#[test]
fn test_per_dataset_serialization(){
    let value = ZeroMeanUnitVariance::PerDataset { eps: 1.0, axes: None };
    let serde_json::Value::Object(map) = serde_json::to_value(value).unwrap() else {
        panic!("expected json object to be produced")
    };
    assert_eq!(map.get("mode").unwrap(), "per_dataset");
}
