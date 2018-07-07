use super::super::load_models::SkeletonLoadModel;
use super::BoundingBox;
use super::Bone;
use super::Slot;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Skeleton
{
    bounding_boxes: Vec<BoundingBox>,
    bones: HashMap<String, Bone>,
    slots: HashMap<String, Slot>,
    bones_by_hierarchy: HashMap<String, Vec<Bone>>
    // slots_by_attachment: HashMap<String, Slot>
}


impl Skeleton
{
    pub fn from_file(path: &String) -> Self
    {
        let manifest_data = fs::read_to_string(path).expect("Unable to open file");
        let skeleton_manifest: SkeletonLoadModel = ::serde_json::from_str(&manifest_data).unwrap();
        Self::new(skeleton_manifest)
    }

    pub fn new(skeleton_manifest: SkeletonLoadModel) -> Self
    {
        let SkeletonLoadModel { bones, skins, slots, .. } = skeleton_manifest;
        
        let bones: HashMap<_,_> = bones
            .into_iter()
            .map(|bone| Bone {
                name: bone.name,
                parent: None,
                length: bone.length,
                rotation: bone.rotation,
                x: bone.x,
                y: bone.y,
                transform: bone.transform,
                scale_x: bone.scale_x,
                scale_y: bone.scale_y
            })
            .map(|bone| (bone.name.clone(), bone))
            .collect();

        let slots_by_name: HashMap<String, Slot> = slots
            .into_iter()
            .map(|slm| Slot {
                name: slm.name,
                bone: slm.bone,
                attachment: slm.attachment,
                color: slm.color
            })
            .map(|slot| (slot.name.clone(), slot))
            .collect();

        let mut bounding_boxes : Vec<BoundingBox> = Vec::default();
        for (_skin_name, slot_data) in skins {
            for (slot_name, attachment_data) in slot_data
            {
                for (_attachment_name, data) in attachment_data
                {
                    if data.attachment_type.iter().filter(|&attachment_type|attachment_type == "boundingbox").count() == 1
                    {
                        let ref parent_slot = slots_by_name[&slot_name];
                        bounding_boxes.push(BoundingBox {
                            // slot_name,
                            // attachment_name,
                            parent_bone: parent_slot.bone.clone(),
                            vertices: data.vertices.unwrap_or(Vec::default())
                        });
                    }
                }
            }
        }

        let bones_by_hierarchy: HashMap<String, Vec<Bone>> = bones
            .iter()
            .map(|(key, value)| (key.clone(), build_hierarchy_for(value, &bones)))
            .collect();

        Skeleton {
            bones,
            slots: slots_by_name,
            bounding_boxes,
            bones_by_hierarchy
        }
    }

    pub fn get_bounding_boxes_at(&self, _timestamp: f64) -> Vec<WorldBoundingBox>
    {
        let pts = self.bounding_boxes
            .iter()
            .map(|bx| {
                let ref bone = self.bones[&bx.parent_bone];
                let world_vertices = bx
                    .vertices
                    .iter()
                    .enumerate()
                    .map(|(v, i)| v as f64 * (match (*i as i32) % 2 { 0 => bone.scale_x, _ => bone.scale_y }))
                    .collect();
                WorldBoundingBox {
                    world_vertices
                }
                // Polygon { position: Vector2d {
                //     x: bone.x.unwrap_or(0.0),
                //     y: bone.y.unwrap_or(0.0)
                // }, points: (0..bx.vertices.len()/2)
                //     .map(|v| Vector2d { x: bx.vertices[v*2] * bone.scale_x, y: bx.vertices[(v*2)+1] * bone.scale_y } )
                //     .collect()
            })
            .collect();
        pts
    }
}

pub struct WorldBoundingBox
{
    pub world_vertices: Vec<f64>
}

pub fn build_hierarchy_for(bone: &Bone, bones: &HashMap<String, Bone>) -> Vec<Bone>
{
    match bone.parent.clone()
    {
        Some(parent) => {
            let ref parent_bone = bones[&parent];
            let parents = build_hierarchy_for(parent_bone, bones);
            //combine_vectors(vec![parent_bone.clone()], parents)
            vec![parent_bone.clone()].into_iter().chain(parents).collect()
        },
        None => Vec::default()
    }
}
