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

//! Refer to [HllSketch].

use crate::macros::*;
use std::ops::Deref;

use cxx::{let_cxx_string, UniquePtr};
use datasketches_sys::hll::ffi::*;

use super::HllType;

/// This is a high performance implementation of Phillipe Flajolet's HLL sketch but with
/// significantly improved error behavior.  If the ONLY use case for sketching is counting
/// uniques and merging, the HLL sketch is a reasonable choice, although the highest performing in terms of accuracy for
/// storage space consumed is CPC (Compressed Probabilistic Counting). For large enough counts, this HLL version (with HLL_4) can be 2 to
/// 16 times smaller than the Theta sketch family for the same accuracy.
///
/// This implementation offers three different types of HLL sketch, each with different
/// trade-offs with accuracy, space and performance. These types are specified with the
/// [HllType] parameter.
///
/// In terms of accuracy, all three types, for the same `lg_config_k`, have the same error
/// distribution as a function of `n`, the number of unique values fed to the sketch.
/// The configuration parameter `lg_config_k` is the log-base-2 of `K`,
/// where `K` is the number of buckets or slots for the sketch.
///
/// During warmup, when the sketch has only received a small number of unique items
/// (up to about 10% of `K`), this implementation leverages a new class of estimator
/// algorithms with significantly better accuracy.
///
/// This sketch also offers the capability of operating off-heap. Given a WritableMemory object
/// created by the user, the sketch will perform all of its updates and internal phase transitions
/// in that object, which can actually reside either on-heap or off-heap based on how it is
/// configured. In large systems that must update and merge many millions of sketches, having the
/// sketch operate off-heap avoids the serialization and deserialization costs of moving sketches
/// to and from off-heap memory-mapped files, for example, and eliminates big garbage collection
/// delays.
///
/// author Jon Malkin
/// author Lee Rhodes
/// author Kevin Lang
pub struct HllSketch(pub(crate) UniquePtr<hll_sketch>);

impl Default for HllSketch {
    fn default() -> Self {
        Self::new(12, HllType::HLL4, false)
    }
}

impl Clone for HllSketch {
    fn clone(&self) -> Self {
        Self(hll_sketch_copy(&self.0))
    }
}

impl std::fmt::Display for HllSketch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string(true, false, false, false))
    }
}

impl HllSketch {
    /// Constructs a new HLL sketch.
    /// - `lg_config_k` Sketch can hold 2^lg_config_k rows
    /// - `tgt_type` The HLL mode to use, if/when the sketch reaches that state
    /// - `start_full_size` Indicates whether to start in HLL mode,
    /// keeping memory use constant (if HLL_6 or HLL_8) at the cost of
    /// starting out using much more memory
    pub fn new(lg_config_k: u8, tgt_type: HllType, start_full_size: bool) -> Self {
        Self(hll_sketch_new(
            lg_config_k,
            tgt_type.into(),
            start_full_size,
        ))
    }

    /// Reconstructs a sketch from a serialized image in a byte array.
    #[inline]
    pub fn deserialize(bytes: &[u8]) -> Self {
        Self(hll_sketch_deserialize(bytes))
    }

    /// Serializes the sketch to a byte array, compacting data structures
    /// where feasible to eliminate unused storage in the serialized image.
    /// - `header_size_bytes` Allows for PostgreSQL integration, otherwise
    /// set it to 0.
    #[inline]
    pub fn serialize_compact(&self, header_size_bytes: usize) -> Vec<u8> {
        hll_sketch_serialize_compact(&self.0, header_size_bytes)
    }

    /// Serializes the sketch to a byte array, retaining all internal
    /// data structures in their current form.
    #[inline]
    pub fn serialize_updatable(&self) -> Vec<u8> {
        hll_sketch_serialize_updatable(&self.0)
    }

    /// Copy constructor to a new target type.
    #[inline]
    pub fn clone_with_type(&self, tgt_type: HllType) -> Self {
        Self(hll_sketch_copy_with_target(&self.0, tgt_type.into()))
    }

    /// Human readable summary with optional detail
    /// - `summary` if true, output the sketch summary
    /// - `detail` if true, output the internal data array
    /// - `aux_detail` if true, output the internal Aux array, if it exists.
    /// - `all` if true, outputs all entries including empty ones
    #[inline]
    pub fn to_string(&self, summary: bool, detail: bool, aux_detail: bool, all: bool) -> String {
        hll_sketch_to_string(&self.0, summary, detail, aux_detail, all)
    }

    /// Present the given string as a potential unique item.
    /// The string is converted to a byte array using UTF8 encoding.
    /// If the string is null or empty no update attempt is made and the method returns.
    #[inline]
    pub fn update_string(&mut self, datum: &str) {
        let_cxx_string!(raw_datum = datum);
        self.0.pin_mut().update_string(raw_datum.deref())
    }

    wrap_mut!(
        /// Resets the sketch to an empty state in coupon collection mode.
        /// Does not re-use existing internal objects.
        pub fn reset() -> ());

    wrap_mut!(
        /// Present the given unsigned 64-bit integer as a potential unique item.
        pub fn update_u64(datum: u64) -> ());
    wrap_mut!(
        /// Present the given unsigned 32-bit integer as a potential unique item.
        pub fn update_u32(datum: u32) -> ());
    wrap_mut!(
        /// Present the given unsigned 16-bit integer as a potential unique item.
        pub fn update_u16(datum: u16) -> ());
    wrap_mut!(
        /// Present the given unsigned 8-bit integer as a potential unique item.
        pub fn update_u8(datum: u8) -> ());

    wrap_mut!(
        /// Present the given signed 64-bit integer as a potential unique item.
        pub fn update_i64(datum: i64) -> ());
    wrap_mut!(
        /// Present the given signed 32-bit integer as a potential unique item.
        pub fn update_i32(datum: i32) -> ());
    wrap_mut!(
        /// Present the given signed 16-bit integer as a potential unique item.
        pub fn update_i16(datum: i16) -> ());
    wrap_mut!(
        /// Present the given signed 8-bit integer as a potential unique item.
        pub fn update_i8(datum: i8) -> ());

    wrap_mut!(
        /// Present the given 64-bit floating point value as a potential unique item.
        pub fn update_f64(datum: f64) -> ());
    wrap_mut!(
        /// Present the given 32-bit floating point value as a potential unique item.
        pub fn update_f32(datum: f32) -> ());

    wrap!(
        /// Returns the current cardinality estimate.
        pub fn get_estimate() -> f64);
    wrap!(
        /// This is less accurate than the get_estimate() method
        /// and is automatically used when the union has gone through
        /// union operations where the more accurate HIP estimator cannot
        /// be used.
        pub fn get_composite_estimate() -> f64);
    wrap!(
        /// Returns the approximate lower error bound given the specified
        /// number of standard deviations.
        pub fn get_lower_bound(num_std_dev: u8) -> f64);
    wrap!(
        /// Returns the approximate upper error bound given the specified
        /// number of standard deviations.
        pub fn get_upper_bound(num_std_dev: u8) -> f64);
    wrap!(
        /// Indicates if the sketch is currently stored compacted.
        pub fn is_compact() -> bool);
    wrap!(
        /// Indicates if the sketch is currently empty.
        pub fn is_empty() -> bool);

    wrap!(
        /// Returns the size of the sketch serialized in compact form.
        pub fn get_compact_serialization_bytes() -> u32);
    wrap!(
        /// Returns the size of the sketch serialized without compaction.
        pub fn get_updatable_serialization_bytes() -> u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_hll() -> HllSketch {
        let mut hll = HllSketch::default();
        hll.update_i8(-1);
        hll.update_i8(1);
        hll.update_i8(7);
        hll
    }

    #[test]
    fn estimation() {
        let hll = dummy_hll();
        assert_eq!(hll.get_estimate().floor(), 3.0);
    }

    #[test]
    fn serde() {
        let hll = dummy_hll();

        let compact_hll = HllSketch::deserialize(&hll.serialize_compact(0));
        assert_eq!(
            hll.to_string(true, true, true, true),
            compact_hll.to_string(true, true, true, true),
        );

        let updatable_hll = HllSketch::deserialize(&hll.serialize_updatable());
        assert_eq!(
            hll.to_string(true, true, true, true),
            updatable_hll.to_string(true, true, true, true),
        )
    }
}
