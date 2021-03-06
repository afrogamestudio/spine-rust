use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TranslationDeltaLoadModel
{
    pub time: f64,
    pub x: f64,
    pub y: f64,
    pub curve: Option<Value>
}
