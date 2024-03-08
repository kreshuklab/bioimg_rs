use std::marker::PhantomData;

pub mod rdf;
pub mod util;

// use pyo3::prelude::*;

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// /// A Python module implemented in Rust. The name of this function must match
// /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
// /// import the module.
// #[pymodule]
// fn bioimg_model_spec(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())
// }

pub trait ValidityMarker{}

pub struct Valid;
impl ValidityMarker for Valid{}

pub struct Invalid;
impl ValidityMarker for Invalid{}

pub struct Something<M: ValidityMarker>{
    a: u32,
    b: String,
    marker: PhantomData<M>,
}

impl Something<Invalid>{
    pub fn new(a: u32, b: String) -> Self{
        Self{a, b, marker: PhantomData}
    }
    pub fn try_validate(&self) -> Result<Something<Valid>, String>{
        if self.a == 123{
            Ok(Something{a: self.a, b: self.b.clone(), marker: PhantomData})
        }else{
            Err("asasd".into())
        }
    }
}