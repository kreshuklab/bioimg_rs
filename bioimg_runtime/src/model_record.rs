use std::io::{Seek, Write};

pub trait ModelRecord {
    fn serialize_bynary(&self, zip_writer: &mut (impl Write + Seek)) -> Result<(), String>;
    fn serialize_rdf(&self) -> ();
}
