use super::default_scale;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoneLoadModel
{
    pub name: String,
    pub parent: Option<String>,
    pub length: Option<f64>,
    pub rotation: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub transform: Option<TransformMode>,
    #[serde(default = "default_scale")]
    pub scale_x: f64,
    #[serde(default = "default_scale")]
    pub scale_y: f64
}

// [Flags]
pub enum TransformMode
{
	//0000 0 Flip Scale Rotation
	Normal = 0, // 0000
	OnlyTranslation = 7, // 0111
	NoRotationOrReflection = 1, // 0001
	NoScale = 2, // 0010
	NoScaleOrReflection = 6, // 0110
}