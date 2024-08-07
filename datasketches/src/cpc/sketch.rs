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

//! Refer to [CpcSketch].

use crate::macros::*;
use std::ops::Deref;

use cxx::{let_cxx_string, UniquePtr};
use datasketches_sys::cpc::{ffi::*, DEFAULT_LG_K, DEFAULT_SEED};

/// High performance C++ implementation of Compressed Probabilistic Counting (CPC) Sketch.
///
/// This is a very compact (in serialized form) distinct counting sketch.
/// The theory is described in the following paper:
/// [arxiv.org/abs/1708.06839](https://arxiv.org/abs/1708.06839)
///
/// - author Kevin Lang
/// - author Alexander Saydakov
pub struct CpcSketch(pub(crate) UniquePtr<cpc_sketch>);

impl Default for CpcSketch {
    fn default() -> Self {
        Self::new(DEFAULT_LG_K, DEFAULT_SEED)
    }
}

impl Clone for CpcSketch {
    fn clone(&self) -> Self {
        Self(cpc_sketch_copy(&self.0))
    }
}

impl std::fmt::Display for CpcSketch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", cpc_sketch_to_string(&self.0))
    }
}

impl CpcSketch {
    /// Creates an instance of the sketch given the lg_k parameter and hash seed.
    pub fn new(lg_k: u8, seed: u64) -> Self {
        Self(cpc_sketch_new(lg_k, seed))
    }

    /// Reconstructs a sketch from a serialized image in a byte array.
    #[inline]
    pub fn deserialize(bytes: &[u8]) -> Self {
        Self(cpc_sketch_deserialize(bytes))
    }

    /// This method serializes the sketch as a vector of bytes.
    #[inline]
    pub fn serialize(&self) -> Vec<u8> {
        cpc_sketch_serialize(&self.0)
    }

    /// Present the given string as a potential unique item.
    /// The string is converted to a byte array using UTF8 encoding.
    /// If the string is null or empty no update attempt is made and the method returns.
    #[inline]
    pub fn update_string(&mut self, datum: &str) {
        let_cxx_string!(raw_datum = datum);
        self.0.pin_mut().update_string(raw_datum.deref());
    }

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
        /// Returns the approximate lower error bound given a parameter kappa (1, 2 or 3).
        /// This parameter is similar to the number of standard deviations of the normal distribution
        /// and corresponds to approximately 67%, 95% and 99% confidence intervals.
        pub fn get_lower_bound(kappa: u32) -> f64);
    wrap!(
        /// Returns the approximate upper error bound given a parameter kappa (1, 2 or 3).
        /// This parameter is similar to the number of standard deviations of the normal distribution
        /// and corresponds to approximately 67%, 95% and 99% confidence intervals.
        pub fn get_upper_bound(kappa: u32) -> f64);
    wrap!(
        /// Indicates if the sketch is currently empty.
        pub fn is_empty() -> bool);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_cpc() -> CpcSketch {
        let mut cpc = CpcSketch::default();
        cpc.update_i8(-1);
        cpc.update_i8(1);
        cpc.update_i8(7);
        cpc
    }

    #[test]
    fn estimation() {
        let cpc = dummy_cpc();
        assert_eq!(cpc.get_estimate().floor(), 3.0);
    }

    #[test]
    fn serde() {
        let cpc = dummy_cpc();

        let compact_cpc = CpcSketch::deserialize(&cpc.serialize());
        assert_eq!(cpc.to_string(), compact_cpc.to_string(),);
    }
}
