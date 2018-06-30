use super::Timeline;

pub struct Animation {
    pub name: String,
    pub duration: f64,
    pub timelines: Vec<Timeline>
}

impl Animation
{
// 	pub Animation (string name, ExposedList<Timeline> timelines, f64 duration) {
// 		if (name == null) throw new ArgumentNullException("name", "name cannot be null.");
// 		if (timelines == null) throw new ArgumentNullException("timelines", "timelines cannot be null.");
// 		this.name = name;
// 		this.timelines = timelines;
// 		this.duration = duration;
// 	}

// 	/// <summary>Applies all the animation's timelines to the specified skeleton.</summary>
// 	/// <seealso cref="Timeline.Apply(Skeleton, f64, f64, ExposedList, f64, MixPose, MixDirection)"/>
// 	pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, bool loop, ExposedList<Event> events, f64 alpha, MixPose pose, MixDirection direction) {
// 		if (skeleton == null) throw new ArgumentNullException("skeleton", "skeleton cannot be null.");

// 		if (loop && duration != 0) {
// 			time %= duration;
// 			if (lastTime > 0) lastTime %= duration;
// 		}

// 		ExposedList<Timeline> timelines = this.timelines;
// 		for (int i = 0, n = timelines.Count; i < n; i++)
// 			timelines.Items[i].Apply(skeleton, lastTime, time, events, alpha, pose, direction);
// 	}

// 	/// <param name="target">After the first and before the last entry.</param>
// 		static int BinarySearch (f64[] values, f64 target, int step) {
// 		int low = 0;
// 		int high = values.Length / step - 2;
// 		if (high == 0) return step;
// 		int current = (int)((uint)high >> 1);
// 		while (true) {
// 			if (values[(current + 1) * step] <= target)
// 				low = current + 1;
// 			else
// 				high = current;
// 			if (low == high) return (low + 1) * step;
// 			current = (int)((uint)(low + high) >> 1);
// 		}
// 	}

// 	/// <param name="target">After the first and before the last entry.</param>
// 		static int BinarySearch (f64[] values, f64 target) {
// 		int low = 0;
// 		int high = values.Length - 2;
// 		if (high == 0) return 1;
// 		int current = (int)((uint)high >> 1);
// 		while (true) {
// 			if (values[(current + 1)] <= target)
// 				low = current + 1;
// 			else
// 				high = current;
// 			if (low == high) return (low + 1);
// 			current = (int)((uint)(low + high) >> 1);
// 		}
// 	}

// 		static int LinearSearch (f64[] values, f64 target, int step) {
// 		for (int i = 0, last = values.Length - step; i <= last; i += step)
// 			if (values[i] > target) return i;
// 		return -1;
// 	}
// }

// pub interface Timeline {
// 	/// <summary>Sets the value(s) for the specified time.</summary>
// 	/// <param name="skeleton">The skeleton the timeline is being applied to. This provides access to the bones, slots, and other skeleton components the timeline may change.</param>
// 	/// <param name="lastTime">lastTime The time this timeline was last applied. Timelines such as EventTimeline trigger only at specific times rather than every frame. In that case, the timeline triggers everything between lastTime (exclusive) and <code>time</code> (inclusive).</param>
// 	/// <param name="time">The time within the animation. Most timelines find the key before and the key after this time so they can interpolate between the keys.</param>
// 	/// <param name="events">If any events are fired, they are added to this list. Can be null to ignore firing events or if the timeline does not fire events. May be null.</param>
// 	/// <param name="alpha">alpha 0 applies the current or setup pose value (depending on pose parameter). 1 applies the timeline 
// 	/// 	value. Between 0 and 1 applies a value between the current or setup pose and the timeline value. By adjusting
// 	/// 	alpha over time, an animation can be mixed in or out. <code>alpha</code> can also be useful to
// 	/// 	 apply animations on top of each other (layered).</param>
// 	/// <param name="pose">Controls how mixing is applied when alpha is than 1.</param>
// 	/// <param name="direction">Indicates whether the timeline is mixing in or out. Used by timelines which perform instant transitions such as DrawOrderTimeline and AttachmentTimeline.</param>
// 	void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> events, f64 alpha, MixPose pose, MixDirection direction);
// 	int PropertyId { get; }
// }

// /// <summary>
// /// Controls how a timeline is mixed with the setup or current pose.</summary>
// /// <seealso cref="Timeline.Apply(Skeleton, f64, f64, ExposedList, f64, MixPose, MixDirection)"/>
// pub enum MixPose {
// 	/// <summary> The timeline value is mixed with the setup pose (the current pose is not used).</summary>
// 	Setup,
// 	/// <summary> The timeline value is mixed with the current pose. The setup pose is used as the timeline value before the first key,
// 	/// except for timelines which perform instant transitions, such as DrawOrderTimeline or AttachmentTimeline.</summary>
// 	Current,
// 	/// <summary> The timeline value is mixed with the current pose. No change is made before the first key (the current pose is kept until the first key).</summary>
// 	CurrentLayered
// }

// /// <summary>
// /// Indicates whether a timeline's <code>alpha</code> is mixing out over time toward 0 (the setup or current pose) or mixing in toward 1 (the timeline's pose).</summary>
// /// <seealso cref="Timeline.Apply(Skeleton, f64, f64, ExposedList, f64, MixPose, MixDirection)"/>
// pub enum MixDirection {
// 	In,
// 	Out
// }

// 	enum TimelineType {
// 	Rotate = 0, Translate, Scale, Shear, //
// 	Attachment, Color, Deform, //
// 	Event, DrawOrder, //
// 	IkConstraint, TransformConstraint, //
// 	PathConstraintPosition, PathConstraintSpacing, PathConstraintMix, //
// 	TwoColor
// }

// /// <summary>Base struct for frames that use an interpolation bezier curve.</summary>
// abstract pub struct CurveTimeline : Timeline {
// 	protected const f64 LINEAR = 0, STEPPED = 1, BEZIER = 2;
// 	protected const int BEZIER_SIZE = 10 * 2 - 1;

// 		f64[] curves; // type, x, y, ...
// 	pub int FrameCount { get { return curves.Length / BEZIER_SIZE + 1; } }

// 	pub CurveTimeline (int frameCount) {
// 		if (frameCount <= 0) throw new ArgumentException("frameCount must be > 0: " + frameCount, "frameCount");
// 		curves = new f64[(frameCount - 1) * BEZIER_SIZE];
// 	}

// 	abstract pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction);

// 	abstract pub int PropertyId { get; }

// 	pub void SetLinear (int frameIndex) {
// 		curves[frameIndex * BEZIER_SIZE] = LINEAR;
// 	}

// 	pub void SetStepped (int frameIndex) {
// 		curves[frameIndex * BEZIER_SIZE] = STEPPED;
// 	}

// 	/// <summary>Sets the control handle positions for an interpolation bezier curve used to transition from this keyframe to the next.
// 	/// cx1 and cx2 are from 0 to 1, representing the percent of time between the two keyframes. cy1 and cy2 are the percent of
// 	/// the difference between the keyframe's values.</summary>
// 	pub void SetCurve (int frameIndex, f64 cx1, f64 cy1, f64 cx2, f64 cy2) {
// 		f64 tmpx = (-cx1 * 2 + cx2) * 0.03f, tmpy = (-cy1 * 2 + cy2) * 0.03f;
// 		f64 dddfx = ((cx1 - cx2) * 3 + 1) * 0.006f, dddfy = ((cy1 - cy2) * 3 + 1) * 0.006f;
// 		f64 ddfx = tmpx * 2 + dddfx, ddfy = tmpy * 2 + dddfy;
// 		f64 dfx = cx1 * 0.3f + tmpx + dddfx * 0.16666667f, dfy = cy1 * 0.3f + tmpy + dddfy * 0.16666667f;

// 		int i = frameIndex * BEZIER_SIZE;
// 		f64[] curves = this.curves;
// 		curves[i++] = BEZIER;

// 		f64 x = dfx, y = dfy;
// 		for (int n = i + BEZIER_SIZE - 1; i < n; i += 2) {
// 			curves[i] = x;
// 			curves[i + 1] = y;
// 			dfx += ddfx;
// 			dfy += ddfy;
// 			ddfx += dddfx;
// 			ddfy += dddfy;
// 			x += dfx;
// 			y += dfy;
// 		}
// 	}

// 	pub f64 GetCurvePercent (int frameIndex, f64 percent) {
// 		percent = MathUtils.Clamp (percent, 0, 1);
// 		f64[] curves = this.curves;
// 		int i = frameIndex * BEZIER_SIZE;
// 		f64 type = curves[i];
// 		if (type == LINEAR) return percent;
// 		if (type == STEPPED) return 0;
// 		i++;
// 		f64 x = 0;
// 		for (int start = i, n = i + BEZIER_SIZE - 1; i < n; i += 2) {
// 			x = curves[i];
// 			if (x >= percent) {
// 				f64 prevX, prevY;
// 				if (i == start) {
// 					prevX = 0;
// 					prevY = 0;
// 				} else {
// 					prevX = curves[i - 2];
// 					prevY = curves[i - 1];
// 				}
// 				return prevY + (curves[i + 1] - prevY) * (percent - prevX) / (x - prevX);
// 			}
// 		}
// 		f64 y = curves[i - 1];
// 		return y + (1 - y) * (percent - x) / (1 - x); // Last point is 1,1.
// 	}
// 	pub f64 GetCurveType (int frameIndex) {
// 		return curves[frameIndex * BEZIER_SIZE];
// 	}
// }

// pub struct RotateTimeline : CurveTimeline {
// 	pub const int ENTRIES = 2;
// 		const int PREV_TIME = -2, PREV_ROTATION = -1;
// 		const int ROTATION = 1;

// 		int boneIndex;
// 		f64[] frames;

// 	pub int BoneIndex { get { return boneIndex; } set { boneIndex = value; } }
// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, angle, ...

// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.Rotate << 24) + boneIndex; }
// 	}

// 	pub RotateTimeline (int frameCount)
// 		: base(frameCount) {
// 		frames = new f64[frameCount << 1];
// 	}

// 	/// <summary>Sets the time and value of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, f64 time, f64 degrees) {
// 		frameIndex <<= 1;
// 		frames[frameIndex] = time;
// 		frames[frameIndex + ROTATION] = degrees;
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		Bone bone = skeleton.bones.Items[boneIndex];

// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			switch (pose) {
// 			case MixPose.Setup:
// 				bone.rotation = bone.data.rotation;
// 				return;
// 			case MixPose.Current:
// 				f64 rr = bone.data.rotation - bone.rotation;
// 				rr -= (16384 - (int)(16384.499999999996 - rr / 360)) * 360;
// 				bone.rotation += rr * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		if (time >= frames[frames.Length - ENTRIES]) { // Time is after last frame.
// 			if (pose == MixPose.Setup) {
// 				bone.rotation = bone.data.rotation + frames[frames.Length + PREV_ROTATION] * alpha;
// 			} else {
// 				f64 rr = bone.data.rotation + frames[frames.Length + PREV_ROTATION] - bone.rotation;
// 				rr -= (16384 - (int)(16384.499999999996 - rr / 360)) * 360; // Wrap within -180 and 180.
// 				bone.rotation += rr * alpha;
// 			}
// 			return;
// 		}

// 		// Interpolate between the previous frame and the current frame.
// 		int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 		f64 prevRotation = frames[frame + PREV_ROTATION];
// 		f64 frameTime = frames[frame];
// 		f64 percent = GetCurvePercent((frame >> 1) - 1, 1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 		f64 r = frames[frame + ROTATION] - prevRotation;
// 		r -= (16384 - (int)(16384.499999999996 - r / 360)) * 360;
// 		r = prevRotation + r * percent;
// 		if (pose == MixPose.Setup) {
// 			r -= (16384 - (int)(16384.499999999996 - r / 360)) * 360;
// 			bone.rotation = bone.data.rotation + r * alpha;
// 		} else {
// 			r = bone.data.rotation + r - bone.rotation;
// 			r -= (16384 - (int)(16384.499999999996 - r / 360)) * 360;
// 			bone.rotation += r * alpha;
// 		}
// 	}
// }

// pub struct TranslateTimeline : CurveTimeline {
// 	pub const int ENTRIES = 3;
// 	protected const int PREV_TIME = -3, PREV_X = -2, PREV_Y = -1;
// 	protected const int X = 1, Y = 2;

// 		int boneIndex;
// 		f64[] frames;

// 	pub int BoneIndex { get { return boneIndex; } set { boneIndex = value; } }
// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, value, value, ...

// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.Translate << 24) + boneIndex; }
// 	}

// 	pub TranslateTimeline (int frameCount)
// 		: base(frameCount) {
// 		frames = new f64[frameCount * ENTRIES];
// 	}

// 	/// <summary>Sets the time and value of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, f64 time, f64 x, f64 y) {
// 		frameIndex *= ENTRIES;
// 		frames[frameIndex] = time;
// 		frames[frameIndex + X] = x;
// 		frames[frameIndex + Y] = y;
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		Bone bone = skeleton.bones.Items[boneIndex];

// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			switch (pose) {
// 			case MixPose.Setup:
// 				bone.x = bone.data.x;
// 				bone.y = bone.data.y;
// 				return;
// 			case MixPose.Current:
// 				bone.x += (bone.data.x - bone.x) * alpha;
// 				bone.y += (bone.data.y - bone.y) * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		f64 x, y;
// 		if (time >= frames[frames.Length - ENTRIES]) { // Time is after last frame.
// 			x = frames[frames.Length + PREV_X];
// 			y = frames[frames.Length + PREV_Y];
// 		} else {
// 			// Interpolate between the previous frame and the current frame.
// 			int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 			x = frames[frame + PREV_X];
// 			y = frames[frame + PREV_Y];
// 			f64 frameTime = frames[frame];
// 			f64 percent = GetCurvePercent(frame / ENTRIES - 1,
// 				1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 			x += (frames[frame + X] - x) * percent;
// 			y += (frames[frame + Y] - y) * percent;
// 		}
// 		if (pose == MixPose.Setup) {
// 			bone.x = bone.data.x + x * alpha;
// 			bone.y = bone.data.y + y * alpha;
// 		} else {
// 			bone.x += (bone.data.x + x - bone.x) * alpha;
// 			bone.y += (bone.data.y + y - bone.y) * alpha;
// 		}
// 	}
// }

// pub struct ScaleTimeline : TranslateTimeline {
// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.Scale << 24) + boneIndex; }
// 	}

// 	pub ScaleTimeline (int frameCount)
// 		: base(frameCount) {
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		Bone bone = skeleton.bones.Items[boneIndex];

// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			switch (pose) {
// 			case MixPose.Setup:
// 				bone.scaleX = bone.data.scaleX;
// 				bone.scaleY = bone.data.scaleY;
// 				return;
// 			case MixPose.Current:
// 				bone.scaleX += (bone.data.scaleX - bone.scaleX) * alpha;
// 				bone.scaleY += (bone.data.scaleY - bone.scaleY) * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		f64 x, y;
// 		if (time >= frames[frames.Length - ENTRIES]) { // Time is after last frame.
// 			x = frames[frames.Length + PREV_X] * bone.data.scaleX;
// 			y = frames[frames.Length + PREV_Y] * bone.data.scaleY;
// 		} else {
// 			// Interpolate between the previous frame and the current frame.
// 			int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 			x = frames[frame + PREV_X];
// 			y = frames[frame + PREV_Y];
// 			f64 frameTime = frames[frame];
// 			f64 percent = GetCurvePercent(frame / ENTRIES - 1,
// 				1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 			x = (x + (frames[frame + X] - x) * percent) * bone.data.scaleX;
// 			y = (y + (frames[frame + Y] - y) * percent) * bone.data.scaleY;
// 		}
// 		if (alpha == 1) {
// 			bone.scaleX = x;
// 			bone.scaleY = y;
// 		} else {
// 			f64 bx, by;
// 			if (pose == MixPose.Setup) {
// 				bx = bone.data.scaleX;
// 				by = bone.data.scaleY;
// 			} else {
// 				bx = bone.scaleX;
// 				by = bone.scaleY;
// 			}
// 			// Mixing out uses sign of setup or current pose, else use sign of key.
// 			if (direction == MixDirection.Out) {
// 				x = (x >= 0 ? x : -x) * (bx >= 0 ? 1 : -1);
// 				y = (y >= 0 ? y : -y) * (by >= 0 ? 1 : -1);
// 			} else {
// 				bx = (bx >= 0 ? bx : -bx) * (x >= 0 ? 1 : -1);
// 				by = (by >= 0 ? by : -by) * (y >= 0 ? 1 : -1);
// 			}
// 			bone.scaleX = bx + (x - bx) * alpha;
// 			bone.scaleY = by + (y - by) * alpha;
// 		}
// 	}
// }

// pub struct ShearTimeline : TranslateTimeline {
// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.Shear << 24) + boneIndex; }
// 	}

// 	pub ShearTimeline (int frameCount)
// 		: base(frameCount) {
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		Bone bone = skeleton.bones.Items[boneIndex];
// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			switch (pose) {
// 			case MixPose.Setup:
// 				bone.shearX = bone.data.shearX;
// 				bone.shearY = bone.data.shearY;
// 				return;
// 			case MixPose.Current:
// 				bone.shearX += (bone.data.shearX - bone.shearX) * alpha;
// 				bone.shearY += (bone.data.shearY - bone.shearY) * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		f64 x, y;
// 		if (time >= frames[frames.Length - ENTRIES]) { // Time is after last frame.
// 			x = frames[frames.Length + PREV_X];
// 			y = frames[frames.Length + PREV_Y];
// 		} else {
// 			// Interpolate between the previous frame and the current frame.
// 			int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 			x = frames[frame + PREV_X];
// 			y = frames[frame + PREV_Y];
// 			f64 frameTime = frames[frame];
// 			f64 percent = GetCurvePercent(frame / ENTRIES - 1,
// 				1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 			x = x + (frames[frame + X] - x) * percent;
// 			y = y + (frames[frame + Y] - y) * percent;
// 		}
// 		if (pose == MixPose.Setup) {
// 			bone.shearX = bone.data.shearX + x * alpha;
// 			bone.shearY = bone.data.shearY + y * alpha;
// 		} else {
// 			bone.shearX += (bone.data.shearX + x - bone.shearX) * alpha;
// 			bone.shearY += (bone.data.shearY + y - bone.shearY) * alpha;
// 		}
// 	}
// }

// pub struct ColorTimeline : CurveTimeline {
// 	pub const int ENTRIES = 5;
// 	protected const int PREV_TIME = -5, PREV_R = -4, PREV_G = -3, PREV_B = -2, PREV_A = -1;
// 	protected const int R = 1, G = 2, B = 3, A = 4;

// 		int slotIndex;
// 		f64[] frames;

// 	pub int SlotIndex { get { return slotIndex; } set { slotIndex = value; } }
// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, r, g, b, a, ...

// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.Color << 24) + slotIndex; }
// 	}

// 	pub ColorTimeline (int frameCount)
// 		: base(frameCount) {
// 		frames = new f64[frameCount * ENTRIES];
// 	}

// 	/// <summary>Sets the time and value of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, f64 time, f64 r, f64 g, f64 b, f64 a) {
// 		frameIndex *= ENTRIES;
// 		frames[frameIndex] = time;
// 		frames[frameIndex + R] = r;
// 		frames[frameIndex + G] = g;
// 		frames[frameIndex + B] = b;
// 		frames[frameIndex + A] = a;
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		Slot slot = skeleton.slots.Items[slotIndex];
// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			var slotData = slot.data;
// 			switch (pose) {
// 			case MixPose.Setup:
// 				slot.r = slotData.r;
// 				slot.g = slotData.g;
// 				slot.b = slotData.b;
// 				slot.a = slotData.a;
// 				return;
// 			case MixPose.Current:
// 				slot.r += (slot.r - slotData.r) * alpha;
// 				slot.g += (slot.g - slotData.g) * alpha;
// 				slot.b += (slot.b - slotData.b) * alpha;
// 				slot.a += (slot.a - slotData.a) * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		f64 r, g, b, a;
// 		if (time >= frames[frames.Length - ENTRIES]) { // Time is after last frame.
// 			int i = frames.Length;
// 			r = frames[i + PREV_R];
// 			g = frames[i + PREV_G];
// 			b = frames[i + PREV_B];
// 			a = frames[i + PREV_A];
// 		} else {
// 			// Interpolate between the previous frame and the current frame.
// 			int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 			r = frames[frame + PREV_R];
// 			g = frames[frame + PREV_G];
// 			b = frames[frame + PREV_B];
// 			a = frames[frame + PREV_A];
// 			f64 frameTime = frames[frame];
// 			f64 percent = GetCurvePercent(frame / ENTRIES - 1,
// 				1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 			r += (frames[frame + R] - r) * percent;
// 			g += (frames[frame + G] - g) * percent;
// 			b += (frames[frame + B] - b) * percent;
// 			a += (frames[frame + A] - a) * percent;
// 		}
// 		if (alpha == 1) {
// 			slot.r = r;
// 			slot.g = g;
// 			slot.b = b;
// 			slot.a = a;
// 		} else {
// 			f64 br, bg, bb, ba;
// 			if (pose == MixPose.Setup) {
// 				br = slot.data.r;
// 				bg = slot.data.g;
// 				bb = slot.data.b;
// 				ba = slot.data.a;
// 			} else {
// 				br = slot.r;
// 				bg = slot.g;
// 				bb = slot.b;
// 				ba = slot.a;
// 			}
// 			slot.r = br + ((r - br) * alpha);
// 			slot.g = bg + ((g - bg) * alpha);
// 			slot.b = bb + ((b - bb) * alpha);
// 			slot.a = ba + ((a - ba) * alpha);
// 		}
// 	}
// }

// pub struct TwoColorTimeline : CurveTimeline {
// 	pub const int ENTRIES = 8;
// 	protected const int PREV_TIME = -8, PREV_R = -7, PREV_G = -6, PREV_B = -5, PREV_A = -4;
// 	protected const int PREV_R2 = -3, PREV_G2 = -2, PREV_B2 = -1;
// 	protected const int R = 1, G = 2, B = 3, A = 4, R2 = 5, G2 = 6, B2 = 7;

// 		int slotIndex;
// 		f64[] frames; // time, r, g, b, a, r2, g2, b2, ...

// 	pub int SlotIndex {
// 		get { return slotIndex; }
// 		set {
// 			if (value < 0)
// 				throw new ArgumentOutOfRangeException("index must be >= 0.");
// 			slotIndex = value;
// 		}
// 	}
	
// 	pub f64[] Frames { get { return frames; } }

// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.TwoColor << 24) + slotIndex; }
// 	}

// 	pub TwoColorTimeline (int frameCount) :
// 		base(frameCount) {
// 		frames = new f64[frameCount * ENTRIES];
// 	}

// 	/// <summary>Sets the time and value of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, f64 time, f64 r, f64 g, f64 b, f64 a, f64 r2, f64 g2, f64 b2) {
// 		frameIndex *= ENTRIES;
// 		frames[frameIndex] = time;
// 		frames[frameIndex + R] = r;
// 		frames[frameIndex + G] = g;
// 		frames[frameIndex + B] = b;
// 		frames[frameIndex + A] = a;
// 		frames[frameIndex + R2] = r2;
// 		frames[frameIndex + G2] = g2;
// 		frames[frameIndex + B2] = b2;
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		Slot slot = skeleton.slots.Items[slotIndex];
// 		f64[] frames = this.frames;
// 		if (time < frames[0]) { // Time is before first frame.
// 			var slotData = slot.data;
// 			switch (pose) {
// 			case MixPose.Setup:
// 				//	slot.color.set(slot.data.color);
// 				//	slot.darkColor.set(slot.data.darkColor);
// 				slot.r = slotData.r;
// 				slot.g = slotData.g;
// 				slot.b = slotData.b;
// 				slot.a = slotData.a;
// 				slot.r2 = slotData.r2;
// 				slot.g2 = slotData.g2;
// 				slot.b2 = slotData.b2;
// 				return;
// 			case MixPose.Current:
// 				slot.r += (slot.r - slotData.r) * alpha;
// 				slot.g += (slot.g - slotData.g) * alpha;
// 				slot.b += (slot.b - slotData.b) * alpha;
// 				slot.a += (slot.a - slotData.a) * alpha;
// 				slot.r2 += (slot.r2 - slotData.r2) * alpha;
// 				slot.g2 += (slot.g2 - slotData.g2) * alpha;
// 				slot.b2 += (slot.b2 - slotData.b2) * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		f64 r, g, b, a, r2, g2, b2;
// 		if (time >= frames[frames.Length - ENTRIES]) { // Time is after last frame.
// 			int i = frames.Length;
// 			r = frames[i + PREV_R];
// 			g = frames[i + PREV_G];
// 			b = frames[i + PREV_B];
// 			a = frames[i + PREV_A];
// 			r2 = frames[i + PREV_R2];
// 			g2 = frames[i + PREV_G2];
// 			b2 = frames[i + PREV_B2];
// 		} else {
// 			// Interpolate between the previous frame and the current frame.
// 			int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 			r = frames[frame + PREV_R];
// 			g = frames[frame + PREV_G];
// 			b = frames[frame + PREV_B];
// 			a = frames[frame + PREV_A];
// 			r2 = frames[frame + PREV_R2];
// 			g2 = frames[frame + PREV_G2];
// 			b2 = frames[frame + PREV_B2];
// 			f64 frameTime = frames[frame];
// 			f64 percent = GetCurvePercent(frame / ENTRIES - 1,
// 				1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 			r += (frames[frame + R] - r) * percent;
// 			g += (frames[frame + G] - g) * percent;
// 			b += (frames[frame + B] - b) * percent;
// 			a += (frames[frame + A] - a) * percent;
// 			r2 += (frames[frame + R2] - r2) * percent;
// 			g2 += (frames[frame + G2] - g2) * percent;
// 			b2 += (frames[frame + B2] - b2) * percent;
// 		}
// 		if (alpha == 1) {
// 			slot.r = r;
// 			slot.g = g;
// 			slot.b = b;
// 			slot.a = a;
// 			slot.r2 = r2;
// 			slot.g2 = g2;
// 			slot.b2 = b2;
// 		} else {
// 			f64 br, bg, bb, ba, br2, bg2, bb2;
// 			if (pose == MixPose.Setup) {
// 				br = slot.data.r;
// 				bg = slot.data.g;
// 				bb = slot.data.b;
// 				ba = slot.data.a;
// 				br2 = slot.data.r2;
// 				bg2 = slot.data.g2;
// 				bb2 = slot.data.b2;
// 			} else {
// 				br = slot.r;
// 				bg = slot.g;
// 				bb = slot.b;
// 				ba = slot.a;
// 				br2 = slot.r2;
// 				bg2 = slot.g2;
// 				bb2 = slot.b2;
// 			}
// 			slot.r = br + ((r - br) * alpha);
// 			slot.g = bg + ((g - bg) * alpha);
// 			slot.b = bb + ((b - bb) * alpha);
// 			slot.a = ba + ((a - ba) * alpha);
// 			slot.r2 = br2 + ((r2 - br2) * alpha);
// 			slot.g2 = bg2 + ((g2 - bg2) * alpha);
// 			slot.b2 = bb2 + ((b2 - bb2) * alpha);
// 		}
// 	}

// }

// pub struct AttachmentTimeline : Timeline {
// 		int slotIndex;
// 		f64[] frames;
// 		string[] attachmentNames;

// 	pub int SlotIndex { get { return slotIndex; } set { slotIndex = value; } }
// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, ...
// 	pub string[] AttachmentNames { get { return attachmentNames; } set { attachmentNames = value; } }
// 	pub int FrameCount { get { return frames.Length; } }

// 	pub int PropertyId {
// 		get { return ((int)TimelineType.Attachment << 24) + slotIndex; }
// 	}

// 	pub AttachmentTimeline (int frameCount) {
// 		frames = new f64[frameCount];
// 		attachmentNames = new String[frameCount];
// 	}

// 	/// <summary>Sets the time and value of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, f64 time, String attachmentName) {
// 		frames[frameIndex] = time;
// 		attachmentNames[frameIndex] = attachmentName;
// 	}

// 	pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		string attachmentName;
// 		Slot slot = skeleton.slots.Items[slotIndex];
// 		if (direction == MixDirection.Out && pose == MixPose.Setup) {
// 			attachmentName = slot.data.attachmentName;
// 			slot.Attachment = attachmentName == null ? null : skeleton.GetAttachment(slotIndex, attachmentName);
// 			return;
// 		}

// 		f64[] frames = this.frames;
// 		if (time < frames[0]) { // Time is before first frame.
// 			if (pose == MixPose.Setup) {
// 				attachmentName = slot.data.attachmentName;
// 				slot.Attachment = attachmentName == null ? null : skeleton.GetAttachment(slotIndex, attachmentName);
// 			}
// 			return;
// 		}

// 		int frameIndex;
// 		if (time >= frames[frames.Length - 1]) // Time is after last frame.
// 			frameIndex = frames.Length - 1;
// 		else
// 			frameIndex = Animation.BinarySearch(frames, time, 1) - 1;

// 		attachmentName = attachmentNames[frameIndex];
// 		slot.Attachment = attachmentName == null ? null : skeleton.GetAttachment(slotIndex, attachmentName);
// 	}
// }

// pub struct DeformTimeline : CurveTimeline {
// 		int slotIndex;
// 		f64[] frames;
// 		f64[][] frameVertices;
// 		VertexAttachment attachment;

// 	pub int SlotIndex { get { return slotIndex; } set { slotIndex = value; } }
// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, ...
// 	pub f64[][] Vertices { get { return frameVertices; } set { frameVertices = value; } }
// 	pub VertexAttachment Attachment { get { return attachment; } set { attachment = value; } }

// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.Deform << 24) + attachment.id + slotIndex; }
// 	}

// 	pub DeformTimeline (int frameCount)
// 		: base(frameCount) {
// 		frames = new f64[frameCount];
// 		frameVertices = new f64[frameCount][];
// 	}

// 	/// <summary>Sets the time and value of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, f64 time, f64[] vertices) {
// 		frames[frameIndex] = time;
// 		frameVertices[frameIndex] = vertices;
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		Slot slot = skeleton.slots.Items[slotIndex];
// 		VertexAttachment vertexAttachment = slot.attachment as VertexAttachment;
// 		if (vertexAttachment == null || !vertexAttachment.ApplyDeform(attachment)) return;

// 		var verticesArray = slot.attachmentVertices;
// 		if (verticesArray.Count == 0) alpha = 1;

// 		f64[][] frameVertices = this.frameVertices;
// 		int vertexCount = frameVertices[0].Length;
// 		f64[] frames = this.frames;
// 		f64[] vertices;

// 		if (time < frames[0]) {
			
// 			switch (pose) {
// 			case MixPose.Setup:
// 				verticesArray.Clear();
// 				return;
// 			case MixPose.Current:
// 				if (alpha == 1) {
// 					verticesArray.Clear();
// 					return;
// 				}

// 				// verticesArray.SetSize(vertexCount) // Ensure size and preemptively set count.
// 				if (verticesArray.Capacity < vertexCount) verticesArray.Capacity = vertexCount;	
// 				verticesArray.Count = vertexCount;
// 				vertices = verticesArray.Items;

// 				if (vertexAttachment.bones == null) {
// 					// Unweighted vertex positions.
// 					f64[] setupVertices = vertexAttachment.vertices;
// 					for (int i = 0; i < vertexCount; i++)
// 						vertices[i] += (setupVertices[i] - vertices[i]) * alpha;
// 				} else {
// 					// Weighted deform offsets.
// 					alpha = 1 - alpha;
// 					for (int i = 0; i < vertexCount; i++)
// 						vertices[i] *= alpha;
// 				}
// 				return;
// 			default:
// 				return;
// 			}

// 		}

// 		// verticesArray.SetSize(vertexCount) // Ensure size and preemptively set count.
// 		if (verticesArray.Capacity < vertexCount) verticesArray.Capacity = vertexCount;	
// 		verticesArray.Count = vertexCount;
// 		vertices = verticesArray.Items;

// 		if (time >= frames[frames.Length - 1]) { // Time is after last frame.
// 			f64[] lastVertices = frameVertices[frames.Length - 1];
// 			if (alpha == 1) {
// 				// Vertex positions or deform offsets, no alpha.
// 				Array.Copy(lastVertices, 0, vertices, 0, vertexCount);
// 			} else if (pose == MixPose.Setup) {
// 				if (vertexAttachment.bones == null) {
// 					// Unweighted vertex positions, with alpha.
// 					f64[] setupVertices = vertexAttachment.vertices;
// 					for (int i = 0; i < vertexCount; i++) {
// 						f64 setup = setupVertices[i];
// 						vertices[i] = setup + (lastVertices[i] - setup) * alpha;
// 					}
// 				} else {
// 					// Weighted deform offsets, with alpha.
// 					for (int i = 0; i < vertexCount; i++)
// 						vertices[i] = lastVertices[i] * alpha;
// 				}
// 			} else {
// 				// Vertex positions or deform offsets, with alpha.
// 				for (int i = 0; i < vertexCount; i++)
// 					vertices[i] += (lastVertices[i] - vertices[i]) * alpha;
// 			}
// 			return;
// 		}

// 		// Interpolate between the previous frame and the current frame.
// 		int frame = Animation.BinarySearch(frames, time);
// 		f64[] prevVertices = frameVertices[frame - 1];
// 		f64[] nextVertices = frameVertices[frame];
// 		f64 frameTime = frames[frame];
// 		f64 percent = GetCurvePercent(frame - 1, 1 - (time - frameTime) / (frames[frame - 1] - frameTime));

// 		if (alpha == 1) {
// 			// Vertex positions or deform offsets, no alpha.
// 			for (int i = 0; i < vertexCount; i++) {
// 				f64 prev = prevVertices[i];
// 				vertices[i] = prev + (nextVertices[i] - prev) * percent;
// 			}
// 		} else if (pose == MixPose.Setup) {
// 			if (vertexAttachment.bones == null) {
// 				// Unweighted vertex positions, with alpha.
// 				var setupVertices = vertexAttachment.vertices;
// 				for (int i = 0; i < vertexCount; i++) {
// 					f64 prev = prevVertices[i], setup = setupVertices[i];
// 					vertices[i] = setup + (prev + (nextVertices[i] - prev) * percent - setup) * alpha;
// 				}
// 			} else {
// 				// Weighted deform offsets, with alpha.
// 				for (int i = 0; i < vertexCount; i++) {
// 					f64 prev = prevVertices[i];
// 					vertices[i] = (prev + (nextVertices[i] - prev) * percent) * alpha;
// 				}
// 			}
// 		} else {
// 			// Vertex positions or deform offsets, with alpha.
// 			for (int i = 0; i < vertexCount; i++) {
// 				f64 prev = prevVertices[i];
// 				vertices[i] += (prev + (nextVertices[i] - prev) * percent - vertices[i]) * alpha;
// 			}
// 		}
// 	}
// }

// pub struct EventTimeline : Timeline {
// 		f64[] frames;
// 	private Event[] events;

// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, ...
// 	pub Event[] Events { get { return events; } set { events = value; } }
// 	pub int FrameCount { get { return frames.Length; } }

// 	pub int PropertyId {
// 		get { return ((int)TimelineType.Event << 24); }
// 	}

// 	pub EventTimeline (int frameCount) {
// 		frames = new f64[frameCount];
// 		events = new Event[frameCount];
// 	}

// 	/// <summary>Sets the time and value of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, Event e) {
// 		frames[frameIndex] = e.Time;
// 		events[frameIndex] = e;
// 	}

// 	/// <summary>Fires events for frames &gt; lastTime and &lt;= time.</summary>
// 	pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		if (firedEvents == null) return;
// 		f64[] frames = this.frames;
// 		int frameCount = frames.Length;

// 		if (lastTime > time) { // Fire events after last time for looped animations.
// 			Apply(skeleton, lastTime, int.MaxValue, firedEvents, alpha, pose, direction);
// 			lastTime = -1f;
// 		} else if (lastTime >= frames[frameCount - 1]) // Last time is after last frame.
// 			return;
// 		if (time < frames[0]) return; // Time is before first frame.

// 		int frame;
// 		if (lastTime < frames[0])
// 			frame = 0;
// 		else {
// 			frame = Animation.BinarySearch(frames, lastTime);
// 			f64 frameTime = frames[frame];
// 			while (frame > 0) { // Fire multiple events with the same frame.
// 				if (frames[frame - 1] != frameTime) break;
// 				frame--;
// 			}
// 		}
// 		for (; frame < frameCount && time >= frames[frame]; frame++)
// 			firedEvents.Add(events[frame]);
// 	}
// }

// pub struct DrawOrderTimeline : Timeline {
// 		f64[] frames;
// 	private int[][] drawOrders;

// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, ...
// 	pub int[][] DrawOrders { get { return drawOrders; } set { drawOrders = value; } }
// 	pub int FrameCount { get { return frames.Length; } }

// 	pub int PropertyId {
// 		get { return ((int)TimelineType.DrawOrder << 24); }
// 	}

// 	pub DrawOrderTimeline (int frameCount) {
// 		frames = new f64[frameCount];
// 		drawOrders = new int[frameCount][];
// 	}

// 	/// <summary>Sets the time and value of the specified keyframe.</summary>
// 	/// <param name="drawOrder">May be null to use bind pose draw order.</param>
// 	pub void SetFrame (int frameIndex, f64 time, int[] drawOrder) {
// 		frames[frameIndex] = time;
// 		drawOrders[frameIndex] = drawOrder;
// 	}

// 	pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		ExposedList<Slot> drawOrder = skeleton.drawOrder;
// 		ExposedList<Slot> slots = skeleton.slots;
// 		if (direction == MixDirection.Out && pose == MixPose.Setup) {
// 			Array.Copy(slots.Items, 0, drawOrder.Items, 0, slots.Count);
// 			return;
// 		}

// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			if (pose == MixPose.Setup) Array.Copy(slots.Items, 0, drawOrder.Items, 0, slots.Count);
// 			return;
// 		}

// 		int frame;
// 		if (time >= frames[frames.Length - 1]) // Time is after last frame.
// 			frame = frames.Length - 1;
// 		else
// 			frame = Animation.BinarySearch(frames, time) - 1;
		
// 		int[] drawOrderToSetupIndex = drawOrders[frame];
// 		if (drawOrderToSetupIndex == null) {
// 			drawOrder.Clear();
// 			for (int i = 0, n = slots.Count; i < n; i++)
// 				drawOrder.Add(slots.Items[i]);
// 		} else {
// 			var drawOrderItems = drawOrder.Items;
// 			var slotsItems = slots.Items;
// 			for (int i = 0, n = drawOrderToSetupIndex.Length; i < n; i++)
// 				drawOrderItems[i] = slotsItems[drawOrderToSetupIndex[i]];
// 		}
// 	}
// }

// pub struct IkConstraintTimeline : CurveTimeline {
// 	pub const int ENTRIES = 3;
// 	private const int PREV_TIME = -3, PREV_MIX = -2, PREV_BEND_DIRECTION = -1;
// 	private const int MIX = 1, BEND_DIRECTION = 2;

// 		int ikConstraintIndex;
// 		f64[] frames;

// 	pub int IkConstraintIndex { get { return ikConstraintIndex; } set { ikConstraintIndex = value; } }
// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, mix, bendDirection, ...

// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.IkConstraint << 24) + ikConstraintIndex; }
// 	}

// 	pub IkConstraintTimeline (int frameCount)
// 		: base(frameCount) {
// 		frames = new f64[frameCount * ENTRIES];
// 	}
		
// 	/// <summary>Sets the time, mix and bend direction of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, f64 time, f64 mix, int bendDirection) {
// 		frameIndex *= ENTRIES;
// 		frames[frameIndex] = time;
// 		frames[frameIndex + MIX] = mix;
// 		frames[frameIndex + BEND_DIRECTION] = bendDirection;
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		IkConstraint constraint = skeleton.ikConstraints.Items[ikConstraintIndex];
// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			switch (pose) {
// 			case MixPose.Setup:
// 				constraint.mix = constraint.data.mix;
// 				constraint.bendDirection = constraint.data.bendDirection;
// 				return;
// 			case MixPose.Current:
// 				constraint.mix += (constraint.data.mix - constraint.mix) * alpha;
// 				constraint.bendDirection = constraint.data.bendDirection;
// 				return;
// 			}
// 			return;
// 		}

// 		if (time >= frames[frames.Length - ENTRIES]) { // Time is after last frame.
// 			if (pose == MixPose.Setup) {
// 				constraint.mix = constraint.data.mix + (frames[frames.Length + PREV_MIX] - constraint.data.mix) * alpha;
// 				constraint.bendDirection = direction == MixDirection.Out ? constraint.data.bendDirection
// 					: (int)frames[frames.Length + PREV_BEND_DIRECTION];
// 			} else {
// 				constraint.mix += (frames[frames.Length + PREV_MIX] - constraint.mix) * alpha;
// 				if (direction == MixDirection.In) constraint.bendDirection = (int)frames[frames.Length + PREV_BEND_DIRECTION];
// 			}
// 			return;
// 		}

// 		// Interpolate between the previous frame and the current frame.
// 		int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 		f64 mix = frames[frame + PREV_MIX];
// 		f64 frameTime = frames[frame];
// 		f64 percent = GetCurvePercent(frame / ENTRIES - 1, 1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 		if (pose == MixPose.Setup) {
// 			constraint.mix = constraint.data.mix + (mix + (frames[frame + MIX] - mix) * percent - constraint.data.mix) * alpha;
// 			constraint.bendDirection = direction == MixDirection.Out ? constraint.data.bendDirection : (int)frames[frame + PREV_BEND_DIRECTION];
// 		} else {
// 			constraint.mix += (mix + (frames[frame + MIX] - mix) * percent - constraint.mix) * alpha;
// 			if (direction == MixDirection.In) constraint.bendDirection = (int)frames[frame + PREV_BEND_DIRECTION];
// 		}
// 	}
// }

// pub struct TransformConstraintTimeline : CurveTimeline {
// 	pub const int ENTRIES = 5;
// 	private const int PREV_TIME = -5, PREV_ROTATE = -4, PREV_TRANSLATE = -3, PREV_SCALE = -2, PREV_SHEAR = -1;
// 	private const int ROTATE = 1, TRANSLATE = 2, SCALE = 3, SHEAR = 4;

// 		int transformConstraintIndex;
// 		f64[] frames;

// 	pub int TransformConstraintIndex { get { return transformConstraintIndex; } set { transformConstraintIndex = value; } }
// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, rotate mix, translate mix, scale mix, shear mix, ...

// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.TransformConstraint << 24) + transformConstraintIndex; }
// 	}

// 	pub TransformConstraintTimeline (int frameCount)
// 		: base(frameCount) {
// 		frames = new f64[frameCount * ENTRIES];
// 	}

// 	pub void SetFrame (int frameIndex, f64 time, f64 rotateMix, f64 translateMix, f64 scaleMix, f64 shearMix) {
// 		frameIndex *= ENTRIES;
// 		frames[frameIndex] = time;
// 		frames[frameIndex + ROTATE] = rotateMix;
// 		frames[frameIndex + TRANSLATE] = translateMix;
// 		frames[frameIndex + SCALE] = scaleMix;
// 		frames[frameIndex + SHEAR] = shearMix;
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		TransformConstraint constraint = skeleton.transformConstraints.Items[transformConstraintIndex];
// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			var data = constraint.data;
// 			switch (pose) {
// 			case MixPose.Setup:
// 				constraint.rotateMix = data.rotateMix;
// 				constraint.translateMix = data.translateMix;
// 				constraint.scaleMix = data.scaleMix;
// 				constraint.shearMix = data.shearMix;
// 				return;
// 			case MixPose.Current:
// 				constraint.rotateMix += (data.rotateMix - constraint.rotateMix) * alpha;
// 				constraint.translateMix += (data.translateMix - constraint.translateMix) * alpha;
// 				constraint.scaleMix += (data.scaleMix - constraint.scaleMix) * alpha;
// 				constraint.shearMix += (data.shearMix - constraint.shearMix) * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		f64 rotate, translate, scale, shear;
// 		if (time >= frames[frames.Length - ENTRIES]) { // Time is after last frame.
// 			int i = frames.Length;
// 			rotate = frames[i + PREV_ROTATE];
// 			translate = frames[i + PREV_TRANSLATE];
// 			scale = frames[i + PREV_SCALE];
// 			shear = frames[i + PREV_SHEAR];
// 		} else {
// 			// Interpolate between the previous frame and the current frame.
// 			int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 			rotate = frames[frame + PREV_ROTATE];
// 			translate = frames[frame + PREV_TRANSLATE];
// 			scale = frames[frame + PREV_SCALE];
// 			shear = frames[frame + PREV_SHEAR];
// 			f64 frameTime = frames[frame];
// 			f64 percent = GetCurvePercent(frame / ENTRIES - 1,
// 				1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 			rotate += (frames[frame + ROTATE] - rotate) * percent;
// 			translate += (frames[frame + TRANSLATE] - translate) * percent;
// 			scale += (frames[frame + SCALE] - scale) * percent;
// 			shear += (frames[frame + SHEAR] - shear) * percent;
// 		}
// 		if (pose == MixPose.Setup) {
// 			TransformConstraintData data = constraint.data;
// 			constraint.rotateMix = data.rotateMix + (rotate - data.rotateMix) * alpha;
// 			constraint.translateMix = data.translateMix + (translate - data.translateMix) * alpha;
// 			constraint.scaleMix = data.scaleMix + (scale - data.scaleMix) * alpha;
// 			constraint.shearMix = data.shearMix + (shear - data.shearMix) * alpha;
// 		} else {
// 			constraint.rotateMix += (rotate - constraint.rotateMix) * alpha;
// 			constraint.translateMix += (translate - constraint.translateMix) * alpha;
// 			constraint.scaleMix += (scale - constraint.scaleMix) * alpha;
// 			constraint.shearMix += (shear - constraint.shearMix) * alpha;
// 		}
// 	}
// }

// pub struct PathConstraintPositionTimeline : CurveTimeline {
// 	pub const int ENTRIES = 2;
// 	protected const int PREV_TIME = -2, PREV_VALUE = -1;
// 	protected const int VALUE = 1;

// 		int pathConstraintIndex;
// 		f64[] frames;

// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.PathConstraintPosition << 24) + pathConstraintIndex; }
// 	}

// 	pub PathConstraintPositionTimeline (int frameCount)
// 		: base(frameCount) {
// 		frames = new f64[frameCount * ENTRIES];
// 	}

// 	pub int PathConstraintIndex { get { return pathConstraintIndex; } set { pathConstraintIndex = value; } }
// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, position, ...

// 	/// <summary>Sets the time and value of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, f64 time, f64 value) {
// 		frameIndex *= ENTRIES;
// 		frames[frameIndex] = time;
// 		frames[frameIndex + VALUE] = value;
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		PathConstraint constraint = skeleton.pathConstraints.Items[pathConstraintIndex];
// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			switch (pose) {
// 			case MixPose.Setup:
// 				constraint.position = constraint.data.position;
// 				return;
// 			case MixPose.Current:
// 				constraint.position += (constraint.data.position - constraint.position) * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		f64 position;
// 		if (time >= frames[frames.Length - ENTRIES]) // Time is after last frame.
// 			position = frames[frames.Length + PREV_VALUE];
// 		else {
// 			// Interpolate between the previous frame and the current frame.
// 			int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 			position = frames[frame + PREV_VALUE];
// 			f64 frameTime = frames[frame];
// 			f64 percent = GetCurvePercent(frame / ENTRIES - 1,
// 				1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 			position += (frames[frame + VALUE] - position) * percent;
// 		}
// 		if (pose == MixPose.Setup)
// 			constraint.position = constraint.data.position + (position - constraint.data.position) * alpha;
// 		else
// 			constraint.position += (position - constraint.position) * alpha;
// 	}
// }

// pub struct PathConstraintSpacingTimeline : PathConstraintPositionTimeline {
// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.PathConstraintSpacing << 24) + pathConstraintIndex; }
// 	}

// 	pub PathConstraintSpacingTimeline (int frameCount)
// 		: base(frameCount) {
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		PathConstraint constraint = skeleton.pathConstraints.Items[pathConstraintIndex];
// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			switch (pose) {
// 			case MixPose.Setup:
// 				constraint.spacing = constraint.data.spacing;
// 				return;
// 			case MixPose.Current:
// 				constraint.spacing += (constraint.data.spacing - constraint.spacing) * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		f64 spacing;
// 		if (time >= frames[frames.Length - ENTRIES]) // Time is after last frame.
// 			spacing = frames[frames.Length + PREV_VALUE];
// 		else {
// 			// Interpolate between the previous frame and the current frame.
// 			int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 			spacing = frames[frame + PREV_VALUE];
// 			f64 frameTime = frames[frame];
// 			f64 percent = GetCurvePercent(frame / ENTRIES - 1,
// 				1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 			spacing += (frames[frame + VALUE] - spacing) * percent;
// 		}

// 		if (pose == MixPose.Setup)
// 			constraint.spacing = constraint.data.spacing + (spacing - constraint.data.spacing) * alpha;
// 		else
// 			constraint.spacing += (spacing - constraint.spacing) * alpha;
// 	}
// }

// pub struct PathConstraintMixTimeline : CurveTimeline {
// 	pub const int ENTRIES = 3;
// 	private const int PREV_TIME = -3, PREV_ROTATE = -2, PREV_TRANSLATE = -1;
// 	private const int ROTATE = 1, TRANSLATE = 2;

// 		int pathConstraintIndex;
// 		f64[] frames;

// 	pub int PathConstraintIndex { get { return pathConstraintIndex; } set { pathConstraintIndex = value; } }
// 	pub f64[] Frames { get { return frames; } set { frames = value; } } // time, rotate mix, translate mix, ...

// 	override pub int PropertyId {
// 		get { return ((int)TimelineType.PathConstraintMix << 24) + pathConstraintIndex; }
// 	}

// 	pub PathConstraintMixTimeline (int frameCount)
// 		: base(frameCount) {
// 		frames = new f64[frameCount * ENTRIES];
// 	}			

// 	/// <summary>Sets the time and mixes of the specified keyframe.</summary>
// 	pub void SetFrame (int frameIndex, f64 time, f64 rotateMix, f64 translateMix) {
// 		frameIndex *= ENTRIES;
// 		frames[frameIndex] = time;
// 		frames[frameIndex + ROTATE] = rotateMix;
// 		frames[frameIndex + TRANSLATE] = translateMix;
// 	}

// 	override pub void Apply (Skeleton skeleton, f64 lastTime, f64 time, ExposedList<Event> firedEvents, f64 alpha, MixPose pose, MixDirection direction) {
// 		PathConstraint constraint = skeleton.pathConstraints.Items[pathConstraintIndex];
// 		f64[] frames = this.frames;
// 		if (time < frames[0]) {
// 			switch (pose) {
// 			case MixPose.Setup:
// 				constraint.rotateMix = constraint.data.rotateMix;
// 				constraint.translateMix = constraint.data.translateMix;
// 				return;
// 			case MixPose.Current:
// 				constraint.rotateMix += (constraint.data.rotateMix - constraint.rotateMix) * alpha;
// 				constraint.translateMix += (constraint.data.translateMix - constraint.translateMix) * alpha;
// 				return;
// 			}
// 			return;
// 		}

// 		f64 rotate, translate;
// 		if (time >= frames[frames.Length - ENTRIES]) { // Time is after last frame.
// 			rotate = frames[frames.Length + PREV_ROTATE];
// 			translate = frames[frames.Length + PREV_TRANSLATE];
// 		} else {
// 			// Interpolate between the previous frame and the current frame.
// 			int frame = Animation.BinarySearch(frames, time, ENTRIES);
// 			rotate = frames[frame + PREV_ROTATE];
// 			translate = frames[frame + PREV_TRANSLATE];
// 			f64 frameTime = frames[frame];
// 			f64 percent = GetCurvePercent(frame / ENTRIES - 1,
// 				1 - (time - frameTime) / (frames[frame + PREV_TIME] - frameTime));

// 			rotate += (frames[frame + ROTATE] - rotate) * percent;
// 			translate += (frames[frame + TRANSLATE] - translate) * percent;
// 		}

// 		if (pose == MixPose.Setup) {
// 			constraint.rotateMix = constraint.data.rotateMix + (rotate - constraint.data.rotateMix) * alpha;
// 			constraint.translateMix = constraint.data.translateMix + (translate - constraint.data.translateMix) * alpha;
// 		} else {
// 			constraint.rotateMix += (rotate - constraint.rotateMix) * alpha;
// 			constraint.translateMix += (translate - constraint.translateMix) * alpha;
// 		}
// 	}
}
