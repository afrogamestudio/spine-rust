

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeformationDeltaLoadModel
{
    time: f64,
    offset: usize,
    vertices: Vec<f64>
}