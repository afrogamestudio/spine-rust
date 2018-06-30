pub mod animation_load_model;
pub mod attachment_delta_load_model;
pub mod attachment_load_model;
pub mod bone_animation_load_model;
pub mod bone_load_model;
pub mod colour_delta_load_model;
pub mod deformation_delta_load_model;
pub mod event_load_model;
pub mod ik_constraint_load_model;
pub mod path_constraint_data;
pub mod rotation_delta_load_model;
pub mod scale_delta_load_model;
pub mod skeleton_load_model;
pub mod skeleton_meta_load_model;
pub mod slot_animation_load_model;
pub mod slot_load_model;
pub mod transform_constraint_load_model;
pub mod transform_mode;
pub mod translation_delta_load_model;
pub mod blend_mode;

pub use self::animation_load_model::AnimationLoadModel;
pub use self::attachment_delta_load_model::AttachmentDeltaLoadModel;
pub use self::attachment_load_model::AttachmentLoadModel;
pub use self::blend_mode::BlendMode;
pub use self::bone_animation_load_model::BoneAnimationLoadModel;
pub use self::bone_load_model::BoneLoadModel;
pub use self::colour_delta_load_model::ColorDeltaLoadModel;
pub use self::deformation_delta_load_model::DeformationDeltaLoadModel;
pub use self::event_load_model::EventLoadModel;
pub use self::ik_constraint_load_model::IkConstraintLoadModel;
pub use self::path_constraint_data::PathConstraintLoadModel;
pub use self::path_constraint_data::PositionMode;
pub use self::path_constraint_data::SpacingMode;
pub use self::path_constraint_data::RotateMode;
pub use self::rotation_delta_load_model::RotationDeltaLoadModel;
pub use self::scale_delta_load_model::ScaleDeltaLoadModel;
pub use self::skeleton_load_model::SkeletonLoadModel;
pub use self::skeleton_meta_load_model::SkeletonMetaLoadModel;
pub use self::slot_animation_load_model::SlotAnimationLoadModel;
pub use self::slot_load_model::SlotLoadModel;
pub use self::transform_constraint_load_model::TransformConstraintLoadModel;
pub use self::transform_mode::TransformMode;
pub use self::translation_delta_load_model::TranslationDeltaLoadModel;

use std::collections::HashMap;

pub fn default_scale() -> f64
{
    1.0
}

fn default_events() -> HashMap<String, EventLoadModel>
{
    HashMap::default()
}