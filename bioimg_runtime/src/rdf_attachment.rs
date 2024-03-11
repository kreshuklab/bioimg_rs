use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use bioimg_spec::rdf;
use uuid::Uuid;

use crate::zip_writer_ext::ModelZipWriter;
use crate::zoo_model::ModelPackingError;

pub enum RdfAttachment {
    Local(LocalRdfAttachment),
}

pub struct LocalRdfAttachment {
    #[allow(dead_code)]
    path: PathBuf,
    file: std::fs::File,
}
impl LocalRdfAttachment {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        Ok(Self { file: std::fs::File::open(&path)?, path })
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn dump(&mut self, zip_file: &mut ModelZipWriter<impl Write + Seek>) -> Result<rdf::FileReference, ModelPackingError> {
        let zip_path = format!("/{}", Uuid::new_v4());
        self.file.seek(std::io::SeekFrom::Start(0))?;
        zip_file.write_file(&zip_path, |writer| -> Result<usize, std::io::Error> {
            const READ_BUFFER_SIZE: usize = 102 * 1024;
            let mut read_buffer: Vec<u8> = vec![0; READ_BUFFER_SIZE];
            let mut total_bytes_read: usize = 0;
            loop {
                let num_read_bytes = self.file.read(&mut read_buffer)?;
                if num_read_bytes == 0 {
                    break;
                }
                total_bytes_read += num_read_bytes;
                writer.write(&read_buffer[0..num_read_bytes])?; //FIXME: check if == num_read_bytes ?
            }
            Ok(total_bytes_read)
        })?;
        Ok(rdf::FileReference::Path(PathBuf::from(zip_path)))
    }
}
