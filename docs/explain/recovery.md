<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Exact recovery from folded moments

Convolution associates a weight with each sum of two input indices. Enumerating many pairs can repeat work when many sums coincide. Folding coordinates modulo a bucket count preserves addition, allowing a smaller cyclic convolution to combine the input contributions. Folding alone loses the original index; weighted moments certify when that index can be recovered.

For a bucket with positive weights `w_x`, let `M0 = sum(w_x)`, `M1 = sum(x*w_x)` and `M2 = sum(x*x*w_x)`. Then `M0*M2 - M1*M1 = sum_{x<y} w_x*w_y*(x-y)^2`. Every summand is nonnegative, so equality to zero proves all contributing coordinates are identical. Recover the coordinate as `M1/M0` and its full weight as `M0`. Multiple pairs at the same coordinate do not prevent recovery. Multiple distinct coordinates do.

Input moments combine through six cyclic convolutions: `C0 = A0*B0`, `C1 = A1*B0 + A0*B1`, and `C2 = A2*B0 + 2*A1*B1 + A0*B2`. Coordinates in those moments are the original indices, not folded residues.

## Exact dense primitive

The current implementation packs a folded vector into digits of one arbitrary-precision integer. A digit width strictly larger than the product of the two coefficient masses prevents carries between convolution coefficients. Integer multiplication then computes the linear convolution; folding its upper half gives cyclic convolution. This makes the reference mechanism exact without rounding and without implementing an NTT yet. Its CPU and bit-complexity costs are not the word-RAM dense primitive assumed by the paper.

## Completion and collisions

The implementation tries moduli 17, 67, 257 and 1031. These are bounded coverage choices for the first executable mechanism, not measured crossover thresholds. Certified terms are stored by original index. Finding the same term in another pass never adds its weight a second time. Once the sum of distinct recovered weights equals `sum(left weights) * sum(right weights)`, positivity proves no output weight is missing. If the schedule does not complete, direct accumulation computes the full result.

Fixed modular folding can collide on every pass: indices separated by the product of all four moduli provide a regression test. The direct fallback guarantees termination and exactness. This schedule does not provide Vaughn's collision probability, carry-free lifting, length reduction or expected output-sensitive bound. See [sources](../research/sources.md) and [roadmap](../../ROADMAP.md).

## SIMD boundary

Both engines validate the largest active input indices with required `fearless_simd` dispatch. Zero-weight coordinates are masked before reduction. This scan rejects unrepresentable output indices without building folded moments or pair products. Weight multiplication and ordered accumulation remain scalar; no SIMD speedup is claimed without measurements.
