use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::rdf::{file_reference::FileReference, FileDescription};

use super::{axes::input_axes::InputAxisGroup, preprocessing::{BinarizeDescr, PreprocessingDescr, ScaleLinearDescr, ScaleRangeDescr, ZeroMeanUnitVariance}, AxisId, TensorId, TensorTextDescription};

#[derive(thiserror::Error, Debug)]
pub enum InputTensorParsingError{
    #[error("Axis reference to non-existing axis")]
    ReferenceToNonExistingAxis,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "InputTensorDescrMessage")]
#[serde(into = "InputTensorDescrMessage")]
pub struct InputTensorDescr {
    /// Input tensor id. No duplicates are allowed across all inputs and outputs.
    id: TensorId,

    /// indicates that this tensor is optional when doing inference
    optional: bool,

    /// Description of how this input should be preprocessed.
    ///
    /// notes:
    /// - If preprocessing does not start with an 'ensure_dtype' entry, it is added
    ///   to ensure an input tensor's data type matches the input tensor's data description.
    /// - If preprocessing does not end with an 'ensure_dtype' or 'binarize' entry, an
    ///   'ensure_dtype' step is added to ensure preprocessing steps are not unintentionally
    ///   changing the data type.
    preprocessing: Vec<PreprocessingDescr>,

    description: TensorTextDescription,
    axes: InputAxisGroup,
    test_tensor: FileDescription,
    sample_tensor: Option<FileReference>,
}

impl TryFrom<InputTensorDescrMessage> for InputTensorDescr{
    type Error = InputTensorParsingError;
    fn try_from(message: InputTensorDescrMessage) -> Result<Self, Self::Error> {

        fn ensure_axis_exists(message: &InputTensorDescrMessage, preproc_axis_id: &AxisId) -> Result<(), InputTensorParsingError>{
            message.axes.iter()
                .map(|ax| ax.id())
                .find(|ax_id| ax_id == preproc_axis_id)
                .ok_or(InputTensorParsingError::ReferenceToNonExistingAxis)
                .map(|_| ())
        }

        for preproc in message.preprocessing.iter(){
            match preproc{
                PreprocessingDescr::Binarize(BinarizeDescr::AlongAxis(descr)) => {
                    ensure_axis_exists(&message, descr.axis.borrow())?;
                },
                PreprocessingDescr::ScaleLinear(ScaleLinearDescr::AlongAxis(descr)) => {
                    ensure_axis_exists(&message, descr.axis.borrow())?;
                },
                PreprocessingDescr::ZeroMeanUnitVariance(ZeroMeanUnitVariance{axes: Some(axes), ..}) => {
                    for preproc_axis_id in axes.iter(){
                        ensure_axis_exists(&message, preproc_axis_id)?;
                    }
                },
                PreprocessingDescr::ScaleRange(ScaleRangeDescr{axes: Some(axes), ..}) => {
                    for preproc_axis_id in axes.iter(){
                        ensure_axis_exists(&message, preproc_axis_id)?;
                    }
                },
                _ => (),
            }
        }

        Ok(Self{
            id: message.id,
            optional: message.optional,
            preprocessing: message.preprocessing,
            description: message.description,
            axes: message.axes,
            test_tensor: message.test_tensor,
            sample_tensor: message.sample_tensor,
        })
    }
}

impl From<InputTensorDescr> for InputTensorDescrMessage{
    fn from(value: InputTensorDescr) -> Self {
        Self{
            id: value.id,
            optional: value.optional,
            preprocessing: value.preprocessing,
            description: value.description,
            axes: value.axes,
            test_tensor: value.test_tensor,
            sample_tensor: value.sample_tensor,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputTensorDescrMessage {
    pub id: TensorId,
    #[serde(default)]
    pub optional: bool,
    pub preprocessing: Vec<PreprocessingDescr>,
    #[serde(default)]
    pub description: TensorTextDescription,
    pub axes: InputAxisGroup,
    pub test_tensor: FileDescription,
    #[serde(default)]
    pub sample_tensor: Option<FileReference>,
}

