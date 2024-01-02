use crate::rdf::{lowercase::Lowercase, identifier::Identifier};

pub type TensorId = Lowercase<Identifier<String>>;
