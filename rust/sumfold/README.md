<!-- SPDX-License-Identifier: BSD-2-Clause -->
# sumfold

Exact sparse nonnegative integer convolution. See the [repository](https://github.com/Yikai-Liao/sumfold) for examples, semantics, measurements and the implementation roadmap.

`fearless_simd` is a mandatory dependency; Rust 1.89 or newer is required. `convolve` is the checked direct baseline. The `recovery` feature adds experimental exact moment recovery with a complete direct fallback; it does not control SIMD and does not claim output-sensitive runtime. This crate is not yet published.
