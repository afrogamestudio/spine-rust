use std::collections::HashMap;
use super::AnimationLoadModel;
use super::SkeletonMetaLoadModel;
use super::SlotLoadModel;
use super::EventLoadModel;
use super::AttachmentLoadModel;
use super::BoneLoadModel;
use super::default_events;
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
    // pub FindBone (string boneName) -> BoneData
    // {
    //     if (boneName == null) throw new ArgumentNullException("boneName", "boneName cannot be null.");
    //     var bones = this.bones;
    //     var bonesItems = bones.Items;
    //     for (int i = 0, n = bones.Count; i < n; i++) {
    //         BoneData bone = bonesItems[i];
    //         if (bone.name == boneName) return bone;
    //     }
    //     return null;
    // }

    // /// <returns>-1 if the bone was not found.</returns>
    // pub FindBoneIndex (string boneName) -> i32
    // {
    //     if (boneName == null) throw new ArgumentNullException("boneName", "boneName cannot be null.");
    //     var bones = this.bones;
    //     var bonesItems = bones.Items;
    //     for (int i = 0, n = bones.Count; i < n; i++)
    //         if (bonesItems[i].name == boneName) return i;
    //     return -1;
    // }

    // // --- Slots.

    // /// <returns>May be null.</returns>
    // pub FindSlot (string slotName) -> SlotData
    // {
    //     if (slotName == null) throw new ArgumentNullException("slotName", "slotName cannot be null.");
    //     ExposedList<SlotData> slots = this.slots;
    //     for (int i = 0, n = slots.Count; i < n; i++) {
    //         SlotData slot = slots.Items[i];
    //         if (slot.name == slotName) return slot;
    //     }
    //     return null;
    // }

    // /// <returns>-1 if the slot was not found.</returns>
    // pub FindSlotIndex (string slotName) -> i32
    // {
    //     if (slotName == null) throw new ArgumentNullException("slotName", "slotName cannot be null.");
    //     ExposedList<SlotData> slots = this.slots;
    //     for (int i = 0, n = slots.Count; i < n; i++)
    //         if (slots.Items[i].name == slotName) return i;
    //     return -1;
    // }

    // // --- Skins.

    // /// <returns>May be null.</returns>
    // pub FindSkin (string skinName) -> Skin
    // {
    //     if (skinName == null) throw new ArgumentNullException("skinName", "skinName cannot be null.");
    //     foreach (Skin skin in skins)
    //         if (skin.name == skinName) return skin;
    //     return null;
    // }

    // // --- Events.

    // /// <returns>May be null.</returns>
    // pub FindEvent (string eventDataName) -> EventData
    // {
    //     if (eventDataName == null) throw new ArgumentNullException("eventDataName", "eventDataName cannot be null.");
    //     foreach (EventData eventData in events)
    //         if (eventData.name == eventDataName) return eventData;
    //     return null;
    // }

    // // --- Animations.

    // /// <returns>May be null.</returns>
    // pub FindAnimation (string animationName) -> Animation
    // {
    //     if (animationName == null) throw new ArgumentNullException("animationName", "animationName cannot be null.");
    //     ExposedList<Animation> animations = this.animations;
    //     for (int i = 0, n = animations.Count; i < n; i++) {
    //         Animation animation = animations.Items[i];
    //         if (animation.name == animationName) return animation;
    //     }
    //     return null;
    // }

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
