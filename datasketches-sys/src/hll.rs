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

pub const DEFAULT_LG_CONFIG_K: u8 = 12;
pub const MIN_LG_CONFIG_K: u8 = 7;
pub const MAX_LG_CONFIG_K: u8 = 21;

#[cxx::bridge(namespace = "datasketches")]
pub mod ffi {
    #[repr(i32)]
    enum target_hll_type {
        HLL_4,
        HLL_6,
        HLL_8,
    }

    unsafe extern "C++" {
        include!("datasketches-sys/src/hll.hh");
        pub type target_hll_type;

        fn hll_rel_error(upper_bound: bool, unioned: bool, lg_config_k: u8, num_std_dev: u8)
            -> f64;
    }

    unsafe extern "C++" {
        include!("datasketches-sys/src/hll.hh");
        pub type hll_sketch;

        fn hll_sketch_new(
            lg_config_k: u8,
            tgt_type: target_hll_type,
            start_full_size: bool,
        ) -> UniquePtr<hll_sketch>;

        fn hll_sketch_copy(sketch: &hll_sketch) -> UniquePtr<hll_sketch>;
        fn hll_sketch_copy_with_target(
            sketch: &hll_sketch,
            tgt_type: target_hll_type,
        ) -> UniquePtr<hll_sketch>;

        fn hll_sketch_deserialize(bytes: &[u8]) -> UniquePtr<hll_sketch>;
        fn hll_sketch_serialize_compact(sketch: &hll_sketch, header_size_bytes: u32) -> Vec<u8>;
        fn hll_sketch_serialize_updatable(sketch: &hll_sketch) -> Vec<u8>;

        fn hll_sketch_to_string(
            sketch: &hll_sketch,
            summary: bool,
            detail: bool,
            aux_detail: bool,
            all: bool,
        ) -> String;

        pub fn reset(self: Pin<&mut hll_sketch>);

        #[rust_name=update_string]
        pub fn update(self: Pin<&mut hll_sketch>, datum: &CxxString);
        #[rust_name=update_u64]
        pub fn update(self: Pin<&mut hll_sketch>, datum: u64);
        #[rust_name=update_u32]
        pub fn update(self: Pin<&mut hll_sketch>, datum: u32);
        #[rust_name=update_u16]
        pub fn update(self: Pin<&mut hll_sketch>, datum: u16);
        #[rust_name=update_u8]
        pub fn update(self: Pin<&mut hll_sketch>, datum: u8);
        #[rust_name=update_i64]
        pub fn update(self: Pin<&mut hll_sketch>, datum: i64);
        #[rust_name=update_i32]
        pub fn update(self: Pin<&mut hll_sketch>, datum: i32);
        #[rust_name=update_i16]
        pub fn update(self: Pin<&mut hll_sketch>, datum: i16);
        #[rust_name=update_i8]
        pub fn update(self: Pin<&mut hll_sketch>, datum: i8);
        #[rust_name=update_f64]
        pub fn update(self: Pin<&mut hll_sketch>, datum: f64);
        #[rust_name=update_f32]
        pub fn update(self: Pin<&mut hll_sketch>, datum: f32);

        pub fn get_estimate(&self) -> f64;
        pub fn get_composite_estimate(&self) -> f64;
        pub fn get_lower_bound(&self, num_std_dev: u8) -> f64;
        pub fn get_upper_bound(&self, num_std_dev: u8) -> f64;
        pub fn get_lg_config_k(&self) -> u8;
        pub fn get_target_type(&self) -> target_hll_type;
        pub fn is_compact(&self) -> bool;
        pub fn is_empty(&self) -> bool;
        pub fn get_compact_serialization_bytes(&self) -> u32;
        pub fn get_updatable_serialization_bytes(&self) -> u32;
    }

    unsafe extern "C++" {
        include!("datasketches-sys/src/hll.hh");
        pub type hll_union;

        fn hll_union_new(lg_config_k: u8) -> UniquePtr<hll_union>;
        fn hll_union_copy(union_: &hll_union) -> UniquePtr<hll_union>;
        fn hll_union_get_result(
            union_: &hll_union,
            tgt_type: target_hll_type,
        ) -> UniquePtr<hll_sketch>;

        pub fn reset(self: Pin<&mut hll_union>);

        #[rust_name=update_sketch]
        pub fn update(self: Pin<&mut hll_union>, sketch: &hll_sketch);
        #[rust_name=update_string]
        pub fn update(self: Pin<&mut hll_union>, datum: &CxxString);
        #[rust_name=update_u64]
        pub fn update(self: Pin<&mut hll_union>, datum: u64);
        #[rust_name=update_u32]
        pub fn update(self: Pin<&mut hll_union>, datum: u32);
        #[rust_name=update_u16]
        pub fn update(self: Pin<&mut hll_union>, datum: u16);
        #[rust_name=update_u8]
        pub fn update(self: Pin<&mut hll_union>, datum: u8);
        #[rust_name=update_i64]
        pub fn update(self: Pin<&mut hll_union>, datum: i64);
        #[rust_name=update_i32]
        pub fn update(self: Pin<&mut hll_union>, datum: i32);
        #[rust_name=update_i16]
        pub fn update(self: Pin<&mut hll_union>, datum: i16);
        #[rust_name=update_i8]
        pub fn update(self: Pin<&mut hll_union>, datum: i8);
        #[rust_name=update_f64]
        pub fn update(self: Pin<&mut hll_union>, datum: f64);
        #[rust_name=update_f32]
        pub fn update(self: Pin<&mut hll_union>, datum: f32);

        pub fn get_estimate(&self) -> f64;
        pub fn get_composite_estimate(&self) -> f64;
        pub fn get_lower_bound(&self, num_std_dev: u8) -> f64;
        pub fn get_upper_bound(&self, num_std_dev: u8) -> f64;
        pub fn get_lg_config_k(&self) -> u8;
        pub fn get_target_type(&self) -> target_hll_type;
        pub fn is_empty(&self) -> bool;
    }
}

#[cfg(test)]
mod tests {
    use cxx::UniquePtr;

    use super::*;

    fn dummy_hll() -> UniquePtr<ffi::hll_sketch> {
        let mut hll = ffi::hll_sketch_new(12, ffi::target_hll_type::HLL_4, false);
        hll.pin_mut().update_i8(-1);
        hll.pin_mut().update_i8(1);
        hll.pin_mut().update_i8(7);
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

        let compact_hll = ffi::hll_sketch_deserialize(&ffi::hll_sketch_serialize_compact(&hll, 0));
        assert_eq!(
            ffi::hll_sketch_to_string(&hll, true, true, true, true),
            ffi::hll_sketch_to_string(&compact_hll, true, true, true, true),
        );

        let updatable_hll = ffi::hll_sketch_deserialize(&ffi::hll_sketch_serialize_updatable(&hll));
        assert_eq!(
            ffi::hll_sketch_to_string(&hll, true, true, true, true),
            ffi::hll_sketch_to_string(&updatable_hll, true, true, true, true),
        )
    }
}
