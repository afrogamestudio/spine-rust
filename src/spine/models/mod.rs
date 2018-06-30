pub mod skeleton_load_models;

use impi::spine::skeleton_load_models::AttachmentLoadModel;
use impi::spine::skeleton_load_models::SkeletonLoadModel;
use impi::world::physics::sat::Vector2d;
use impi::world::physics::sat::Polygon;
use std::collections::HashMap;
use impi::world::physics::sat::combine_vectors;
use std::fs;

pub fn build_hierarchy_for(bone: &Bone, bones: &HashMap<String, Bone>) -> Vec<Bone>
{
    match bone.parent.clone()
    {
        Some(parent) => {
            let ref parent_bone = bones[&parent];
            let parents = build_hierarchy_for(parent_bone, bones);
            combine_vectors(vec![parent_bone.clone()], parents)
        },
        None => Vec::default()
    }
}
