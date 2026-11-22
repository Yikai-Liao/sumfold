<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Arithmetic and output semantics

This document is the behavior authority for all future language bindings.

## Representation

An input is a finite list of terms `(index, weight)` with `index` in `[0, 2^64 - 1]` and `weight` in `[0, 2^128 - 1]`. Lists may be unordered, contain duplicate indices and include zeros. A zero term contributes nothing and its index cannot trigger an arithmetic error. Inputs are borrowed and never modified.

For each output index `s`, the coefficient is the exact integer sum of `left.weight * right.weight` over all positive-weight input pairs whose indices sum to `s`. Duplicate input indices contribute independently. Output terms have strictly increasing indices and positive weights. There is exactly one term per output index.

## Errors and empty inputs

If either side has no positive weights, return an empty vector. Otherwise return `IndexOverflow` if a positive pair's index sum exceeds `u64::MAX`, or `WeightOverflow` if any output coefficient exceeds `u128::MAX`. When both occur, either error is permitted. Global input mass and global output mass need not fit `u128`; only individual output coefficients do. No arithmetic wraps or saturates.

An arithmetic failure returns no partial result. Allocation failures follow the Rust allocator's behavior and are not represented by `Error`. The implementation does not promise fallible allocation or a user-set memory cap.

## Engines

`convolve` enumerates input pairs and aggregates into an ordered map. It takes `O(a + b + a*b*log(t + 1))` time and `O(t)` auxiliary memory for input lengths `a, b` and output support `t`.

With `recovery`, `convolve_recovering` first tries exact folded singleton recovery at four fixed moduli and falls back to the direct engine if recovery is incomplete. It returns the same result and errors. Moment arithmetic is arbitrary precision. Every recovered coefficient is proved by the zero-variance identity; the sum of deduplicated recovered weights must equal the exact product of input masses before returning early. Randomness is not used. Neither the Las Vegas property nor output-sensitive expected time is claimed for this deterministic implementation.

`fearless_simd` is required for input index scans in both engines; it is independent of `recovery`. SIMD backend choice cannot change arithmetic semantics. The project does not allocate storage proportional to the full coordinate span.

## CLI

The [CLI protocol](../docs/guide/cli.md) uses the same integer widths and semantics. Parse or arithmetic errors produce no stdout result and exit with a nonzero status. I/O failures during result writing may leave a partial stream; atomic output is not promised for those failures.
