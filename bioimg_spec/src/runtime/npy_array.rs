use ndarray_npy::ReadNpyError;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(thiserror::Error, Debug)]
pub struct UnsupportedNumpyElementType {
    path: PathBuf,
}
impl Display for UnsupportedNumpyElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't interpret npy element type from {}", self.path.to_string_lossy())
    }
}

macro_rules! impl_NpyArray_try_read {
    ($($element_type:ident),+) => {
        paste::paste! {

            #[derive(Clone)]
            pub enum NpyArray {
                $(
                    [<Array $element_type:upper>](ndarray::ArrayD<$element_type>),
                )*
            }

            impl NpyArray {
                pub fn try_read(npy_path: &Path) -> Result<Self, ReadNpyError> {
                    $(
                        match ndarray_npy::read_npy::<_, ndarray::ArrayD<$element_type>>(npy_path) {
                            Ok(arr) => return Ok(Self::[<Array $element_type:upper>](arr)),
                            Err(err) => match err {
                                ndarray_npy::ReadNpyError::WrongDescriptor(_) => (),
                                other_err => return Err(other_err),
                            },
                        };
                    )+
                    return Err(ReadNpyError::ParseData(Box::new(UnsupportedNumpyElementType{
                        path: PathBuf::from(npy_path)
                    })))

                }

                pub fn shape(&self) -> &[usize] {
                    match self {
                        $(
                            Self::[<Array $element_type:upper>](arr) => arr.shape(),
                        )*
                    }
                }
            }
        }
    };
}

impl_NpyArray_try_read!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
