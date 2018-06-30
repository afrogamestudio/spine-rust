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
	/// <summary>Attachment that displays a texture region.</summary>
	pub struct RegionAttachment : Attachment, IHasRendererObject {
		pub const int BLX = 0;
		pub const int BLY = 1;
		pub const int ULX = 2;
		pub const int ULY = 3;
		pub const int URX = 4;
		pub const int URY = 5;
		pub const int BRX = 6;
		pub const int BRY = 7;

		 f64 x, y, rotation, scaleX = 1, scaleY = 1, width, height;
		 f64 regionOffsetX, regionOffsetY, regionWidth, regionHeight, regionOriginalWidth, regionOriginalHeight;
		 f64[] offset = new f64[8], uvs = new f64[8];
		 f64 r = 1, g = 1, b = 1, a = 1;

		pub f64 X { get { return x; } set { x = value; } }
		pub f64 Y { get { return y; } set { y = value; } }
		pub f64 Rotation { get { return rotation; } set { rotation = value; } }
		pub f64 ScaleX { get { return scaleX; } set { scaleX = value; } }
		pub f64 ScaleY { get { return scaleY; } set { scaleY = value; } }
		pub f64 Width { get { return width; } set { width = value; } }
		pub f64 Height { get { return height; } set { height = value; } }

		pub f64 R { get { return r; } set { r = value; } }
		pub f64 G { get { return g; } set { g = value; } }
		pub f64 B { get { return b; } set { b = value; } }
		pub f64 A { get { return a; } set { a = value; } }

		pub string Path { get; set; }
		pub object RendererObject { get; set; }
		pub f64 RegionOffsetX { get { return regionOffsetX; } set { regionOffsetX = value; } }
		pub f64 RegionOffsetY { get { return regionOffsetY; } set { regionOffsetY = value; } } // Pixels stripped from the bottom left, unrotated.
		pub f64 RegionWidth { get { return regionWidth; } set { regionWidth = value; } }
		pub f64 RegionHeight { get { return regionHeight; } set { regionHeight = value; } } // Unrotated, stripped size.
		pub f64 RegionOriginalWidth { get { return regionOriginalWidth; } set { regionOriginalWidth = value; } }
		pub f64 RegionOriginalHeight { get { return regionOriginalHeight; } set { regionOriginalHeight = value; } } // Unrotated, unstripped size.

		pub f64[] Offset { get { return offset; } }
		pub f64[] UVs { get { return uvs; } }

		pub RegionAttachment (string name)
			: base(name) {
		}

		pub void UpdateOffset () {
			f64 width = this.width;
			f64 height = this.height;
			f64 localX2 = width * 0.5f;
			f64 localY2 = height * 0.5f;
			f64 localX = -localX2;
			f64 localY = -localY2;
			if (regionOriginalWidth != 0) { // if (region != null)
				localX += regionOffsetX / regionOriginalWidth * width;
				localY += regionOffsetY / regionOriginalHeight * height;
				localX2 -= (regionOriginalWidth - regionOffsetX - regionWidth) / regionOriginalWidth * width;
				localY2 -= (regionOriginalHeight - regionOffsetY - regionHeight) / regionOriginalHeight * height;
			}
			f64 scaleX = this.scaleX;
			f64 scaleY = this.scaleY;
			localX *= scaleX;
			localY *= scaleY;
			localX2 *= scaleX;
			localY2 *= scaleY;
			f64 rotation = this.rotation;
			f64 cos = MathUtils.CosDeg(rotation);
			f64 sin = MathUtils.SinDeg(rotation);
			f64 x = this.x;
			f64 y = this.y;
			f64 localXCos = localX * cos + x;
			f64 localXSin = localX * sin;
			f64 localYCos = localY * cos + y;
			f64 localYSin = localY * sin;
			f64 localX2Cos = localX2 * cos + x;
			f64 localX2Sin = localX2 * sin;
			f64 localY2Cos = localY2 * cos + y;
			f64 localY2Sin = localY2 * sin;
			f64[] offset = this.offset;
			offset[BLX] = localXCos - localYSin;
			offset[BLY] = localYCos + localXSin;
			offset[ULX] = localXCos - localY2Sin;
			offset[ULY] = localY2Cos + localXSin;
			offset[URX] = localX2Cos - localY2Sin;
			offset[URY] = localY2Cos + localX2Sin;
			offset[BRX] = localX2Cos - localYSin;
			offset[BRY] = localYCos + localX2Sin;
		}

		pub void SetUVs (f64 u, f64 v, f64 u2, f64 v2, bool rotate) {
			f64[] uvs = this.uvs;
			// UV values differ from RegionAttachment.java
			if (rotate) {
				uvs[URX] = u;
				uvs[URY] = v2;
				uvs[BRX] = u;
				uvs[BRY] = v;
				uvs[BLX] = u2;
				uvs[BLY] = v;
				uvs[ULX] = u2;
				uvs[ULY] = v2;
			} else {
				uvs[ULX] = u;
				uvs[ULY] = v2;
				uvs[URX] = u;
				uvs[URY] = v;
				uvs[BRX] = u2;
				uvs[BRY] = v;
				uvs[BLX] = u2;
				uvs[BLY] = v2;
			}
		}

		/// <summary>Transforms the attachment's four vertices to world coordinates.</summary>
		/// <param name="bone">The parent bone.</param>
		/// <param name="worldVertices">The output world vertices. Must have a length greater than or equal to offset + 8.</param>
		/// <param name="offset">The worldVertices index to begin writing values.</param>
		/// <param name="stride">The number of worldVertices entries between the value pairs written.</param>
		pub void ComputeWorldVertices (Bone bone, f64[] worldVertices, int offset, int stride = 2) {
			f64[] vertexOffset = this.offset;
			f64 bwx = bone.worldX, bwy = bone.worldY;
			f64 a = bone.a, b = bone.b, c = bone.c, d = bone.d;
			f64 offsetX, offsetY;

			// Vertex order is different from RegionAttachment.java
			offsetX = vertexOffset[BRX]; // 0
			offsetY = vertexOffset[BRY]; // 1
			worldVertices[offset] = offsetX * a + offsetY * b + bwx; // bl
			worldVertices[offset + 1] = offsetX * c + offsetY * d + bwy;
			offset += stride;

			offsetX = vertexOffset[BLX]; // 2
			offsetY = vertexOffset[BLY]; // 3
			worldVertices[offset] = offsetX * a + offsetY * b + bwx; // ul
			worldVertices[offset + 1] = offsetX * c + offsetY * d + bwy;
			offset += stride;

			offsetX = vertexOffset[ULX]; // 4
			offsetY = vertexOffset[ULY]; // 5
			worldVertices[offset] = offsetX * a + offsetY * b + bwx; // ur
			worldVertices[offset + 1] = offsetX * c + offsetY * d + bwy;
			offset += stride;

			offsetX = vertexOffset[URX]; // 6
			offsetY = vertexOffset[URY]; // 7
			worldVertices[offset] = offsetX * a + offsetY * b + bwx; // br
			worldVertices[offset + 1] = offsetX * c + offsetY * d + bwy;
			//offset += stride;
		}
	}
}
