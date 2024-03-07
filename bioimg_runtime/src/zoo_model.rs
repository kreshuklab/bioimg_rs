use std::{
    borrow::Borrow,
    io::{Seek, Write},
    path::PathBuf,
};

use bioimg_spec::rdf::model::ModelRdf;

use crate::{zip_writer_ext::ModelZipWriter, ModelInterface, NpyArray};

#[derive(thiserror::Error, Debug)]
pub enum ModelPackingError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("File {0} already exists")]
    AlreadyExists(PathBuf),
    #[error("{0}")]
    WriteNpyError(#[from] ndarray_npy::WriteNpyError),
    #[error("{0}")]
    RdfSerializationError(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct ZooModel<DATA: Borrow<NpyArray>> {
    pub interface: ModelInterface<DATA>,
}

impl<DATA: Borrow<NpyArray>> ZooModel<DATA> {
    pub fn pack_into<Sink: Write + Seek>(&self, sink: Sink) -> Result<(), ModelPackingError> {
        let mut writer = ModelZipWriter::new(sink);

        let (inputs, outputs) = self.interface.dump(&mut writer)?;
        let model_rdf = ModelRdf { inputs, outputs };

        //FIXME: write yaml, not json
        writer.write_file("/rdf.yaml", |writer| serde_json::to_writer(writer, &model_rdf))?;

        writer.finish()?;
        Ok(())
    }
}
