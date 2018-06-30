pub mod animation_load_model;
pub mod attachment_delta_load_model;
pub mod attachment_load_model;
pub mod bone_animation_load_model;
pub mod bone_load_model;
pub mod colour_delta_load_model;
pub mod deformation_delta_load_model;
pub mod event_load_model;
pub mod rotation_delta_load_model;
pub mod rotation_load_model;
pub mod scale_delta_load_model;
pub mod skeleton_load_model;
pub mod skeleton_meta_load_model;
pub mod slot_animation_load_model;
pub mod slot_load_model;
pub mod translation_delta_load_model;

use self::event_load_model::EventLoadModel;
use std::collections::HashMap;
use serde_json::Value;

pub fn default_scale() -> f64
{
    1.0
}

fn default_events() -> HashMap<String, EventLoadModel>
{
    HashMap::default()
}