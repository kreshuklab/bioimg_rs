use ndarray_npy::{ReadNpyError, WriteNpyExt, ReadNpyExt};
use std::{
    io::{Read, Seek},
    fmt::Display,
    sync::Arc,
};

#[derive(thiserror::Error, Debug)]
pub struct UnsupportedNumpyElementType;

impl Display for UnsupportedNumpyElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't interpret npy element type")
    }
}

#[rustfmt::skip]
macro_rules! impl_NpyArray_try_read {( $($element_type:ident),+ ) => { paste::paste! {
    #[derive(Clone)]
    pub enum NpyArray {$(
        [<Array $element_type:upper>](ndarray::ArrayD<$element_type>),
    )*}

    impl NpyArray {
        pub fn try_load(mut reader: impl Read) -> Result<Self, ReadNpyError> {
            let mut data = vec![];
            reader.read_to_end(&mut data)?; //FIXME: what if too big?
            let mut cursor = std::io::Cursor::new(data);
            $(
                cursor.rewind()?;
                match ndarray::ArrayD::<$element_type>::read_npy(&mut cursor) {
                    Ok(arr) => return Ok(Self::[<Array $element_type:upper>](arr)),
                    Err(err) => match err {
                        ndarray_npy::ReadNpyError::WrongDescriptor(_) => (),
                        other_err => return Err(other_err),
                    },
                };
            )+
            return Err(ReadNpyError::ParseData(Box::new(UnsupportedNumpyElementType)))
        }

        pub fn write_npy<W: std::io::Write>(&self, writer: W) -> Result<(), ndarray_npy::WriteNpyError>{
            match self {$(
                Self::[<Array $element_type:upper>](arr) => arr.write_npy(writer),
            )*}
        }

        pub fn shape(&self) -> &[usize] {
            match self {$(
                Self::[<Array $element_type:upper>](arr) => arr.shape(),
            )*}
        }
    }
}};}

impl_NpyArray_try_read!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

pub type ArcNpyArray = Arc<NpyArray>;
