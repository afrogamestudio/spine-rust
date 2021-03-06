use super::attachment_delta_load_model::AttachmentDeltaLoadModel;
use super::colour_delta_load_model::ColorDeltaLoadModel;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SlotAnimationLoadModel
{
    color: Option<Vec<ColorDeltaLoadModel>>,
    attachment: Vec<AttachmentDeltaLoadModel>
}
