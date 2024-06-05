use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::rdf::{model::{postprocessing::ScaleMeanVarianceDescr, preprocessing::{BinarizeDescr, ScaleLinearDescr, ScaleRangeDescr, Zmuv}, AxisId}, FileDescription};

use super::{axes::output_axes::OutputAxisGroup, postprocessing::PostprocessingDescr, TensorId, TensorTextDescription};

#[derive(thiserror::Error, Debug)]
pub enum OutputTensorParsingError{
    #[error("Axis reference to non-existing axis")]
    PreprocessingReferencesNonExistingAxis,
}

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
    postprocessing: Vec<PostprocessingDescr>,
}

impl OutputTensorMetadata{
    pub fn axes(&self) -> &OutputAxisGroup{ &self.axes }
    pub fn postprocessing(&self) -> &Vec<PostprocessingDescr>{ &self.postprocessing }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputTensorMetadataMsg{
    pub id: TensorId,
    pub postprocessing: Vec<PostprocessingDescr>,
    #[serde(default)]
    pub description: TensorTextDescription,
    pub axes: OutputAxisGroup,
}

impl TryFrom<OutputTensorMetadataMsg> for OutputTensorMetadata{
    type Error = OutputTensorParsingError;
    fn try_from(message: OutputTensorMetadataMsg) -> Result<Self, Self::Error> {
        fn ensure_axis_exists(message: &OutputTensorMetadataMsg, preproc_axis_id: &AxisId) -> Result<(), OutputTensorParsingError>{
            message.axes.iter()
                .map(|ax| ax.id())
                .find(|ax_id| ax_id == preproc_axis_id)
                .ok_or(OutputTensorParsingError::PreprocessingReferencesNonExistingAxis)
                .map(|_| ())
        }

        for preproc in message.postprocessing.iter(){
            match preproc{
                PostprocessingDescr::Binarize(BinarizeDescr::AlongAxis(descr)) => {
                    ensure_axis_exists(&message, descr.axis.borrow())?;
                },
                PostprocessingDescr::ScaleLinear(ScaleLinearDescr::AlongAxis(descr)) => {
                    ensure_axis_exists(&message, descr.axis.borrow())?;
                },
                PostprocessingDescr::ZeroMeanUnitVariance(Zmuv{axes: Some(axes), ..}) => {
                    for preproc_axis_id in axes.iter(){
                        ensure_axis_exists(&message, preproc_axis_id)?;
                    }
                },
                PostprocessingDescr::ScaleRange(ScaleRangeDescr{axes: Some(axes), ..}) => {
                    for preproc_axis_id in axes.iter(){
                        ensure_axis_exists(&message, preproc_axis_id)?;
                    }
                },
                PostprocessingDescr::ScaleMeanVarianceDescr(ScaleMeanVarianceDescr{axes: Some(axes), ..}) => {
                    for preproc_axis_id in axes.iter(){
                        ensure_axis_exists(&message, preproc_axis_id)?;
                    }
                },
                _ => (),
            }
        }
        Ok(Self{
            id: message.id,
            postprocessing: message.postprocessing,
            description: message.description,
            axes: message.axes,
        })
    }
}

impl From<OutputTensorMetadata> for OutputTensorMetadataMsg{
    fn from(value: OutputTensorMetadata) -> Self {
        Self{
            id: value.id,
            description: value.description,
            postprocessing: value.postprocessing,
            axes: value.axes,
        }
    }
}
