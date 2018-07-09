use std::collections::HashMap;
use super::AnimationLoadModel;
use super::SkeletonMetaLoadModel;
use super::SlotLoadModel;
use super::EventLoadModel;
use super::AttachmentLoadModel;
use super::BoneLoadModel;
use super::IkConstraintLoadModel;
use super::TransformConstraintLoadModel;
use super::PathConstraintLoadModel;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonLoadModel
{
    pub skeleton: SkeletonMetaLoadModel,

	#[serde(default)]
    pub bones: Vec<BoneLoadModel>,

	#[serde(default)]
    pub slots: Vec<SlotLoadModel>,

    #[serde(default)]
    pub events: HashMap<String, EventLoadModel>,

	#[serde(default)]
    pub animations: HashMap<String, AnimationLoadModel>,

	#[serde(default)]
    pub skins: HashMap<String, HashMap<String, HashMap<String, AttachmentLoadModel>>>,

	#[serde(default)]
    pub ik_constraints: Vec<IkConstraintLoadModel>,

	#[serde(default)]
    pub transform_constraints: Vec<TransformConstraintLoadModel>,

	#[serde(default)]
    pub path_constraints: Vec<PathConstraintLoadModel>
}

impl SkeletonLoadModel
{
    pub fn find_bone (&self, bone_name: &String) -> Option<BoneLoadModel>
    {
		self.bones.iter().find(|b| b.name == *bone_name).map(|b| b.clone())
    }

    pub fn bind_bone_index (&self, bone_name: &String) -> Option<usize>
    {
		self.bones.iter().position(|b| b.name == *bone_name)
    }

    pub fn find_slot(&self, slot_name: &String) -> Option<SlotLoadModel>
    {
		self.slots.iter().find(|b| b.name == *slot_name).map(|b| b.clone())
    }

    pub fn bind_slot_index (&self, slot_name: &String) -> Option<usize>
    {
		self.slots.iter().position(|b| b.name == *slot_name)
    }

    // pub fn find_skin(&self, skin_name: &String) -> Option<SkinLoadModel>
    // {
	// 	self.skins.iter().find(|b| b.name == *skin_name).map(|b| b.clone())
    // }

    pub fn find_event(&self, event_name: &String) -> Option<EventLoadModel>
    {
		self.events.get(event_name).map(|e| e.clone())
    }

    pub fn find_animation(&self, animation_name: &String) -> Option<AnimationLoadModel>
    {
		self.animations.get(animation_name).map(|e| e.clone())
    }

    // // --- IK constraints.

    // /// <returns>May be null.</returns>
    // pub FindIkConstraint (string constraintName) -> IkConstraintData
    // {
    //     if (constraintName == null) throw new ArgumentNullException("constraintName", "constraintName cannot be null.");
    //     ExposedList<IkConstraintData> ikConstraints = this.ikConstraints;
    //     for (int i = 0, n = ikConstraints.Count; i < n; i++) {
    //         IkConstraintData ikConstraint = ikConstraints.Items[i];
    //         if (ikConstraint.name == constraintName) return ikConstraint;
    //     }
    //     return null;
    // }

    // // --- Transform constraints.

    // /// <returns>May be null.</returns>
    // pub FindTransformConstraint (string constraintName) -> TransformConstraintData
    // {
    //     if (constraintName == null) throw new ArgumentNullException("constraintName", "constraintName cannot be null.");
    //     ExposedList<TransformConstraintData> transformConstraints = this.transformConstraints;
    //     for (int i = 0, n = transformConstraints.Count; i < n; i++) {
    //         TransformConstraintData transformConstraint = transformConstraints.Items[i];
    //         if (transformConstraint.name == constraintName) return transformConstraint;
    //     }
    //     return null;
    // }

    // // --- Path constraints.

    // /// <returns>May be null.</returns>
    // pub FindPathConstraint (string constraintName) -> PathConstraintData
    // {
    //     if (constraintName == null) throw new ArgumentNullException("constraintName", "constraintName cannot be null.");
    //     ExposedList<PathConstraintData> pathConstraints = this.pathConstraints;
    //     for (int i = 0, n = pathConstraints.Count; i < n; i++) {
    //         PathConstraintData constraint = pathConstraints.Items[i];
    //         if (constraint.name.Equals(constraintName)) return constraint;
    //     }
    //     return null;
    // }

    // pub FindPathConstraintIndex (string pathConstraintName) -> i32
    // {
    //     if (pathConstraintName == null) throw new ArgumentNullException("pathConstraintName", "pathConstraintName cannot be null.");
    //     ExposedList<PathConstraintData> pathConstraints = this.pathConstraints;
    //     for (int i = 0, n = pathConstraints.Count; i < n; i++)
    //         if (pathConstraints.Items[i].name.Equals(pathConstraintName)) return i;
    //     return -1;
    // }
}

impl SkeletonLoadModel
{
    pub fn from_file(path: &String) -> Self
    {
        let manifest_data = fs::read_to_string(path).expect("Unable to open file");
        ::serde_json::from_str(&manifest_data).unwrap()
    }
}


#[cfg(test)]
mod tests {

	use super::SkeletonLoadModel;

    #[test]
    fn load_spine_json() {
        let skeleton_data = SkeletonLoadModel::from_file(&"test/trash-can.json".to_string());
    }
}
