# The Apache DataSketches Library for Rust

[<img src="https://img.shields.io/crates/v/datasketches" alt="Crates.io Version" />](https://crates.io/crates/datasketches)
[<img src="https://img.shields.io/crates/l/datasketches" alt="Crates.io License" />](https://crates.io/crates/datasketches)
[<img src="https://img.shields.io/docsrs/datasketches" alt="docs.rs Documentation" />](https://docs.rs/datasketches/latest/datasketches/)

This is the core Rust component of the Apache DataSketches library. It contains some of the key sketching algorithms that are in the C++ component and can be accessed directly from user applications.

Note that Apache has a parallel core component for [C++](https://github.com/apache/datasketches-cpp), [Java](https://github.com/apache/datasketches-java), and [Python](https://github.com/apache/datasketches-python) implementations of the same sketch algorithms.

### Supported sketches

| Name                                                     | Supported |
| -------------------------------------------------------- | --------- |
| KLL (Absolute Error Quantiles)                           | -         |
| `kll_ints_sketch`                                        | no        |
| `kll_floats_sketch`                                      | no        |
| `kll_doubles_sketch`                                     | no        |
| `kll_items_sketch`                                       | no        |
| Quantiles (Absolute Error Quantiles, inferior algorithm) | -         |
| `quantiles_ints_sketch`                                  | no        |
| `quantiles_floats_sketch`                                | no        |
| `quantiles_doubles_sketch`                               | no        |
| `quantiles_items_sketch`                                 | no        |
| REQ (Relative Error Quantiles)                           | -         |
| `req_ints_sketch`                                        | no        |
| `req_floats_sketch`                                      | no        |
| `req_items_sketch`                                       | no        |
| Frequent Items                                           | -         |
| `frequent_strings_sketch`                                | no        |
| `frequent_items_sketch`                                  | no        |
| Theta                                                    | -         |
| `update_theta_sketch`                                    | no        |
| `compact_theta_sketch`                                   | no        |
| `theta_union`                                            | no        |
| `theta_intersection`                                     | no        |
| `theta_a_not_b`                                          | no        |
| `theta_jaccard_similarity`                               | no        |
| Tuple                                                    | -         |
| `update_tuple_sketch`                                    | no        |
| `compact_tuple_sketch`                                   | no        |
| `tuple_union`                                            | no        |
| `tuple_intersection`                                     | no        |
| `tuple_a_not_b`                                          | no        |
| `tuple_jaccard_similarity`                               | no        |
| HLL                                                      | -         |
| `hll_sketch`                                             | ✅         |
| `hll_union`                                              | ✅         |
| CPC                                                      | -         |
| `cpc_sketch`                                             | ✅         |
| `cpc_union`                                              | ✅         |
| VarOpt Sampling                                          | -         |
| `var_opt_sketch`                                         | no        |
| `var_opt_union`                                          | no        |
| EBPPS Sampling (Exactly proportional to weight)          | -         |
| `ebpps_sketch`                                           | no        |
| Vector of KLL                                            | -         |
| `vector_of_kll_ints_sketches`                            | no        |
| `vector_of_kll_floats_sketches`                          | no        |
| Kolmogorov-Smirnov Test                                  | -         |
| `ks_test`                                                | no        |
| Density                                                  | -         |
| `density_sketch`                                         | no        |
| Count-min sketch                                         | -         |
| `count_min_sketch`                                       | no        |