use super::default_scale;
use super::TransformMode;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoneLoadModel
{
    pub name: String,

	#[serde(default)]
    pub parent: Option<String>,

	#[serde(default)]
    pub length: f64,

	#[serde(default)]
    pub rotation: f64,

	#[serde(default)]
    pub x: f64,

	#[serde(default)]
    pub y: f64,

	#[serde(default)]
    pub transform: TransformMode,

    #[serde(default = "default_scale")]
    pub scale_x: f64,
    
    #[serde(default = "default_scale")]
    pub scale_y: f64
}
