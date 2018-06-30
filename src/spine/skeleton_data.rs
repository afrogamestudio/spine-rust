/******************************************************************************
 * Spine Runtimes Software License v2.5
 *
 * Copyright (c) 2013-2016, Esoteric Software
 * All rights reserved.
 *
 * You are granted a perpetual, non-exclusive, non-sublicensable, and
 * non-transferable license to use, install, execute, and perform the Spine
 * Runtimes software and derivative works solely for personal or 
 * use. Without the written permission of Esoteric Software (see Section 2 of
 * the Spine Software License Agreement), you may not (a) modify, translate,
 * adapt, or develop new applications using the Spine Runtimes or otherwise
 * create derivative works or improvements of the Spine Runtimes or (b) remove,
 * delete, alter, or obscure any trademarks or any copyright, trademark, patent,
 * or other intellectual property or proprietary rights notices on or in the
 * Software, including any copy thereof. Redistributions in binary or source
 * form must include this license and terms.
 *
 * THIS SOFTWARE IS PROVIDED BY ESOTERIC SOFTWARE "AS IS" AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO
 * EVENT SHALL ESOTERIC SOFTWARE BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES, BUSINESS INTERRUPTION, OR LOSS OF
 * USE, DATA, OR PROFITS) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER
 * IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *****************************************************************************/

pub struct SkeletonData {
	name: String,
	bones: Vec<BoneData>,
	slots: Vec<SlotData>,
	skins: Vec<Skin>,
	default_skin: Skin,
	events: Vec<EventData>,
	animations: Vec<Animation>,
	ik_constraints: Vec<IkConstraintData>,
	transform_constraints: Vec<TransformConstraintData>,
	path_constraints: Vec<PathConstraintData>,
	width: f64,
	height: f64,
	version: String,
	hash: String
}

impl SkeletonData
{
	pub FindBone (string boneName) -> BoneData
	{
		if (boneName == null) throw new ArgumentNullException("boneName", "boneName cannot be null.");
		var bones = this.bones;
		var bonesItems = bones.Items;
		for (int i = 0, n = bones.Count; i < n; i++) {
			BoneData bone = bonesItems[i];
			if (bone.name == boneName) return bone;
		}
		return null;
	}

	/// <returns>-1 if the bone was not found.</returns>
	pub FindBoneIndex (string boneName) -> i32
	{
		if (boneName == null) throw new ArgumentNullException("boneName", "boneName cannot be null.");
		var bones = this.bones;
		var bonesItems = bones.Items;
		for (int i = 0, n = bones.Count; i < n; i++)
			if (bonesItems[i].name == boneName) return i;
		return -1;
	}

	// --- Slots.

	/// <returns>May be null.</returns>
	pub FindSlot (string slotName) -> SlotData
	{
		if (slotName == null) throw new ArgumentNullException("slotName", "slotName cannot be null.");
		ExposedList<SlotData> slots = this.slots;
		for (int i = 0, n = slots.Count; i < n; i++) {
			SlotData slot = slots.Items[i];
			if (slot.name == slotName) return slot;
		}
		return null;
	}

	/// <returns>-1 if the slot was not found.</returns>
	pub FindSlotIndex (string slotName) -> i32
	{
		if (slotName == null) throw new ArgumentNullException("slotName", "slotName cannot be null.");
		ExposedList<SlotData> slots = this.slots;
		for (int i = 0, n = slots.Count; i < n; i++)
			if (slots.Items[i].name == slotName) return i;
		return -1;
	}

	// --- Skins.

	/// <returns>May be null.</returns>
	pub FindSkin (string skinName) -> Skin
	{
		if (skinName == null) throw new ArgumentNullException("skinName", "skinName cannot be null.");
		foreach (Skin skin in skins)
			if (skin.name == skinName) return skin;
		return null;
	}

	// --- Events.

	/// <returns>May be null.</returns>
	pub FindEvent (string eventDataName) -> EventData
	{
		if (eventDataName == null) throw new ArgumentNullException("eventDataName", "eventDataName cannot be null.");
		foreach (EventData eventData in events)
			if (eventData.name == eventDataName) return eventData;
		return null;
	}

	// --- Animations.

	/// <returns>May be null.</returns>
	pub FindAnimation (string animationName) -> Animation
	{
		if (animationName == null) throw new ArgumentNullException("animationName", "animationName cannot be null.");
		ExposedList<Animation> animations = this.animations;
		for (int i = 0, n = animations.Count; i < n; i++) {
			Animation animation = animations.Items[i];
			if (animation.name == animationName) return animation;
		}
		return null;
	}

	// --- IK constraints.

	/// <returns>May be null.</returns>
	pub FindIkConstraint (string constraintName) -> IkConstraintData
	{
		if (constraintName == null) throw new ArgumentNullException("constraintName", "constraintName cannot be null.");
		ExposedList<IkConstraintData> ikConstraints = this.ikConstraints;
		for (int i = 0, n = ikConstraints.Count; i < n; i++) {
			IkConstraintData ikConstraint = ikConstraints.Items[i];
			if (ikConstraint.name == constraintName) return ikConstraint;
		}
		return null;
	}

	// --- Transform constraints.

	/// <returns>May be null.</returns>
	pub FindTransformConstraint (string constraintName) -> TransformConstraintData
	{
		if (constraintName == null) throw new ArgumentNullException("constraintName", "constraintName cannot be null.");
		ExposedList<TransformConstraintData> transformConstraints = this.transformConstraints;
		for (int i = 0, n = transformConstraints.Count; i < n; i++) {
			TransformConstraintData transformConstraint = transformConstraints.Items[i];
			if (transformConstraint.name == constraintName) return transformConstraint;
		}
		return null;
	}

	// --- Path constraints.

	/// <returns>May be null.</returns>
	pub FindPathConstraint (string constraintName) -> PathConstraintData
	{
		if (constraintName == null) throw new ArgumentNullException("constraintName", "constraintName cannot be null.");
		ExposedList<PathConstraintData> pathConstraints = this.pathConstraints;
		for (int i = 0, n = pathConstraints.Count; i < n; i++) {
			PathConstraintData constraint = pathConstraints.Items[i];
			if (constraint.name.Equals(constraintName)) return constraint;
		}
		return null;
	}

	pub FindPathConstraintIndex (string pathConstraintName) -> i32
	{
		if (pathConstraintName == null) throw new ArgumentNullException("pathConstraintName", "pathConstraintName cannot be null.");
		ExposedList<PathConstraintData> pathConstraints = this.pathConstraints;
		for (int i = 0, n = pathConstraints.Count; i < n; i++)
			if (pathConstraints.Items[i].name.Equals(pathConstraintName)) return i;
		return -1;
	}
}
