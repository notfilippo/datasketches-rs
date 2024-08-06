// Copyright 2024 Filippo Rossi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The hll module contains a set of very compact implementations of Phillipe
//! Flajolet’s HyperLogLog (HLL) sketch but with significantly improved error
//! behavior and excellent speed performance.
//! If the use case for sketching is primarily counting uniques and merging, the
//! HLL sketch is the 2nd highest performing in terms of accuracy for storage
//! space consumed (the new CPC sketch developed by Kevin J. Lang now beats HLL).
//! For large counts, HLL sketches can be 2 to 16 times smaller for the same
//! accuracy than the Theta Sketches mentioned above, and the CPC sketch is
//! another 30 to 40% smaller still.

use datasketches_sys::hll::ffi::target_hll_type;

pub mod sketch;
pub mod union;

/// Specifies the target type of HLL sketch to be created. It is a target in that the actual
/// allocation of the HLL array is deferred until sufficient number of items have been received by
/// the warm-up phases.
///
/// These three target types are isomorphic representations of the same underlying HLL algorithm.
/// Thus, given the same value of `lg_config_k` and the same input, all three HLL target types
/// will produce identical estimates and have identical error distributions.
///
/// The memory (and also the serialization) of the sketch during this early warmup phase starts
/// out very small (8 bytes, when empty) and then grows in increments of 4 bytes as required
/// until the full HLL array is allocated.  This transition point occurs at about 10% of K for
/// sketches where `lg_config_k` is > 8.
pub enum HllType {
    /// This uses a 4-bit field per HLL bucket and for large counts may require
    /// the use of a small internal auxiliary array for storing statistical exceptions, which are rare.
    /// For the values of `lg_config_k > 13` (`K` = 8192),
    /// this additional array adds about 3% to the overall storage. It is generally the slowest in
    /// terms of update time, but has the smallest storage footprint of about
    /// `K/2 * 1.03` bytes.
    HLL4,
    /// This uses a 6-bit field per HLL bucket. It is the generally the next fastest
    /// in terms of update time with a storage footprint of about `3/4 * K` bytes.
    HLL6,
    /// This uses an 8-bit byte per HLL bucket. It is generally the
    /// fastest in terms of update time, but has the largest storage footprint of about
    /// `K` bytes.
    HLL8,
}

impl From<HllType> for target_hll_type {
    fn from(value: HllType) -> Self {
        match value {
            HllType::HLL4 => target_hll_type::HLL_4,
            HllType::HLL6 => target_hll_type::HLL_6,
            HllType::HLL8 => target_hll_type::HLL_8,
        }
    }
}
