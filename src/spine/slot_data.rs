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

pub struct SlotData {
	index: i32,
	name: String,
	bone_data: BoneData,
	r: f64,
	g: f64,
	b: f64,
	a: f64,
	r2: f64,
	g2: f64,
	b2: f64,
	has_second_colour: bool,
	attachment_name: String,
	blend_mode: BlendMode,
	
	// pub SlotData (int index, String name, BoneData boneData) {
	// 	if (index < 0) throw new ArgumentException ("index must be >= 0.", "index");
	// 	if (name == null) throw new ArgumentNullException("name", "name cannot be null.");
	// 	if (boneData == null) throw new ArgumentNullException("boneData", "boneData cannot be null.");
	// 	this.index = index;
	// 	this.name = name;
	// 	this.boneData = boneData;
	// }

	// override pub string ToString () {
	// 	return name;
	// }
}

impl SlotData
{
	pub fn new(
		index: i32,
		name: String,
		bone_data: BoneData,
		has_second_color: bool,
		attachment_name: String,
		blend_mode: BlendMode) -> SlotData
	{
		SlotData {
			index,
			name,
			bone_data,
			r: 1,
			g: 1,
			b: 1,
			a: 1,
			r2: 0,
			g2: 0,
			b2: 0,
			has_second_color,
			attachment_name,
			blend_mode
		}
	}
}