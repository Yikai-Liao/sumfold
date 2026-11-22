<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Engine architecture

The public contract has one term representation and two complete operations with identical results: `convolve` and, under `recovery`, `convolve_recovering`. The alternative of a normalized histogram object would make every caller sort and aggregate before invoking the operation, while adding an owned representation without an established repeated-use workload. Borrowed term slices keep canonicalization and overflow responsibilities inside the operation. Revisit an owned type only with reuse measurements.

`lib.rs` owns the public representation, errors and direct ordered accumulation. `scan.rs` owns the required runtime-dispatched index maximum, including zero-weight masking. Both engines use this scan; no SIMD build feature or separate interpretation of overflow exists. The configured MSRV covers the required dependency.

`recovery.rs` owns folding, exact dense multiplication, singleton certification, deduplication, completeness and fallback. Callers never manage partial recovered output or coordinate the fallback. `num-bigint` stays private behind `recovery`, while the mandatory SIMD dependency serves all builds. No project code uses unsafe blocks.

The CLI owns file parsing and diagnostics, then calls a complete library operation. Benchmark workloads call those same public operations and compare outputs; they do not maintain an independent recovery engine. An independently written sort-and-aggregate oracle lives only in conformance tests.

## Vocabulary

A **term** is an index and weight. An **index** identifies a nonnegative coordinate. A **weight** is a nonnegative integer coefficient. A **moment** is a weighted power sum of original indices. A **bucket** contains contributions sharing a folded index. **Recovered mass** counts the weights of distinct certified output indices. **Support** is the number of positive output indices.

## Defaults and future routing

Direct accumulation is the current default because an automatic crossover has not been measured. Recovery is explicitly selected, with a bounded schedule and direct fallback. Adding NTT/CRT, randomized lifting or algorithm selection requires equality coverage, exact arithmetic bounds, resource accounting and the benchmark controls. A floating-point convolution cannot become an exact backend through a tolerance check.
