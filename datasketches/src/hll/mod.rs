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

use datasketches_sys::hll::ffi::target_hll_type;

pub mod sketch;
pub mod union;

pub enum HllType {
    HLL4,
    HLL6,
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
