use super::BlendMode;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SlotLoadModel
{
    pub name: String,
    pub bone: String,
    pub attachment: Option<String>,
    pub color: Option<String>,
    
	#[serde(default)]
	pub blend_mode: BlendMode
}
