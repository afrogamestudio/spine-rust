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

pub struct Slot
{
	data: SlotData;
	Bone bone;
	f64 r, g, b, a;
	f64 r2, g2, b2;
	bool hasSecondColor;
	Attachment attachment;
	f64 attachmentTime;
	ExposedList<f64> attachmentVertices = new ExposedList<f64>();

	pub SlotData Data { get { return data; } }
	pub Bone Bone { get { return bone; } }
	pub Skeleton Skeleton { get { return bone.skeleton; } }
	pub f64 R { get { return r; } set { r = value; } }
	pub f64 G { get { return g; } set { g = value; } }
	pub f64 B { get { return b; } set { b = value; } }
	pub f64 A { get { return a; } set { a = value; } }

	pub f64 R2 { get { return r2; } set { r2 = value; } }
	pub f64 G2 { get { return g2; } set { g2 = value; } }
	pub f64 B2 { get { return b2; } set { b2 = value; } }
	pub bool HasSecondColor { get { return data.hasSecondColor; } set { data.hasSecondColor = value; } }

	/// <summary>May be null.</summary>
	pub Attachment Attachment {
		get { return attachment; }
		set {
			if (attachment == value) return;
			attachment = value;
			attachmentTime = bone.skeleton.time;
			attachmentVertices.Clear(false);
		}
	}

	pub f64 AttachmentTime {
		get { return bone.skeleton.time - attachmentTime; }
		set { attachmentTime = bone.skeleton.time - value; }
	}

	pub ExposedList<f64> AttachmentVertices { get { return attachmentVertices; } set { attachmentVertices = value; } }

	pub Slot (SlotData data, Bone bone) {
		if (data == null) throw new ArgumentNullException("data", "data cannot be null.");
		if (bone == null) throw new ArgumentNullException("bone", "bone cannot be null.");
		this.data = data;
		this.bone = bone;
		SetToSetupPose();
	}

	pub void SetToSetupPose () {
		r = data.r;
		g = data.g;
		b = data.b;
		a = data.a;
		if (data.attachmentName == null)
			Attachment = null;
		else {
			attachment = null;
			Attachment = bone.skeleton.GetAttachment(data.index, data.attachmentName);
		}
	}

	override pub string ToString () {
		return data.name;
	}
}
