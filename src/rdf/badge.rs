use serde::{Serialize, Deserialize};

use super::file_reference::FileReference;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Badge{
    pub label: String, // (String) e.g. 'Open in Colab'
    pub icon: FileReference, // (String) e.g. 'https://colab.research.google.com/assets/colab-badge.svg'
    pub url: FileReference, // (Union[URL→URI | Path→String])
}