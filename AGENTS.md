<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Instructions for coding agents

Read [CONTRIBUTING.md](CONTRIBUTING.md). Sumfold is an exact sparse convolution library being developed toward output-sensitive recovery. Keep implemented behavior, unmeasured hypotheses and paper guarantees separate. Everything in the repository is in English.

## Workspace and disk

Use one Git-ignored task directory under `target/scratch/` for logs, profiles, downloads and experiments. `/tmp` is shared RAM-backed storage; never put builds or large artifacts there. Reuse normal build caches and check `df -h` before multi-gigabyte work. Delete only your own scratch and stop processes you started before reporting; name the path and size of anything retained. Do not use subagents unless the user explicitly requests them.

## Layout

| Path | Responsibility |
| --- | --- |
| `rust/sumfold/src/lib.rs` | Public contract and checked direct engine |
| `rust/sumfold/src/scan.rs` | Required fearless_simd input scans |
| `rust/sumfold/src/recovery.rs` | Exact folded moments and complete-operation fallback |
| `rust/sumfold-cli/` | Text protocol and process diagnostics |
| `conformance/semantics.md` | Cross-language authority for arithmetic and output |
| `bench/` | Catalog, plans, CLI and Rust timing harness |
| `docs/` | Guides, explanation, design, research and development |
| `.github/` | CI and contribution policy enforcement |

## Checks

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --workspace --doc --locked
cargo test -p sumfold --no-default-features --locked
cargo doc --workspace --all-features --no-deps
pnpm install --frozen-lockfile
pnpm lint
pnpm bench:test
pnpm bench:typecheck
pnpm docs:build
python3 -m unittest discover -s .github/scripts -p 'test_*.py'
uvx ruff@0.17.4 format --check
uvx ruff@0.17.4 check
```

CI also checks MSRV and operating systems. Run the relevant local checks, push authorized commits and read results on the current head. A green benchmark smoke job proves execution and equality, not performance.

## Code and naming

- Prefer the smallest complete operation. Avoid speculative adapters and parallel interpretations of arithmetic rules.
- Stable Rust. The MSRV lives in `Cargo.toml`; keep CI in sync. `fearless_simd` is mandatory, never a feature. `recovery` controls only arbitrary-precision research code.
- Forbid unsafe code. SIMD goes through safe fearless_simd APIs with runtime dispatch. Keep scalar-oracle, lane, tail and arithmetic-boundary tests.
- Use index, weight, term, moment, bucket and recovered mass consistently. Functions are verbs; `try_` may decline; units belong in names such as `_bits`, `_bytes`, `_ns`.
- Public items have a summary, contract, errors, memory and asserting doctests where applicable. Internal items have a summary; comments explain invariants and hazards.
- New source files carry an SPDX BSD-2-Clause comment. Never transplant implementation from third-party research code without license review.
- Tools own formatting: rustfmt, Biome and Ruff. Fix causes rather than adding broad suppressions.

## Documentation and measurement

Markdown paragraphs occupy one source line. Update the semantics document when exactness, overflow, canonicalization or memory contracts change. Engine changes update `docs/explain/` and `docs/design/`. Keep executable behavior and research plans separate in `ROADMAP.md`. Record settled choices under `docs/decisions/`, using the settling PR number after bootstrap.

Keep raw results and profiles out of Git. Benchmark plans and identities live in `bench/plans/` and `bench/catalog.toml`, not runner conditionals. Performance claims need identical inputs, exact output equality, alternating order, affinity/resource controls, fresh process repetitions, an A/A range and explicit limitations per the benchmark guide. Investigate a slowdown outside the controls. Do not claim the paper's time bound for a different hash or recovery schedule.

## Delivery

Conventional Commits, at most 64 characters. Sign commits with `git commit -s` using the configured identity and include an `Assisted-by:` or `Co-authored-by:` trailer. Do not publish a crate or release without explicit authorization. Public repository creation is separate from a package release.
