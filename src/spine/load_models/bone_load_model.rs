use serde_json::Value;
use super::default_scale;

#[derive(Serialize, Deserialize, Clone)]
pub struct BoneLoadModel
{
    pub name: String,
    pub parent: Option<String>,
    pub length: Option<f64>,
    pub rotation: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub transform: Option<String>,
    #[serde(default = "default_scale")]
    pub scaleX: f64,
    #[serde(default = "default_scale")]
    pub scaleY: f64
}
