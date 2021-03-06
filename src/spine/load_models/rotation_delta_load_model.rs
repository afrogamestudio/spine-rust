use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RotationDeltaLoadModel
{
    pub time: f64,
    pub angle: f64,
    pub curve: Option<Value>
}
