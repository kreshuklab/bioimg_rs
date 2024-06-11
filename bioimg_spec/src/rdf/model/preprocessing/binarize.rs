use std::fmt::Display;

use crate::rdf::{model::axes::NonBatchAxisId, non_empty_list::NonEmptyList};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SimpleBinarizeDescr{
    pub threshold: f32,
}

impl Display for SimpleBinarizeDescr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binarize (threshold: {})", self.threshold)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BinarizeAlongAxisDescr{
    pub threshold: NonEmptyList<f32>,
    pub axis: NonBatchAxisId,
}

impl Display for BinarizeAlongAxisDescr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binarize along {} (thresholds: {})", self.axis, self.threshold)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum BinarizeDescr{
    Simple(SimpleBinarizeDescr),
    AlongAxis(BinarizeAlongAxisDescr),
}

impl Display for BinarizeDescr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Simple(preproc) => preproc.fmt(f),
            Self::AlongAxis(preproc) => preproc.fmt(f),
        }
    }
}
