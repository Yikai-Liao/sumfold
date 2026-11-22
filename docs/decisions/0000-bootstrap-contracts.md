<!-- SPDX-License-Identifier: BSD-2-Clause -->
# 0000: Bootstrap contracts

Status: accepted for the initial repository, before its first pull request.

Use borrowed lists of `Term { index: u64, weight: u128 }` and return canonical exact terms or arithmetic errors. An owned normalized histogram would add preparation and representation obligations without a demonstrated reuse workload. Arbitrary-precision public weights would remove overflow but also make every caller pay for a larger representation. Keep arbitrary precision internal to experimental recovery.

Require `fearless_simd` in every build as requested by the maintainer. It owns input maximum scans with runtime dispatch and no feature switch. Set MSRV to 1.89 to cover that dependency. Keep exact weight multiplication and ordered accumulation scalar until measurements identify useful SIMD work there.

Keep `convolve` as a transparent direct baseline and expose `convolve_recovering` through an independent research feature. That feature controls only the big-integer backend, never SIMD. The recovery operation owns certification, deduplication, completeness and fallback. Fixed modular folding is intentionally described as an executable mechanism; claiming Vaughn's bound for this schedule would be incorrect.

Mirror Tessera's contract authority, documentation taxonomy, catalog/plans, tool ownership, signed contributions and current-head CI discipline. Start with a small working matrix and manual source preparation; do not copy workflows for absent language packages or dedicated benchmark infrastructure.

Verification includes exhaustive small histograms against an independent oracle, active/zero index boundaries, coefficient and global-mass boundaries, adversarial persistent collisions, packed convolution and SIMD lanes/tails. Performance remains unclaimed until controlled measurements exist.
