#[derive(Serialize, Deserialize, Clone)]
pub enum BlendMode {
	Normal,
	Additive,
	Multiply,
	Screen
}

impl Default for BlendMode {
    fn default() -> BlendMode { BlendMode::Normal }
}
