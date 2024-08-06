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

use crate::macros::*;
use std::ops::Deref;

use cxx::{let_cxx_string, UniquePtr};
use datasketches_sys::hll::ffi::*;

use super::HllType;

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
    pub fn new(lg_config_k: u8, tgt_type: HllType, start_full_size: bool) -> Self {
        Self(hll_sketch_new(
            lg_config_k,
            tgt_type.into(),
            start_full_size,
        ))
    }

    #[inline]
    pub fn deserialize(bytes: &[u8]) -> Self {
        Self(hll_sketch_deserialize(bytes))
    }

    #[inline]
    pub fn serialize_compact(&self, header_size_bytes: usize) -> Vec<u8> {
        hll_sketch_serialize_compact(&self.0, header_size_bytes)
    }

    #[inline]
    pub fn serialize_updatable(&self) -> Vec<u8> {
        hll_sketch_serialize_updatable(&self.0)
    }

    #[inline]
    pub fn clone_with_type(&self, tgt_type: HllType) -> Self {
        Self(hll_sketch_copy_with_target(&self.0, tgt_type.into()))
    }

    #[inline]
    pub fn to_string(&self, summary: bool, detail: bool, aux_detail: bool, all: bool) -> String {
        hll_sketch_to_string(&self.0, summary, detail, aux_detail, all)
    }

    #[inline]
    pub fn update_string(&mut self, datum: &str) {
        let_cxx_string!(raw_datum = datum);
        self.0.pin_mut().update_string(raw_datum.deref())
    }

    wrap_mut!(reset, ());

    wrap_mut!(update_u64, (), datum: u64);
    wrap_mut!(update_u32, (), datum: u32);
    wrap_mut!(update_u16, (), datum: u16);
    wrap_mut!(update_u8, (), datum: u8);

    wrap_mut!(update_i64, (), datum: i64);
    wrap_mut!(update_i32, (), datum: i32);
    wrap_mut!(update_i16, (), datum: i16);
    wrap_mut!(update_i8, (), datum: i8);

    wrap_mut!(update_f64, (), datum: f64);
    wrap_mut!(update_f32, (), datum: f32);

    wrap!(get_estimate, f64);
    wrap!(get_composite_estimate, f64);
    wrap!(get_lower_bound, f64, num_std_dev: u8);
    wrap!(get_upper_bound, f64, num_std_dev: u8);
    wrap!(is_compact, bool);
    wrap!(is_empty, bool);
    wrap!(get_compact_serialization_bytes, u32);
    wrap!(get_updatable_serialization_bytes, u32);
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
