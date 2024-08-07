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

use std::path::Path;

fn main() {
    let vendor_modules = [
        "common",
        "count",
        "cpc",
        "density",
        "fi",
        "hll",
        "kll",
        "quantiles",
        "req",
        "sampling",
        "tdigest",
        "theta",
        "tuple",
    ];

    let vendor_path = Path::new("datasketches-cpp");
    let vendor_includes = vendor_modules
        .iter()
        .map(|module| vendor_path.join(module).join("include"));

    cxx_build::bridges(["src/hll.rs", "src/cpc.rs"]) // returns a cc::Build
        .includes(vendor_includes)
        .include("src")
        .std("c++17")
        .compile("datasketches");

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=wrappers");
}
