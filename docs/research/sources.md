<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Research sources and implementation boundary

The following primary sources were checked on November 22, 2026. Read their assumptions before transferring an asymptotic statement to a fixed-width CPU implementation.

1. Trevor Vaughn, [A Simple Las Vegas Algorithm for Sparse Nonnegative Convolution](https://arxiv.org/abs/2608.16123), submitted August 17, 2026. The abstract states expected `O(t log t)` time and a weaker tail bound than Jin–Xu. The construction uses dense convolution, linear hashing, length reduction and constant-dimensional carry-free index representations. Sections 3–4 are the starting point for implementation research. Sumfold's fixed modular schedule does not implement that construction.
2. Karl Bringmann, Nick Fischer and Vasileios Nakos, [Deterministic and Las Vegas Algorithms for Sparse Nonnegative Convolution](https://arxiv.org/abs/2107.07625), 2021 preprint. Includes a simpler Las Vegas `O(t log² t)` algorithm and a refined `O(t log t log log t)` algorithm. This is the source to study alongside the moment identities and length reduction cited by Vaughn.
3. Karl Bringmann, Nick Fischer and Vasileios Nakos, [Sparse Nonnegative Convolution Is Equivalent to Dense Nonnegative Convolution](https://arxiv.org/abs/2105.05984), 2021 preprint. Its bounded-error reduction and assumptions differ from the exact, always-correct recovery target; do not interchange their guarantees.

The current library implements exact integer pair accumulation and the zero-variance recovery mechanism using big-integer packed multiplication. It has no random hash family, length reduction, carry-free coordinate lifting, NTT/CRT engine or proof of output-sensitive runtime. Its complete fallback may enumerate all pairs. The repository intentionally makes these boundaries visible in API documentation, the introduction and the roadmap.

The idea also mentions approximate Hamming-distance work and FLINT/SciPy comparisons. They are useful future context, but they do not validate this implementation's correctness or speed. Do not repeat comparative performance or absence claims without checking the relevant current sources and workloads.
