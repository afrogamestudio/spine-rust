use std::collections::HashMap;
use super::animation_load_model::AnimationLoadModel;
use super::skeleton_meta_load_model::SkeletonMetaLoadModel;
use super::slot_load_model::SlotLoadModel;
use super::event_load_model::EventLoadModel;
use super::attachment_load_model::AttachmentLoadModel;
use super::bone_load_model::BoneLoadModel;
use super::default_events;

#[derive(Serialize, Deserialize, Clone)]
pub struct SkeletonLoadModel
{
    pub skeleton: SkeletonMetaLoadModel,
    pub bones: Vec<BoneLoadModel>,
    pub slots: Vec<SlotLoadModel>,
    #[serde(default = "default_events")]
    pub events: HashMap<String, EventLoadModel>,
    pub animations: HashMap<String, AnimationLoadModel>,
    pub skins: HashMap<String, HashMap<String, HashMap<String, AttachmentLoadModel>>>
}
