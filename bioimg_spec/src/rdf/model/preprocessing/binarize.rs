use crate::rdf::{model::axes::NonBatchAxisId, non_empty_list::NonEmptyList};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SimpleBinarizeDescr{
    pub threshold: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BinarizeAlongAxisDescr{
    pub threshold: NonEmptyList<f32>,
    pub axis: NonBatchAxisId,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum BinarizeDescr{
    Simple(SimpleBinarizeDescr),
    AlongAxis(BinarizeAlongAxisDescr),
}
