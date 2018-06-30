use std::collections::HashMap;
use self::super::slot_animation_load_model::SlotAnimationLoadModel;
use self::super::bone_animation_load_model::BoneAnimationLoadModel;

#[derive(Serialize, Deserialize, Clone)]
pub struct AnimationLoadModel
{
    pub slots: Option<HashMap<String, SlotAnimationLoadModel>>,
    pub bones: Option<HashMap<String, BoneAnimationLoadModel>>,
    // pub deform: HashMap<String, HashMap<String, HashMap<String, DeformationDeltaLoadModel>>>
}
