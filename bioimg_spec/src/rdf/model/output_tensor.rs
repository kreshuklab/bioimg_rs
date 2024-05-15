use serde::{Deserialize, Serialize};

use crate::rdf::FileDescription;

use super::{axes::output_axes::OutputAxisGroup, TensorId, TensorTextDescription};

#[derive(thiserror::Error, Debug)]
pub enum OutputTensorParsingError{}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputTensorDescr {
    #[serde(flatten)]
    pub metadata: OutputTensorMetadata,
    pub test_tensor: FileDescription,
    #[serde(default)]
    pub sample_tensor: Option<FileDescription>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "OutputTensorMetadataMsg")]
#[serde(into = "OutputTensorMetadataMsg")]
pub struct OutputTensorMetadata{
    pub id: TensorId,
    pub description: TensorTextDescription,
    axes: OutputAxisGroup,
    // postprocessing: FIXME
}

impl OutputTensorMetadata{
    pub fn axes(&self) -> &OutputAxisGroup{ &self.axes }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputTensorMetadataMsg{
    pub id: TensorId,
    #[serde(default)]
    pub description: TensorTextDescription,
    pub axes: OutputAxisGroup,
}

impl TryFrom<OutputTensorMetadataMsg> for OutputTensorMetadata{
    type Error = OutputTensorParsingError;
    fn try_from(value: OutputTensorMetadataMsg) -> Result<Self, Self::Error> {
        Ok(Self{
            id: value.id,
            description: value.description,
            axes: value.axes,
        })
    }
}

impl From<OutputTensorMetadata> for OutputTensorMetadataMsg{
    fn from(value: OutputTensorMetadata) -> Self {
        Self{
            id: value.id,
            description: value.description,
            axes: value.axes,
        }
    }
}