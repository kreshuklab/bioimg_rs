use std::borrow::Borrow;
use std::collections::HashSet;
use std::io::{Seek, Write};

use bioimg_spec::rdf::non_empty_list::NonEmptyList;
use bioimg_spec::rdf;
use paste::paste;

use crate::axis_size_resolver::{ResolvedAxisSizeExt, SlotResolver};
use crate::npy_array::NpyArray;
use crate::zip_writer_ext::ModelZipWriter;
use crate::zoo_model::ModelPackingError;
use bioimg_spec::rdf::model::axis_size::QualifiedAxisId;
use bioimg_spec::rdf::model::{AnyAxisSize, InputAxis, OutputAxis};
use bioimg_spec::rdf::model::{self as modelrdf, TensorId};

use super::axis_size_resolver::AxisSizeResolutionError;

#[rustfmt::skip]
macro_rules! declare_slot { ($struct_name:ident, inout = $inout:ident) => { paste!{
    #[allow(dead_code)]
    #[derive(Clone)]
    pub struct $struct_name <DATA: Borrow<NpyArray>> {
        pub id: TensorId,
        pub description: modelrdf::TensorTextDescription,
        pub axes: modelrdf::[<$inout AxisGroup>],
        pub test_tensor: DATA,
    }

    impl<DATA: Borrow<NpyArray>> $struct_name <DATA> {
        pub fn dump(
            &self,
            zip_file: &mut ModelZipWriter<impl Write + Seek>,
        ) -> Result< modelrdf::[<$inout TensorDescr>], ModelPackingError> {
            let test_tensor_zip_path = rdf::FsPath::unique_suffixed(&format!("_{}_test_tensor.npy", self.id));
            zip_file.write_file(&test_tensor_zip_path, |writer| self.test_tensor.borrow().write_npy(writer))?;
            Ok(modelrdf::[<$inout TensorDescr>]{
                id: self.id.clone(),
                description: self.description.clone(),
                axes: self.axes.clone(),
                test_tensor: rdf::FileDescription{
                    source: test_tensor_zip_path.into(),
                    sha256: None,
                },
                sample_tensor: None, //FIXME
            })
        }
    }

    pub trait [<Vec $struct_name Ext>]{
        fn qual_id_axes(&self) -> impl Iterator<Item=(QualifiedAxisId, &[<$inout Axis>])>;
        fn qual_id_sizes(&self) -> impl Iterator<Item=(QualifiedAxisId, AnyAxisSize)>{
            self.qual_id_axes().filter_map(|(qual_id, axis)| axis.size().map(|size| (qual_id, size)))
        }
    }
    impl<DATA: Borrow<NpyArray>> [<Vec $struct_name Ext>] for [$struct_name<DATA>]{
        fn qual_id_axes(&self) -> impl Iterator<Item=(QualifiedAxisId, &[<$inout Axis>])>{
            self.iter()
                .map(|rt_tensor_descr|{
                    rt_tensor_descr.axes.iter().map(|axis|{
                        let qual_id = QualifiedAxisId{tensor_id: rt_tensor_descr.id.clone(), axis_id: axis.id()};
                        (qual_id, axis)
                    })
                })
                .flatten()
        }
    }
}};}

declare_slot!(InputSlot, inout=Input);
declare_slot!(OutputSlot, inout=Output);



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
    #[error(
        "Axis '{qualified_axis_id}' is incompatible with test tensor dim #{test_tensor_dim_index} with extent {test_tensor_dim_size}"
    )]
    IncompatibleAxis {
        qualified_axis_id: QualifiedAxisId,
        test_tensor_dim_size: usize,
        test_tensor_dim_index: usize,
    },
    #[error("{0}")]
    AxisSizeResolutionError(#[from] AxisSizeResolutionError),
    #[error("Duplicate tensor id: {0}")]
    DuplicateTensorId(TensorId),
    #[error("Empty model interface inputs")]
    EmptyInputs,
    #[error("Empty model interface outputs")]
    EmptyOutputs,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ModelInterface<DATA: Borrow<NpyArray>> {
    inputs: NonEmptyList<InputSlot<DATA>>,
    outputs: NonEmptyList<OutputSlot<DATA>>,
}

impl<DATA: Borrow<NpyArray>> ModelInterface<DATA> {
    pub fn dump(
        &self,
        zip_writer: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<
        (
            NonEmptyList<modelrdf::InputTensorDescr>,
            NonEmptyList<modelrdf::OutputTensorDescr>,
        ),
        ModelPackingError,
    > {
        let inputs = self.inputs.try_map(|inp| inp.dump(zip_writer))?;
        let outputs = self.outputs.try_map(|out| out.dump(zip_writer))?;
        Ok((inputs, outputs))
    }
    pub fn try_build(inputs: Vec<InputSlot<DATA>>, outputs: Vec<OutputSlot<DATA>>) -> Result<Self, TensorValidationError> {
        let inputs = NonEmptyList::try_from(inputs).map_err(|_| TensorValidationError::EmptyInputs)?;
        let outputs = NonEmptyList::try_from(outputs).map_err(|_| TensorValidationError::EmptyOutputs)?;

        {
            let capacity: usize = usize::from(inputs.len()) + usize::from(outputs.len());
            let mut seen_tensor_ids = HashSet::<&TensorId>::with_capacity(capacity);
            inputs.iter().map(|tensor_descr| &tensor_descr.id)
                .chain(outputs.iter().map(|tensor_descr| &tensor_descr.id))
                .map(|tensor_id|{
                    if !seen_tensor_ids.insert(tensor_id){
                        Err(TensorValidationError::DuplicateTensorId(tensor_id.clone()))
                    }else{
                        Ok(())
                    }
                })
                .collect::<Result<(), TensorValidationError>>()?;
        }

        let axis_sizes: Vec<(QualifiedAxisId, AnyAxisSize)> = inputs.qual_id_sizes()
            .chain(outputs.qual_id_sizes())
            .collect();

        let size_map = SlotResolver::new(axis_sizes)?.solve()?;

        macro_rules! validate_resolution {( $slots:ident ) => {
            for slot in $slots.iter(){
                let test_tensor_shape = slot.test_tensor.borrow().shape();
                let mut test_tensor_dims = test_tensor_shape.iter().enumerate();
                for axis in slot.axes.iter(){
                    let Some((test_tensor_dim_index, test_tensor_dim_size)) = test_tensor_dims.next() else{
                        return Err(TensorValidationError::MismatchedNumDimensions {
                            test_tensor_shape: test_tensor_shape.into(),
                            num_described_axes: slot.axes.len(),
                        });
                    };
                    if axis.size().is_none(){ // batch i guess?
                        continue;
                    };
                    let qual_id = QualifiedAxisId{tensor_id: slot.id.clone(), axis_id: axis.id()};
                    let resolved = size_map.get(&qual_id).unwrap();
                    if !resolved.is_compatible_with_extent(*test_tensor_dim_size){
                        return Err(TensorValidationError::IncompatibleAxis {
                            qualified_axis_id: qual_id,
                            test_tensor_dim_index,
                            test_tensor_dim_size: *test_tensor_dim_size,
                        });
                    }
                }
            }
        };}
        validate_resolution!(inputs);
        validate_resolution!(outputs);

        Ok(Self{inputs, outputs})
    }
}
