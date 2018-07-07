

#[derive(Debug, Clone)]
pub struct Bone
{
    pub name: String,
    pub parent: Option<String>,
    pub length: f64,
    pub rotation: f64,
    pub x: f64,
    pub y: f64,
    pub transform: ::spine::load_models::transform_mode::TransformMode,
    pub scale_x: f64,
    pub scale_y: f64
}
