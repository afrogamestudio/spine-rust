

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentDeltaLoadModel
{
    pub time: f64,
    pub name: Option<String>
}
