<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Sumfold

**Exact sparse nonnegative integer convolution.**

Sumfold combines integer histograms without allocating an array spanning their coordinates. A term has a `u64` index and a `u128` weight; each pair contributes the product of its weights at the sum of its indices. Inputs may be unordered and contain duplicates or zeros. Results are sorted, unique and exact, or return an explicit overflow error.

```rust
use sumfold::{Term, convolve};

let left = [Term::new(0, 2), Term::new(10, 3)];
let right = [Term::new(1, 4), Term::new(11, 5)];
assert_eq!(convolve(&left, &right)?, vec![
    Term::new(1, 8), Term::new(11, 22), Term::new(21, 15),
]);
# Ok::<(), sumfold::Error>(())
```

## Status

Pre-release. The repository contains a checked direct engine, an optional experimental exact moment-recovery engine, a text CLI, conformance tests, a benchmark harness and a documentation site. **The output-sensitive Las Vegas algorithm and its expected `O(t log t)` bound are research goals, not properties of this release.** No performance advantage is claimed.

`fearless_simd` is a required dependency used for runtime-dispatched input scans. It has no feature switch. Rust 1.89 or newer is required. The `recovery` feature adds `num-bigint` for exact folded moments and packed integer convolution; it does not control SIMD. The direct implementation remains the default API until measurements justify automatic engine selection.

## Try it

The crate is not published to crates.io. Use a Git dependency pinned to a reviewed commit, or build this checkout:

```sh
cargo test --workspace --all-targets
cargo run -p sumfold-cli -- convolve examples/left.txt examples/right.txt
cargo run -p sumfold-cli -- convolve --recover examples/left.txt examples/right.txt
```

Both commands print:

```text
1 8
11 22
21 15
```

Set weights to one to count representations in an integer sumset. Histogram weights also represent exact counts for sums of independent discrete quantities. Signed-lag correlation and probability normalization are planned; they are not separate APIs yet.

## Learn and contribute

- [Semantics](conformance/semantics.md): exactness, duplicates, overflow and memory.
- [Recovery mechanism](docs/explain/recovery.md): folding, moments and completeness.
- [Architecture](docs/design/engine.md) and [research sources](docs/research/sources.md).
- [CLI](docs/guide/cli.md), [benchmarks](docs/development/benchmarking.md), [CI](docs/development/ci.md).
- [Contributing](CONTRIBUTING.md), [roadmap](ROADMAP.md), [release process](RELEASING.md).

BSD-2-Clause. The project structure and maintenance conventions follow [Tessera](https://github.com/Yikai-Liao/tessera-sort); the convolution implementation is original.
