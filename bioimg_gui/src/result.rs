use std::{fmt::Display, sync::Arc};

pub type Result<T, E = GuiError> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct GuiError{
    message: Arc<str>,
    pub failed_widget_rect: Option<egui::Rect>,
}

impl Display for GuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl<E> From<E> for GuiError
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        Self::new(error.to_string())
    }
}

impl GuiError {
    pub fn new<S: AsRef<str>>(message: S) -> Self {
        return Self{ message: Arc::from(message.as_ref()), failed_widget_rect: None };
    }
    pub fn new_with_rect<S: AsRef<str>>(message: S, failed_widget_rect: Option<egui::Rect>) -> Self {
        return Self{ message: Arc::from(message.as_ref()), failed_widget_rect };
    }
}

pub trait VecResultExt{
    type Item;
    fn collect_result(self) -> Result<Vec<Self::Item>>;
}
impl<T> VecResultExt for Vec<Result<T>>{
    type Item = T;
    fn collect_result(self) -> Result<Vec<Self::Item>>{
        self.into_iter().collect()
    }
}
impl<'a, T> VecResultExt for Vec<&'a Result<T>>{
    type Item = &'a T;
    fn collect_result(self) -> Result<Vec<Self::Item>>{
        self.iter().map(|it| match it{
            Ok(val) => Ok(val),
            Err(err) => Err(err.clone()) 
        }).collect()
    }
}
