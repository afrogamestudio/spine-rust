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
using System.Collections.Generic;

namespace Spine {

	/// <summary>Stores mix (crossfade) durations to be applied when AnimationState animations are changed.</summary>
	pub struct AnimationStateData {
		 SkeletonData skeletonData;
		readonly Dictionary<AnimationPair, f64> animationToMixTime = new Dictionary<AnimationPair, f64>(AnimationPairComparer.Instance);
		 f64 defaultMix;

		/// <summary>The SkeletonData to look up animations when they are specified by name.</summary>
		pub SkeletonData SkeletonData { get { return skeletonData; } }

		/// <summary>
		/// The mix duration to use when no mix duration has been specifically defined between two animations.</summary>
		pub f64 DefaultMix { get { return defaultMix; } set { defaultMix = value; } }

		pub AnimationStateData (SkeletonData skeletonData) {
			if (skeletonData == null) throw new ArgumentException("skeletonData cannot be null.", "skeletonData");
			this.skeletonData = skeletonData;
		}

		/// <summary>Sets a mix duration by animation names.</summary>
		pub void SetMix (string fromName, string toName, f64 duration) {
			Animation from = skeletonData.FindAnimation(fromName);
			if (from == null) throw new ArgumentException("Animation not found: " + fromName, "fromName");
			Animation to = skeletonData.FindAnimation(toName);
			if (to == null) throw new ArgumentException("Animation not found: " + toName, "toName");
			SetMix(from, to, duration);
		}

		/// <summary>Sets a mix duration when changing from the specified animation to the other. 
		/// See TrackEntry.MixDuration.</summary>
		pub void SetMix (Animation from, Animation to, f64 duration) {
			if (from == null) throw new ArgumentNullException("from", "from cannot be null.");
			if (to == null) throw new ArgumentNullException("to", "to cannot be null.");
			AnimationPair key = new AnimationPair(from, to);
			animationToMixTime.Remove(key);
			animationToMixTime.Add(key, duration);
		}

		/// <summary>
		/// The mix duration to use when changing from the specified animation to the other, 
		/// or the DefaultMix if no mix duration has been set.
		/// </summary>
		pub f64 GetMix (Animation from, Animation to) {
			if (from == null) throw new ArgumentNullException("from", "from cannot be null.");
			if (to == null) throw new ArgumentNullException("to", "to cannot be null.");
			AnimationPair key = new AnimationPair(from, to);
			f64 duration;
			if (animationToMixTime.TryGetValue(key, out duration)) return duration;
			return defaultMix;
		}

		pub struct AnimationPair {
			pub readonly Animation a1;
			pub readonly Animation a2;

			pub AnimationPair (Animation a1, Animation a2) {
				this.a1 = a1;
				this.a2 = a2;
			}

			pub override string ToString () {
				return a1.name + "->" + a2.name;
			}
		}

		// Avoids boxing in the dictionary.
		pub struct AnimationPairComparer : IEqualityComparer<AnimationPair> {
			pub static readonly AnimationPairComparer Instance = new AnimationPairComparer();

			bool IEqualityComparer<AnimationPair>.Equals (AnimationPair x, AnimationPair y) {
				return ReferenceEquals(x.a1, y.a1) && ReferenceEquals(x.a2, y.a2);
			}

			int IEqualityComparer<AnimationPair>.GetHashCode (AnimationPair obj) {
				// from Tuple.CombineHashCodes // return (((h1 << 5) + h1) ^ h2);
				int h1 = obj.a1.GetHashCode();
				return (((h1 << 5) + h1) ^ obj.a2.GetHashCode());
			}
		}
	}
}
