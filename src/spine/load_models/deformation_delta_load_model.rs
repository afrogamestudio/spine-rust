

#[derive(Serialize, Deserialize, Clone)]
pub struct DeformationDeltaLoadModel
{
    time: f64,
    offset: usize,
    vertices: Vec<f64>
}