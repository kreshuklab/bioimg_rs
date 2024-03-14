use super::FileReference;


#[derive(thiserror::Error, Debug)]
pub enum CoverImageSourceParsingError{
    #[error("Path '{0}' has unrecognized suffix")]
    BadSuffix(FileReference)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CoverImageSource(FileReference);

impl TryFrom<FileReference> for CoverImageSource{
    type Error = CoverImageSourceParsingError;
    fn try_from(value: FileReference) -> Result<Self, Self::Error> {
        let filename = match &value{
            FileReference::Path(path) => path.file_name().to_lowercase(),
            FileReference::Url(url) => url.path().into(),
        };
        let filename = filename.to_lowercase();
        for extension in [".gif", ".jpeg", ".jpg", ".png", ".svg"]{
            if filename.ends_with(extension){
                return Ok(Self(value))
            }
        }
        return Err(CoverImageSourceParsingError::BadSuffix(value))
    }
}