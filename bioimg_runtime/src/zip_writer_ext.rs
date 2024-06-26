use std::io::{Seek, Write};

use bioimg_spec::rdf::FsPath;

use crate::zoo_model::ModelPackingError;

// Hides the ZipWriter to enforce correct usage
pub struct ModelZipWriter<W: Write + Seek>(zip::ZipWriter<W>);

impl<W: Write + Seek> ModelZipWriter<W> {
    pub fn new(zip_sink: W) -> Self {
        Self(zip::ZipWriter::new(zip_sink))
    }

    pub fn write_file<F, Out, E>(&mut self, path: &FsPath, f: F) -> Result<Out, ModelPackingError>
    where
        //FIXME: using W as a param keeps Seek, so using dyn to remove it
        F: FnOnce(&mut dyn Write) -> Result<Out, E>,
        E: Into<ModelPackingError>,
    {
        let file_options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        let path: String = path.clone().into();
        self.0.start_file(path, file_options)?;
        f(&mut self.0).map_err(|e| e.into())
    }

    //FIXME: can we enforce the calling of this function with something like must_use ?
    pub fn finish(self) -> Result<(), ModelPackingError> {
        self.0.finish()?;
        Ok(())
    }
}
