<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Roadmap

## Implemented foundation

- [x] Exact sparse convolution over `u64` indices and `u128` weights.
- [x] Required fearless_simd input validation, with lane/tail and scalar-oracle checks.
- [x] Exact folded moment recovery with arbitrary-precision packing, deduplication, mass verification and a bounded direct fallback.
- [x] Text CLI, independent exhaustive oracle tests, arithmetic boundaries and adversarial collision coverage.
- [x] Catalog-driven benchmark plans, alternating direct A/A/recovery measurements and output equality.
- [x] Documentation site, Rust/platform/MSRV CI, contribution policy checks and manual release preparation.

## Research gates

1. Translate Vaughn and Bringmann–Fischer–Nakos into a branch-local implementation plan: word widths, hash family, carry handling, length reduction, unknown-support growth, verification and retry bounds. Keep each claim tied to a primary source.
2. Profile whole operations on overlap, distinct-sum and wide-coordinate workloads; include allocation and big-integer cost. Establish controls and A/A ranges before changing defaults.
3. Implement an exact dense backend with independently checked NTT primes, transform bounds and CRT reconstruction, using the required SIMD layer where supported. Preserve exactness for the full public numeric domain or decline internally to a proven fallback.
4. Implement and audit randomized recovery and length reduction. Prove termination/verification separately from expected runtime. A fixed modular schedule does not satisfy this gate.
5. Measure direct, recovery and mature native baselines with versioned input identity, placement controls and current-head artifacts. Identify actual CPU crossover regions without restricting claims to handpicked winners.
6. Add automatic selection only after the preceding measurements. Document its memory bounds and update the explanation, conformance and decision record.

## Product gates

- [ ] Signed-lag sparse correlation with checked signed coordinate arithmetic.
- [ ] Sumset convenience API only if it removes real caller work beyond unit weights.
- [ ] Stable API, fuzzing corpus and release acceptance report.
- [ ] crates.io publication and signed/versioned releases after explicit maintainer authorization.
- [ ] Continuous controlled benchmark infrastructure and shared figure templates when repeatable measurement volume justifies them.

Do not add Python/C++ bindings, a distributed benchmark scheduler or release automation for unimplemented packages merely to mirror another repository's size.
