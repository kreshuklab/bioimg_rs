use std::{fmt::Display, sync::Arc};

pub type Result<T, E = GuiError> = std::result::Result<T, E>;

// pub trait ResultExt {
//     type Ok;
//     fn to_owned(self) -> Result<Self::Ok, GuiError>;
// }
// impl<T: Clone> ResultExt for &Result<T, GuiError> {
//     type Ok = T;
//     fn to_owned(self) -> Result<Self::Ok, GuiError> {
//         match self {
//             Ok(val) => Ok(val.clone()),
//             Err(err) => Err(err.clone()),
//         }
//     }
// }

fn blas(r: &Result<String, GuiError>) {
    let r_o = r.to_owned();
    panic!()
}

#[derive(Debug, Clone)]
pub struct GuiError(Arc<str>);

impl Display for GuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for GuiError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        Self::new(error.to_string())
    }
}

impl GuiError {
    pub fn new(message: String) -> Self {
        return Self(Arc::from(message));
    }
}
