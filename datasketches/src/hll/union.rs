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

use super::{sketch::HllSketch, HllType};

pub struct HllUnion(pub(crate) UniquePtr<hll_union>);

impl Default for HllUnion {
    fn default() -> Self {
        Self::new(12)
    }
}

impl HllUnion {
    pub fn new(lg_config_k: u8) -> Self {
        Self(hll_union_new(lg_config_k))
    }

    #[inline]
    pub fn get_result(&self, tgt_type: HllType) -> HllSketch {
        HllSketch(hll_union_get_result(&self.0, tgt_type.into()))
    }

    #[inline]
    pub fn update_sketch(&mut self, sketch: &HllSketch) {
        self.0.pin_mut().update_sketch(&sketch.0)
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
    wrap!(is_empty, bool);
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
