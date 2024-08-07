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

//! Refer to [CpcUnion].

use cxx::UniquePtr;
use datasketches_sys::cpc::{ffi::*, DEFAULT_LG_K, DEFAULT_SEED};

use super::sketch::CpcSketch;

/// High performance C++ implementation of Compressed Probabilistic Counting (CPC) Union.
///
/// - author Kevin Lang
/// - author Alexander Saydakov
pub struct CpcUnion(pub(crate) UniquePtr<cpc_union>);

impl Default for CpcUnion {
    fn default() -> Self {
        Self::new(DEFAULT_LG_K, DEFAULT_SEED)
    }
}

impl CpcUnion {
    /// Creates an instance of the union given the lg_k parameter and hash seed.
    pub fn new(lg_k: u8, seed: u64) -> Self {
        Self(cpc_union_new(lg_k, seed))
    }

    /// This method produces a copy of the current state of the union as a sketch.
    #[inline]
    pub fn get_result(&self) -> CpcSketch {
        CpcSketch(cpc_union_get_result(&self.0))
    }

    /// Update this union operator with the given sketch.
    #[inline]
    pub fn update_sketch(&mut self, sketch: &CpcSketch) {
        self.0.pin_mut().update_sketch(&sketch.0);
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    fn dummy_cpc(cardinality: u64, offset: u64) -> CpcSketch {
        let mut cpc = CpcSketch::default();

        let mut rng = rand::thread_rng();
        for datum in 0..cardinality {
            for _ in 0..rng.gen_range(1..10) {
                cpc.update_u64(datum + offset)
            }
        }

        cpc
    }

    #[test]
    fn estimation() {
        let a = dummy_cpc(200, 0);
        let b = dummy_cpc(50, 175);
        let mut union = CpcUnion::default();
        union.update_sketch(&a);
        union.update_sketch(&b);
        let estimate = union.get_result().get_estimate().floor();
        assert!(estimate >= 224.0 && estimate <= 226.0);
    }
}
