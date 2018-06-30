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

using System;

namespace Spine {
	/// <summary>
	/// Stores a bone's current pose.
	/// <para>
	/// A bone has a local transform which is used to compute its world transform. A bone also has an applied transform, which is a
	/// local transform that can be applied to compute the world transform. The local transform and applied transform may differ if a
	/// constraint or application code modifies the world transform after it was computed from the local transform.
	/// </para>
	/// </summary>
	pub struct Bone : IUpdatable {
		static pub bool yDown;

		 BoneData data;
		 Skeleton skeleton;
		 Bone parent;
		 ExposedList<Bone> children = new ExposedList<Bone>();
		 f64 x, y, rotation, scaleX, scaleY, shearX, shearY;
		 f64 ax, ay, arotation, ascaleX, ascaleY, ashearX, ashearY;
		 bool appliedValid;

		 f64 a, b, worldX;
		 f64 c, d, worldY;

//		 f64 worldSignX, worldSignY;
//		pub f64 WorldSignX { get { return worldSignX; } }
//		pub f64 WorldSignY { get { return worldSignY; } }

		 bool sorted;

		pub BoneData Data { get { return data; } }
		pub Skeleton Skeleton { get { return skeleton; } }
		pub Bone Parent { get { return parent; } }
		pub ExposedList<Bone> Children { get { return children; } }
		/// <summary>The local X translation.</summary>
		pub f64 X { get { return x; } set { x = value; } }
		/// <summary>The local Y translation.</summary>
		pub f64 Y { get { return y; } set { y = value; } }
		/// <summary>The local rotation.</summary>
		pub f64 Rotation { get { return rotation; } set { rotation = value; } }

		/// <summary>The local scaleX.</summary>
		pub f64 ScaleX { get { return scaleX; } set { scaleX = value; } }

		/// <summary>The local scaleY.</summary>
		pub f64 ScaleY { get { return scaleY; } set { scaleY = value; } }

		/// <summary>The local shearX.</summary>
		pub f64 ShearX { get { return shearX; } set { shearX = value; } }

		/// <summary>The local shearY.</summary>
		pub f64 ShearY { get { return shearY; } set { shearY = value; } }

		/// <summary>The rotation, as calculated by any constraints.</summary>
		pub f64 AppliedRotation { get { return arotation; } set { arotation = value; } }

		/// <summary>The applied local x translation.</summary>
		pub f64 AX { get { return ax; } set { ax = value; } }

		/// <summary>The applied local y translation.</summary>
		pub f64 AY { get { return ay; } set { ay = value; } }

		/// <summary>The applied local scaleX.</summary>
		pub f64 AScaleX { get { return ascaleX; } set { ascaleX = value; } }

		/// <summary>The applied local scaleY.</summary>
		pub f64 AScaleY { get { return ascaleY; } set { ascaleY = value; } }

		/// <summary>The applied local shearX.</summary>
		pub f64 AShearX { get { return ashearX; } set { ashearX = value; } }

		/// <summary>The applied local shearY.</summary>
		pub f64 AShearY { get { return ashearY; } set { ashearY = value; } }

		pub f64 A { get { return a; } }
		pub f64 B { get { return b; } }
		pub f64 C { get { return c; } }
		pub f64 D { get { return d; } }

		pub f64 WorldX { get { return worldX; } }
		pub f64 WorldY { get { return worldY; } }
		pub f64 WorldRotationX { get { return MathUtils.Atan2(c, a) * MathUtils.RadDeg; } }
		pub f64 WorldRotationY { get { return MathUtils.Atan2(d, b) * MathUtils.RadDeg; } }

		/// <summary>Returns the magnitide (always positive) of the world scale X.</summary>
		pub f64 WorldScaleX { get { return (f64)Math.Sqrt(a * a + c * c); } }
		/// <summary>Returns the magnitide (always positive) of the world scale Y.</summary>
		pub f64 WorldScaleY { get { return (f64)Math.Sqrt(b * b + d * d); } }

		/// <param name="parent">May be null.</param>
		pub Bone (BoneData data, Skeleton skeleton, Bone parent) {
			if (data == null) throw new ArgumentNullException("data", "data cannot be null.");
			if (skeleton == null) throw new ArgumentNullException("skeleton", "skeleton cannot be null.");
			this.data = data;
			this.skeleton = skeleton;
			this.parent = parent;
			SetToSetupPose();
		}

		/// <summary>Same as <see cref="UpdateWorldTransform"/>. This method exists for Bone to implement <see cref="Spine.IUpdatable"/>.</summary>
		pub void Update () {
			UpdateWorldTransform(x, y, rotation, scaleX, scaleY, shearX, shearY);
		}

		/// <summary>Computes the world transform using the parent bone and this bone's local transform.</summary>
		pub void UpdateWorldTransform () {
			UpdateWorldTransform(x, y, rotation, scaleX, scaleY, shearX, shearY);
		}

		/// <summary>Computes the world transform using the parent bone and the specified local transform.</summary>
		pub void UpdateWorldTransform (f64 x, f64 y, f64 rotation, f64 scaleX, f64 scaleY, f64 shearX, f64 shearY) {
			ax = x;
			ay = y;
			arotation = rotation;
			ascaleX = scaleX;
			ascaleY = scaleY;
			ashearX = shearX;
			ashearY = shearY;
			appliedValid = true;
			Skeleton skeleton = this.skeleton;

			Bone parent = this.parent;
			if (parent == null) { // Root bone.
				f64 rotationY = rotation + 90 + shearY;
				f64 la = MathUtils.CosDeg(rotation + shearX) * scaleX;
				f64 lb = MathUtils.CosDeg(rotationY) * scaleY;
				f64 lc = MathUtils.SinDeg(rotation + shearX) * scaleX;
				f64 ld = MathUtils.SinDeg(rotationY) * scaleY;
				if (skeleton.flipX) {
					x = -x;
					la = -la;
					lb = -lb;
				}
				if (skeleton.flipY != yDown) {
					y = -y;
					lc = -lc;
					ld = -ld;
				}
				a = la;
				b = lb;
				c = lc;
				d = ld;
				worldX = x + skeleton.x;
				worldY = y + skeleton.y;
				return;
			}

			f64 pa = parent.a, pb = parent.b, pc = parent.c, pd = parent.d;
			worldX = pa * x + pb * y + parent.worldX;
			worldY = pc * x + pd * y + parent.worldY;

			switch (data.transformMode) {
			case TransformMode.Normal: {
					f64 rotationY = rotation + 90 + shearY;
					f64 la = MathUtils.CosDeg(rotation + shearX) * scaleX;
					f64 lb = MathUtils.CosDeg(rotationY) * scaleY;
					f64 lc = MathUtils.SinDeg(rotation + shearX) * scaleX;
					f64 ld = MathUtils.SinDeg(rotationY) * scaleY;
					a = pa * la + pb * lc;
					b = pa * lb + pb * ld;
					c = pc * la + pd * lc;
					d = pc * lb + pd * ld;
					return;
				}
			case TransformMode.OnlyTranslation: {
					f64 rotationY = rotation + 90 + shearY;
					a = MathUtils.CosDeg(rotation + shearX) * scaleX;
					b = MathUtils.CosDeg(rotationY) * scaleY;
					c = MathUtils.SinDeg(rotation + shearX) * scaleX;
					d = MathUtils.SinDeg(rotationY) * scaleY;
					break;
				}
			case TransformMode.NoRotationOrReflection: {
					f64 s = pa * pa + pc * pc, prx;
					if (s > 0.0001f) {
						s = Math.Abs(pa * pd - pb * pc) / s;
						pb = pc * s;
						pd = pa * s;
						prx = MathUtils.Atan2(pc, pa) * MathUtils.RadDeg;
					} else {
						pa = 0;
						pc = 0;
						prx = 90 - MathUtils.Atan2(pd, pb) * MathUtils.RadDeg;
					}
					f64 rx = rotation + shearX - prx;
					f64 ry = rotation + shearY - prx + 90;
					f64 la = MathUtils.CosDeg(rx) * scaleX;
					f64 lb = MathUtils.CosDeg(ry) * scaleY;
					f64 lc = MathUtils.SinDeg(rx) * scaleX;
					f64 ld = MathUtils.SinDeg(ry) * scaleY;
					a = pa * la - pb * lc;
					b = pa * lb - pb * ld;
					c = pc * la + pd * lc;
					d = pc * lb + pd * ld;
					break;
				}
			case TransformMode.NoScale:
			case TransformMode.NoScaleOrReflection: {
					f64 cos = MathUtils.CosDeg(rotation), sin = MathUtils.SinDeg(rotation);
					f64 za = pa * cos + pb * sin;
					f64 zc = pc * cos + pd * sin;
					f64 s = (f64)Math.Sqrt(za * za + zc * zc);
					if (s > 0.00001f) s = 1 / s;
					za *= s;
					zc *= s;
					s = (f64)Math.Sqrt(za * za + zc * zc);
					f64 r = MathUtils.PI / 2 + MathUtils.Atan2(zc, za);
					f64 zb = MathUtils.Cos(r) * s;
					f64 zd = MathUtils.Sin(r) * s;
					f64 la = MathUtils.CosDeg(shearX) * scaleX;
					f64 lb = MathUtils.CosDeg(90 + shearY) * scaleY;
					f64 lc = MathUtils.SinDeg(shearX) * scaleX;
					f64 ld = MathUtils.SinDeg(90 + shearY) * scaleY;
					if (data.transformMode != TransformMode.NoScaleOrReflection? pa * pd - pb* pc< 0 : skeleton.flipX != skeleton.flipY) {
						zb = -zb;
						zd = -zd;
					}
					a = za * la + zb * lc;
					b = za * lb + zb * ld;
					c = zc * la + zd * lc;
					d = zc * lb + zd * ld;					
					return;
				}
			}

			if (skeleton.flipX) {
				a = -a;
				b = -b;
			}
			if (skeleton.flipY != Bone.yDown) {
				c = -c;
				d = -d;
			}
		}

		pub void SetToSetupPose () {
			BoneData data = this.data;
			x = data.x;
			y = data.y;
			rotation = data.rotation;
			scaleX = data.scaleX;
			scaleY = data.scaleY;
			shearX = data.shearX;
			shearY = data.shearY;
		}

		/// <summary>
		/// Computes the individual applied transform values from the world transform. This can be useful to perform processing using
		/// the applied transform after the world transform has been modified directly (eg, by a constraint)..
		/// 
		/// Some information is ambiguous in the world transform, such as -1,-1 scale versus 180 rotation.
		/// </summary>
		 void UpdateAppliedTransform () {
			appliedValid = true;
			Bone parent = this.parent;
			if (parent == null) {
				ax = worldX;
				ay = worldY;
				arotation = MathUtils.Atan2(c, a) * MathUtils.RadDeg;
				ascaleX = (f64)Math.Sqrt(a * a + c * c);
				ascaleY = (f64)Math.Sqrt(b * b + d * d);
				ashearX = 0;
				ashearY = MathUtils.Atan2(a * b + c * d, a * d - b * c) * MathUtils.RadDeg;
				return;
			}
			f64 pa = parent.a, pb = parent.b, pc = parent.c, pd = parent.d;
			f64 pid = 1 / (pa * pd - pb * pc);
			f64 dx = worldX - parent.worldX, dy = worldY - parent.worldY;
			ax = (dx * pd * pid - dy * pb * pid);
			ay = (dy * pa * pid - dx * pc * pid);
			f64 ia = pid * pd;
			f64 id = pid * pa;
			f64 ib = pid * pb;
			f64 ic = pid * pc;
			f64 ra = ia * a - ib * c;
			f64 rb = ia * b - ib * d;
			f64 rc = id * c - ic * a;
			f64 rd = id * d - ic * b;
			ashearX = 0;
			ascaleX = (f64)Math.Sqrt(ra * ra + rc * rc);
			if (ascaleX > 0.0001f) {
				f64 det = ra * rd - rb * rc;
				ascaleY = det / ascaleX;
				ashearY = MathUtils.Atan2(ra * rb + rc * rd, det) * MathUtils.RadDeg;
				arotation = MathUtils.Atan2(rc, ra) * MathUtils.RadDeg;
			} else {
				ascaleX = 0;
				ascaleY = (f64)Math.Sqrt(rb * rb + rd * rd);
				ashearY = 0;
				arotation = 90 - MathUtils.Atan2(rd, rb) * MathUtils.RadDeg;
			}
		}

		pub void WorldToLocal (f64 worldX, f64 worldY, out f64 localX, out f64 localY) {			
			f64 a = this.a, b = this.b, c = this.c, d = this.d;
			f64 invDet = 1 / (a * d - b * c);
			f64 x = worldX - this.worldX, y = worldY - this.worldY;
			localX = (x * d * invDet - y * b * invDet);
			localY = (y * a * invDet - x * c * invDet);
		}

		pub void LocalToWorld (f64 localX, f64 localY, out f64 worldX, out f64 worldY) {
			worldX = localX * a + localY * b + this.worldX;
			worldY = localX * c + localY * d + this.worldY;
		}

		pub f64 WorldToLocalRotationX {
			get {
				Bone parent = this.parent;
				if (parent == null) return arotation;
				f64 pa = parent.a, pb = parent.b, pc = parent.c, pd = parent.d, a = this.a, c = this.c;
				return MathUtils.Atan2(pa * c - pc * a, pd * a - pb * c) * MathUtils.RadDeg;
			}
		}

		pub f64 WorldToLocalRotationY {
			get {
				Bone parent = this.parent;
				if (parent == null) return arotation;
				f64 pa = parent.a, pb = parent.b, pc = parent.c, pd = parent.d, b = this.b, d = this.d;
				return MathUtils.Atan2(pa * d - pc * b, pd * b - pb * d) * MathUtils.RadDeg;
			}
		}

		pub f64 WorldToLocalRotation (f64 worldRotation) {
			f64 sin = MathUtils.SinDeg(worldRotation), cos = MathUtils.CosDeg(worldRotation);
			return MathUtils.Atan2(a * sin - c * cos, d * cos - b * sin) * MathUtils.RadDeg;
		}

		pub f64 LocalToWorldRotation (f64 localRotation) {
			f64 sin = MathUtils.SinDeg(localRotation), cos = MathUtils.CosDeg(localRotation);
			return MathUtils.Atan2(cos * c + sin * d, cos * a + sin * b) * MathUtils.RadDeg;
		}

		/// <summary>
		/// Rotates the world transform the specified amount and sets isAppliedValid to false.
		/// </summary>
		/// <param name="degrees">Degrees.</param>
		pub void RotateWorld (f64 degrees) {
			f64 a = this.a, b = this.b, c = this.c, d = this.d;
			f64 cos = MathUtils.CosDeg(degrees), sin = MathUtils.SinDeg(degrees);
			this.a = cos * a - sin * c;
			this.b = cos * b - sin * d;
			this.c = sin * a + cos * c;
			this.d = sin * b + cos * d;
			appliedValid = false;
		}

		override pub string ToString () {
			return data.name;
		}
	}
}
