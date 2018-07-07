use super::default_scale;
use super::TransformMode;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoneLoadModel
{
    pub name: String,
    pub parent: Option<String>,
    pub length: f64,
    pub rotation: f64,
    pub x: f64,
    pub y: f64,
    pub transform: TransformMode,
    #[serde(default = "default_scale")]
    pub scale_x: f64,
    #[serde(default = "default_scale")]
    pub scale_y: f64
}
