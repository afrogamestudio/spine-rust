

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttachmentLoadModel
{
    #[serde(rename = "type")]
    pub attachment_type: Option<String>,
    pub vertexCount: Option<usize>,
    pub vertices: Option<Vec<f64>>
}
