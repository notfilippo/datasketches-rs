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

//! Refer to [HllUnion].

use crate::macros::*;
use std::ops::Deref;

use cxx::{let_cxx_string, UniquePtr};
use datasketches_sys::hll::ffi::*;

use super::{sketch::HllSketch, HllType};

/// This performs union operations for HLL sketches. This union operator is configured with a
/// `lgMaxK` instead of the normal `lg_config_k`.
///
/// This union operator does permit the unioning of sketches with different values of
/// `lg_config_k`.  The user should be aware that the resulting accuracy of a sketch returned
/// at the end of the unioning process will be a function of the smallest of `lg_max_k` and
/// `lg_config_k` that the union operator has seen.
///
/// This union operator also permits unioning of any of the three different target hll_sketch
/// types.
///
/// Although the API for this union operator parallels many of the methods of the
/// `HllSketch`, the behavior of the union operator has some fundamental differences.
///
/// First, the user cannot specify the [HllType] as an input parameter.
/// Instead, it is specified for the sketch returned with [HllUnion::get_result].
///
/// Second, the internal effective value of log-base-2 of `k` for the union operation can
/// change dynamically based on the smallest `lg_config_k` that the union operation has seen.
///
/// author Jon Malkin
/// author Lee Rhodes
/// author Kevin Lang
pub struct HllUnion(pub(crate) UniquePtr<hll_union>);

impl Default for HllUnion {
    fn default() -> Self {
        Self::new(12)
    }
}

impl HllUnion {
    /// Construct an hll_union operator with the given maximum log2 of k.
    ///
    /// lg_max_k: The maximum size, in log2, of k. The value must
    /// be between 7 and 21, inclusive.
    pub fn new(lg_max_k: u8) -> Self {
        Self(hll_union_new(lg_max_k))
    }

    /// Returns the result of this union operator with the specified
    /// [HllType].
    #[inline]
    pub fn get_result(&self, tgt_type: HllType) -> HllSketch {
        HllSketch(hll_union_get_result(&self.0, tgt_type.into()))
    }

    /// Update this union operator with the given sketch.
    #[inline]
    pub fn update_sketch(&mut self, sketch: &HllSketch) {
        self.0.pin_mut().update_sketch(&sketch.0)
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
        /// Resets the union to an empty state in coupon collection mode.
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
        /// Indicates if the union is currently empty.
        pub fn is_empty() -> bool);
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    fn dummy_hll(cardinality: u64, offset: u64) -> HllSketch {
        let mut hll = HllSketch::default();

        let mut rng = rand::thread_rng();
        for datum in 0..cardinality {
            for _ in 0..rng.gen_range(1..10) {
                hll.update_u64(datum + offset)
            }
        }

        hll
    }

    #[test]
    fn estimation() {
        let a = dummy_hll(200, 0);
        let b = dummy_hll(50, 175);
        let mut union = HllUnion::default();
        union.update_sketch(&a);
        union.update_sketch(&b);
        assert_eq!(union.get_estimate().floor(), 225.0);
    }
}
