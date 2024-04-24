use std::io::{Read, Seek, Write};
use std::path::Path;

use bioimg_spec::rdf as rdf;

use crate::zip_writer_ext::ModelZipWriter;
use crate::zoo_model::ModelPackingError;

pub trait FileExt{
    fn rdf_dump(
        &self, zip_file: &mut ModelZipWriter<impl Write + Seek>
    ) -> Result<rdf::FileReference, ModelPackingError>;
    fn rdf_dump_suffixed(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
        suffix: &str
    ) -> Result<rdf::FileReference, ModelPackingError>;
}

fn rdf_file_dump(
    file_path: &Path,
    zip_file: &mut ModelZipWriter<impl Write + Seek>,
    zip_path: rdf::FsPath,
) -> Result<rdf::FileReference, ModelPackingError> {
    let mut file = std::fs::File::open(file_path)?;
    zip_file.write_file(&zip_path, |writer| -> Result<usize, std::io::Error> {
        const READ_BUFFER_SIZE: usize = 16 * 1024 * 1024;
        let mut read_buffer: Vec<u8> = vec![0; READ_BUFFER_SIZE];
        let mut total_bytes_read: usize = 0;
        loop {
            let num_read_bytes = file.read(&mut read_buffer)?;
            if num_read_bytes == 0 {
                break;
            }
            total_bytes_read += num_read_bytes;
            writer.write_all(&read_buffer[0..num_read_bytes])?;
        }
        Ok(total_bytes_read)
    })?;
    Ok(rdf::FileReference::Path(zip_path))
}


impl FileExt for Path{
    fn rdf_dump_suffixed(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
        suffix: &str
    ) -> Result<rdf::FileReference, ModelPackingError> {
        rdf_file_dump(self, zip_file, rdf::FsPath::unique_suffixed(suffix))
    }

    fn rdf_dump(
        &self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<rdf::FileReference, ModelPackingError> {
        rdf_file_dump(self, zip_file, rdf::FsPath::unique())
    }
}
