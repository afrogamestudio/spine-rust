

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonMetaLoadModel
{
    pub hash: String,
    pub spine: String,
    pub width: f64,
    pub height: f64,
    pub images: String,
    pub audio: String
}
