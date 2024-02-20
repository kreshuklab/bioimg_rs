use std::borrow::Borrow;

use crate::rdf;
use crate::rdf::model as modelrdf;
use crate::rdf::non_empty_list::NonEmptyList;
use crate::rdf::BoundedString;
use crate::runtime::npy_array::NpyArray;

pub struct InputTensor<DATA: Borrow<NpyArray>> {
    pub id: modelrdf::TensorId,
    pub description: BoundedString<0, 128>,
    pub axes: NonEmptyList<modelrdf::InputAxis>,
    pub test_tensor: DATA,
    // pub sample_tensor: Option<FileReference>, //FIXME: add this back
}

#[derive(thiserror::Error, Debug)]
pub enum InputTensorValidationError {
    #[error("{0}")]
    ReadNpyError(#[from] ndarray_npy::ReadNpyError),
    #[error("Urls file references are unsupported for now")]
    UrlUnsupported,
    #[error("Test tensor with shape {test_tensor_shape:?} does not map number of reported axes ({num_axes})")]
    MismatchedNumDimensions { test_tensor_shape: Vec<usize>, num_axes: usize },
    #[error("Axis #{axis_id} is incompatible with test tensor dim #{axis_index} of extent ({expected_extent})")]
    IncompatibleAxis {
        axis_id: modelrdf::AxisId,
        expected_extent: usize,
        axis_index: usize,
    },
}

impl<DATA: Borrow<NpyArray>> InputTensor<DATA> {
    pub fn new(
        id: modelrdf::TensorId,
        description: BoundedString<0, 128>,
        axes: NonEmptyList<modelrdf::InputAxis>,
        test_tensor: DATA,
    ) -> Result<Self, InputTensorValidationError> {
        let num_axes = axes.len();
        let num_test_dims = test_tensor.borrow().shape().len();
        if num_axes != num_test_dims {
            return Err(InputTensorValidationError::MismatchedNumDimensions {
                test_tensor_shape: test_tensor.borrow().shape().to_owned(),
                num_axes,
            });
        }

        for (idx, (expected_extent, axis)) in test_tensor.borrow().shape().iter().zip(axes.iter()).enumerate() {
            if !axis.is_compatible_with_extent(*expected_extent) {
                let axis_id = axis.id().clone();
                return Err(InputTensorValidationError::IncompatibleAxis {
                    axis_id,
                    axis_index: idx,
                    expected_extent: *expected_extent,
                });
            }
        }

        Ok(InputTensor {
            id,
            description,
            test_tensor,
            axes,
        })
    }
}

impl InputTensor<NpyArray> {
    pub fn try_load(descr: &modelrdf::InputTensorDescr) -> Result<Self, InputTensorValidationError> {
        let test_tensor_data = match &descr.test_tensor {
            rdf::FileReference::Path(path) => NpyArray::try_read(&path)?,
            rdf::FileReference::Url(_) => return Err(InputTensorValidationError::UrlUnsupported),
        };
        return Self::new(
            descr.id.clone(),
            descr.description.clone(),
            descr.axes.clone(),
            test_tensor_data,
        );
    }
}
