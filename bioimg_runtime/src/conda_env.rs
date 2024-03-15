use std::io::{Seek, Write};

use bioimg_spec::rdf;

use crate::{zip_writer_ext::ModelZipWriter, zoo_model::ModelPackingError};

#[derive(Clone)]
pub struct CondaEnv{
    pub raw: serde_yaml::Value,
}

impl CondaEnv{
    pub fn rdf_dump(
        &mut self,
        zip_file: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<rdf::EnvironmentFileDescr, ModelPackingError> {
        let zip_path = rdf::FsPath::unique_suffixed(".yml");
        zip_file.write_file(&Into::<String>::into(zip_path.clone()), |writer| {
            serde_yaml::to_writer(writer, &self.raw)
        })?;
        let file_ref = rdf::FileReference::Path(zip_path);
        Ok(rdf::FileDescription{
            source: file_ref.try_into().unwrap(),
            sha256: None //FIXME
        })
    }
}