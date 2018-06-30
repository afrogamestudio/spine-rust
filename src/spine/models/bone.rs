

#[derive(Debug, Clone)]
pub struct Bone
{
    pub name: String,
    pub parent: Option<String>,
    pub length: Option<f64>,
    pub rotation: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub transform: Option<String>,
    pub scale_x: f64,
    pub scale_y: f64
}
