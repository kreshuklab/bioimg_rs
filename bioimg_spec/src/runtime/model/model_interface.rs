use std::borrow::Borrow;

use paste::paste;

use crate::rdf::model as modelrdf;
use crate::rdf::model::axis_size::QualifiedAxisId;
use crate::rdf::model::AnyAxisSize;
use crate::runtime::model::axis_size_resolver::SlotResolver;
use crate::runtime::npy_array::NpyArray;

use super::axis_size_resolver::AxisSizeResolutionError;

//FIXME: these should always have resolved sizes, but spec structs don't
#[allow(dead_code)]
pub struct InputSlot<DATA: Borrow<NpyArray>> {
    descr: modelrdf::InputTensorDescr,
    test_tensor: DATA,
}

//FIXME: these should always have resolved sizes, but spec structs don't
#[allow(dead_code)]
pub struct OutputSlot<DATA: Borrow<NpyArray>> {
    descr: modelrdf::OutputTensorDescr,
    test_tensor: DATA,
}

#[derive(thiserror::Error, Debug)]
pub enum TensorValidationError {
    #[error("{0}")]
    ReadNpyError(#[from] ndarray_npy::ReadNpyError),
    #[error("Urls file references are unsupported for now")]
    UrlUnsupported,
    #[error("Test tensor with shape {test_tensor_shape:?} does not map number of reported axes ({num_described_axes})")]
    MismatchedNumDimensions {
        test_tensor_shape: Vec<usize>,
        num_described_axes: usize,
    },
    #[error("Axis #{axis_id} is incompatible with test tensor dim #{axis_index} of extent ({expected_extent})")]
    IncompatibleAxis {
        axis_id: modelrdf::AxisId,
        expected_extent: usize,
        axis_index: usize,
    },
    #[error("{0}")]
    AxisSizeResolutionError(#[from] AxisSizeResolutionError),
}

#[allow(dead_code)]
pub struct ModelInterface<DATA: Borrow<NpyArray>> {
    inputs: Vec<InputSlot<DATA>>,
    outputs: Vec<OutputSlot<DATA>>,
}

impl<DATA: Borrow<NpyArray>> ModelInterface<DATA> {
    pub fn try_build(
        mut inputs: Vec<(modelrdf::InputTensorDescr, DATA)>,
        mut outputs: Vec<(modelrdf::OutputTensorDescr, DATA)>,
    ) -> Result<Self, TensorValidationError> {
        let mut axes_sizes: Vec<(QualifiedAxisId, AnyAxisSize)> = Vec::with_capacity(inputs.len() + outputs.len());

        #[rustfmt::skip]
        macro_rules! collect_sizes {($slots:ident) => { paste! {
            for slot in $slots.iter().map(|i| &i.0) {
                for axis in slot.axes.iter() {
                    let Some(size) = axis.size() else{
                        continue;
                    };
                    let qual_id = QualifiedAxisId {
                        tensor_id: slot.id.clone(),
                        axis_id: axis.id().clone(),
                    };
                    axes_sizes.push((qual_id, size.clone()));
                }
            }
        }};}
        collect_sizes!(inputs);
        collect_sizes!(outputs);

        let size_map = SlotResolver::new(axes_sizes).solve()?;

        #[rustfmt::skip] macro_rules! resolve_and_validate {($slots:ident) => {
            for (slot, test_tensor) in $slots.iter_mut() {
                let test_tensor_shape = (*test_tensor).borrow().shape();
                let mut test_tensor_dims = test_tensor_shape.iter();
                let num_described_axes = slot.axes.len();
                for (axis_index, resolved_size) in slot.axes.resolve_sizes_with(&size_map).iter().enumerate() {
                    let Some(dim) = test_tensor_dims.next() else {
                        return Err(TensorValidationError::MismatchedNumDimensions {
                            test_tensor_shape: test_tensor_shape.into(),
                            num_described_axes,
                        });
                    };
                    let Some(resolved_size) = resolved_size else {
                        continue;
                    };
                    if !resolved_size.is_compatible_with_extent(*dim) {
                        return Err(TensorValidationError::IncompatibleAxis {
                            axis_id: slot.axes[axis_index].id().clone(), //FIXME: alternative to indexing?
                            expected_extent: *dim,
                            axis_index,
                        });
                    }
                }
            }
        }}
        resolve_and_validate!(inputs);
        resolve_and_validate!(outputs);

        Ok(Self {
            inputs: inputs
                .into_iter()
                .map(|inp| InputSlot {
                    descr: inp.0,
                    test_tensor: inp.1,
                })
                .collect(),
            outputs: outputs
                .into_iter()
                .map(|out| OutputSlot {
                    descr: out.0,
                    test_tensor: out.1,
                })
                .collect(),
        })
    }
}
