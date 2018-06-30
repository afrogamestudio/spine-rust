use super::rotation_delta_load_model::RotationDeltaLoadModel;
use super::translation_delta_load_model::TranslationDeltaLoadModel;
use super::scale_delta_load_model::ScaleDeltaLoadModel;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoneAnimationLoadModel
{
    pub rotate: Option<Vec<RotationDeltaLoadModel>>,
    pub translate: Option<Vec<TranslationDeltaLoadModel>>,
    pub scale: Option<Vec<ScaleDeltaLoadModel>>
}
