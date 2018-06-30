

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
                parent: bone.parent,
                length: bone.length,
                rotation: bone.rotation,
                x: bone.x,
                y: bone.y,
                transform: bone.transform,
                scale_x: bone.scaleX,
                scale_y: bone.scaleY
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
        for (skin_name, slot_data) in skins {
            for (slot_name, attachment_data) in slot_data
            {
                for (attachment_name, data) in attachment_data
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

    pub fn get_bounding_boxes_at(&self, timestamp: f64) -> Vec<Polygon>
    {
        let pts = self.bounding_boxes
            .iter()
            .map(|bx| {
                let ref bone = self.bones[&bx.parent_bone];
                Polygon { position: Vector2d {
                    x: bone.x.unwrap_or(0.0),
                    y: bone.y.unwrap_or(0.0)
                }, points: (0..bx.vertices.len()/2)
                    .map(|v| Vector2d { x: bx.vertices[v*2] * bone.scale_x, y: bx.vertices[(v*2)+1] * bone.scale_y } )
                    .collect()
            }})
            .collect();
        // println!("{:?}", vec!(0..self.bounding_boxes[1].vertices.len()/2).iter().collect::<Vec<_>>());
        // println!("{:?}", self.bounding_boxes[1]);
        // println!("{:?}", pts);
        pts
    }

    pub fn get_box_ids_by_name(&self, names: Vec<String>) -> Vec<Polygon>
    {
        Vec::default()
    }
}
