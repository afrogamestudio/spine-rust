use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorDeltaLoadModel
{
    pub time: f64,
    pub color: String,
    pub curve: Option<Value>
}
