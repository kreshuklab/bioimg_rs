#[derive(thiserror::Error, Debug)]
pub enum ConstraintsError{
    #[error("Number '{0}' fails contraint checks")]
    Failed(f32),
}

pub struct Constrained<
    const CLOSED_START: bool,
    const START: f32,
    const END: f32,
    const CLOSED_END: bool,
>(f32);

impl<
    const CLOSED_START: bool,
    const START: f32,
    const END: f32,
    const CLOSED_END: bool,
> TryFrom<f32> for Constrained<CLOSED_START, START, END, CLOSED_END>{
    type Error = ConstraintsError;
}