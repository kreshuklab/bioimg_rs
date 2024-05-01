use crate::rdf::{model::axes::NonBatchAxisId, non_empty_list::NonEmptyList};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SimpleBinarizeDescr{
    pub threshold: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BinarizeAlongAxisDescr{
    threshold: NonEmptyList<f32>,
    axis: NonBatchAxisId,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum BinarizeDescr{
    Simple(SimpleBinarizeDescr),
    AlongAxis(BinarizeAlongAxisDescr),
}
