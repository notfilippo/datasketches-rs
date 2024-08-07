/**
 * Copyright 2024 Filippo Rossi
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once
#include "cpc_sketch.hpp"
#include "cpc_union.hpp"
#include "rust/cxx.h"

namespace datasketches
{
  inline std::unique_ptr<cpc_sketch> cpc_sketch_new(uint8_t lg_k = cpc_constants::DEFAULT_LG_K, uint64_t seed = DEFAULT_SEED)
  {
    return std::unique_ptr<cpc_sketch>(new cpc_sketch(lg_k, seed));
  }

  inline std::unique_ptr<cpc_sketch> cpc_sketch_copy(const cpc_sketch &self)
  {
    return std::unique_ptr<cpc_sketch>(new cpc_sketch(self));
  }

  inline std::unique_ptr<cpc_sketch> cpc_sketch_deserialize(rust::Slice<const uint8_t> bytes)
  {
    return std::unique_ptr<cpc_sketch>(new cpc_sketch(cpc_sketch::deserialize((const void *)(bytes.data()), bytes.length())));
  }

  rust::Vec<uint8_t> cpc_sketch_serialize(const cpc_sketch &self)
  {
    cpc_sketch::vector_bytes bytes = self.serialize();
    rust::Vec<uint8_t> vec;
    std::move(bytes.begin(), bytes.end(), std::back_inserter(vec));
    return vec;
  }

  rust::String cpc_sketch_to_string(const cpc_sketch &self)
  {
    return rust::String(self.to_string());
  }

  inline std::unique_ptr<cpc_union> cpc_union_new(uint8_t lg_k = cpc_constants::DEFAULT_LG_K, uint64_t seed = DEFAULT_SEED)
  {
    return std::unique_ptr<cpc_union>(new cpc_union(lg_k, seed));
  }

  inline std::unique_ptr<cpc_union> cpc_union_copy(const cpc_union &self)
  {
    return std::unique_ptr<cpc_union>(new cpc_union(self));
  }

  inline std::unique_ptr<cpc_sketch> cpc_union_get_result(const cpc_union &self)
  {
    return std::unique_ptr<cpc_sketch>(new cpc_sketch(self.get_result()));
  }
}