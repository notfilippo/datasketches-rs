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
#include "hll.hpp"
#include "rust/cxx.h"

namespace datasketches
{
  inline std::unique_ptr<hll_sketch> hll_sketch_new(uint8_t lg_config_k, target_hll_type tgt_type = HLL_4, bool start_full_size = false)
  {
    return std::unique_ptr<hll_sketch>(new hll_sketch(lg_config_k, tgt_type, start_full_size));
  }

  inline std::unique_ptr<hll_sketch> hll_sketch_copy(const hll_sketch &self)
  {
    return std::unique_ptr<hll_sketch>(new hll_sketch(self));
  }

  inline std::unique_ptr<hll_sketch> hll_sketch_copy_with_target(const hll_sketch &self, target_hll_type tgt_type)
  {
    return std::unique_ptr<hll_sketch>(new hll_sketch(self, tgt_type));
  }

  inline std::unique_ptr<hll_sketch> hll_sketch_deserialize(rust::Slice<const uint8_t> bytes)
  {
    return std::unique_ptr<hll_sketch>(new hll_sketch(hll_sketch::deserialize((const void *)(bytes.data()), bytes.length())));
  }

  rust::Vec<uint8_t> hll_sketch_serialize_compact(const hll_sketch &self, unsigned header_size_bytes = 0)
  {
    hll_sketch::vector_bytes bytes = self.serialize_compact(header_size_bytes);
    rust::Vec<uint8_t> vec;
    std::move(bytes.begin(), bytes.end(), std::back_inserter(vec));
    return vec;
  }

  rust::Vec<uint8_t> hll_sketch_serialize_updatable(const hll_sketch &self)
  {
    hll_sketch::vector_bytes bytes = self.serialize_updatable();
    rust::Vec<uint8_t> vec;
    std::move(bytes.begin(), bytes.end(), std::back_inserter(vec));
    return vec;
  }

  rust::String hll_sketch_to_string(const hll_sketch &self,
                                    const bool summary,
                                    const bool detail,
                                    const bool aux_detail,
                                    const bool all)
  {
    return rust::String(self.to_string(summary, detail, aux_detail, all));
  }

  inline std::unique_ptr<hll_union> hll_union_new(uint8_t lg_config_k)
  {
    return std::unique_ptr<hll_union>(new hll_union(lg_config_k));
  }

  inline std::unique_ptr<hll_union> hll_union_copy(const hll_union &self)
  {
    return std::unique_ptr<hll_union>(new hll_union(self));
  }

  inline std::unique_ptr<hll_sketch> hll_union_get_result(const hll_union &self, target_hll_type tgt_type = HLL_4)
  {
    return std::unique_ptr<hll_sketch>(new hll_sketch(self.get_result(tgt_type)));
  }

  inline double hll_rel_error(bool upper_bound, bool unioned,
                              uint8_t lg_config_k, uint8_t num_std_dev)
  {
    return HllUtil<>::getRelErr(upper_bound, unioned, lg_config_k, num_std_dev);
  }
}