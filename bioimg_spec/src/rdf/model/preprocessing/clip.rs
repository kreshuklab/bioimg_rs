#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ClipDescr {
    pub min: f32,
    pub max: f32,
}