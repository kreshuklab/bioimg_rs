use crate::rdf::{HttpUrl, ResourceId};

// A bioimage.io dataset resource description file (dataset RDF) describes a dataset relevant to bioimage
// processing.

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DatasetDescrEnum{
    DatasetDescr(DatasetDescr),
    LinkedDatasetDescr(LinkedDatasetDescr),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(into = "String")]
#[serde(try_from = "String")]
pub struct DatasetDescrMarker;

impl From<DatasetDescrMarker> for String{
    fn from(value: DatasetDescrMarker) -> Self {
        return "dataset".into()
    }
}

impl TryFrom<String> for DatasetDescrMarker{
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "dataset"{
            Ok(Self)
        }else{
            Err(value)
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DatasetDescr{
    #[serde(rename = "type")]
    marker: DatasetDescrMarker,
    /// URL to the source of the dataset
    #[serde(default)]
    source: Option<HttpUrl>
}



/// Reference to a bioimage.io dataset.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LinkedDatasetDescr{
    /// A valid dataset `id` from the bioimage.io collection.
    id: ResourceId
}
