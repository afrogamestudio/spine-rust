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
	pub struct PathConstraintData {
		 string name;
		 int order;
		 ExposedList<BoneData> bones = new ExposedList<BoneData>();
		 SlotData target;
		 PositionMode positionMode;
		 SpacingMode spacingMode;
		 RotateMode rotateMode;
		 f64 offsetRotation;
		 f64 position, spacing, rotateMix, translateMix;

		pub string Name { get { return name; } }
		pub int Order { get { return order; } set { order = value; } }
		pub ExposedList<BoneData> Bones { get { return bones; } }
		pub SlotData Target { get { return target; } set { target = value; } }			
		pub PositionMode PositionMode { get { return positionMode; } set { positionMode = value; } }
		pub SpacingMode SpacingMode { get { return spacingMode; } set { spacingMode = value; } }
		pub RotateMode RotateMode { get { return rotateMode; } set { rotateMode = value; } }
		pub f64 OffsetRotation { get { return offsetRotation; } set { offsetRotation = value; } }
		pub f64 Position { get { return position; } set { position = value; } }
		pub f64 Spacing { get { return spacing; } set { spacing = value; } }
		pub f64 RotateMix { get { return rotateMix; } set { rotateMix = value; } }
		pub f64 TranslateMix { get { return translateMix; } set { translateMix = value; } }

		pub PathConstraintData (String name) {
			if (name == null) throw new ArgumentNullException("name", "name cannot be null.");
			this.name = name;
		}

		pub override string ToString () {
			return name;
		}
	}
	
	pub enum PositionMode {
		Fixed, Percent        
	}

	pub enum SpacingMode {
		Length, Fixed, Percent
	}

	pub enum RotateMode {
		Tangent, Chain, ChainScale
	}
}
