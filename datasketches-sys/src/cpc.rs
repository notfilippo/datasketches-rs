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

pub const DEFAULT_SEED: u64 = 9001;

pub const DEFAULT_LG_K: u8 = 11;
pub const MIN_LG_K: u8 = 4;
pub const MAX_LG_K: u8 = 26;

#[cxx::bridge(namespace = "datasketches")]
pub mod ffi {
    unsafe extern "C++" {
        include!("datasketches-sys/src/cpc.hh");
        pub type cpc_sketch;

        fn cpc_sketch_new(lg_k: u8, seed: u64) -> UniquePtr<cpc_sketch>;
        fn cpc_sketch_copy(sketch: &cpc_sketch) -> UniquePtr<cpc_sketch>;

        fn cpc_sketch_deserialize(bytes: &[u8]) -> UniquePtr<cpc_sketch>;
        fn cpc_sketch_serialize(sketch: &cpc_sketch) -> Vec<u8>;

        fn cpc_sketch_to_string(sketch: &cpc_sketch) -> String;

        #[rust_name=update_string]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: &CxxString);
        #[rust_name=update_u64]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: u64);
        #[rust_name=update_u32]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: u32);
        #[rust_name=update_u16]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: u16);
        #[rust_name=update_u8]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: u8);
        #[rust_name=update_i64]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: i64);
        #[rust_name=update_i32]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: i32);
        #[rust_name=update_i16]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: i16);
        #[rust_name=update_i8]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: i8);
        #[rust_name=update_f64]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: f64);
        #[rust_name=update_f32]
        pub fn update(self: Pin<&mut cpc_sketch>, datum: f32);

        pub fn get_estimate(&self) -> f64;
        pub fn get_lower_bound(&self, kappa: u32) -> f64;
        pub fn get_upper_bound(&self, kappa: u32) -> f64;
        pub fn get_lg_k(&self) -> u8;
        pub fn is_empty(&self) -> bool;
    }

    unsafe extern "C++" {
        include!("datasketches-sys/src/cpc.hh");
        pub type cpc_union;

        fn cpc_union_new(lg_k: u8, seed: u64) -> UniquePtr<cpc_union>;
        fn cpc_union_copy(union_: &cpc_union) -> UniquePtr<cpc_union>;
        fn cpc_union_get_result(union_: &cpc_union) -> UniquePtr<cpc_sketch>;

        #[rust_name=update_sketch]
        pub fn update(self: Pin<&mut cpc_union>, sketch: &cpc_sketch);
    }
}

#[cfg(test)]
mod tests {
    use cxx::UniquePtr;

    use super::*;

    fn dummy_cpc() -> UniquePtr<ffi::cpc_sketch> {
        let mut cpc = ffi::cpc_sketch_new(DEFAULT_LG_K, DEFAULT_SEED);
        cpc.pin_mut().update_i8(-1);
        cpc.pin_mut().update_i8(1);
        cpc.pin_mut().update_i8(7);
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

        let compact_cpc = ffi::cpc_sketch_deserialize(&ffi::cpc_sketch_serialize(&cpc));
        assert_eq!(
            ffi::cpc_sketch_to_string(&cpc),
            ffi::cpc_sketch_to_string(&compact_cpc),
        );
    }
}
