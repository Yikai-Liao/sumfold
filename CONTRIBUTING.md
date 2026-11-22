<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Contributing to Sumfold

Useful contributions include exactness regressions, reproducible workload descriptions, independent algorithm implementations and controlled measurements. Discuss interface changes and new engine strategies in an issue before implementing them.

## Development

Use stable Rust, at least the `rust-version` in the root manifest. SIMD is a required dependency and is exercised in every build. Node 24 runs the TypeScript tooling directly; pnpm is pinned by `packageManager`.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --workspace --doc --locked
cargo test -p sumfold --no-default-features --locked
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps
pnpm install --frozen-lockfile
pnpm lint
pnpm bench:test
pnpm bench:typecheck
pnpm docs:build
python3 -m unittest discover -s .github/scripts -p 'test_*.py'
uvx ruff@0.17.4 format --check
uvx ruff@0.17.4 check
```

`--no-default-features` checks the direct-only library; it does not disable SIMD. The CLI and benchmark enable recovery themselves. No project crate contains unsafe code. Miri becomes required if that policy changes; platform and MSRV builds already run in CI.

## Contracts and tests

[Semantics](conformance/semantics.md) defines results and failures. Test observable behavior with independent oracles, arithmetic boundaries and adversarial collisions. Keep randomized tests reproducible. Do not copy implementation thresholds into tests as expected behavior. Never use floating-point arithmetic to certify exact singleton recovery.

## Performance

Follow the [measurement guide](docs/development/benchmarking.md). Smoke CI checks harness execution and output equality. Changes to hashing, dense convolution, schedules or routing require measurements before performance claims or default selection changes.

## Documentation

Public Rust API documentation contains a one-line summary, a complete contract and applicable `# Errors`, `# Memory`, `# Panics` and asserting `# Examples` sections. Comments explain why. Markdown paragraphs occupy one line. Update the contract, explanation, design and roadmap alongside code. Research sources must state the model assumptions needed for their guarantees.

## Commit messages and sign-off

Use `type(scope)!: summary`, at most 64 characters, with an imperative English summary and no final period. Types are `feat`, `fix`, `perf`, `refactor`, `docs`, `test`, `ci`, `build`, `chore` and `revert`; scopes are `core`, `simd`, `recovery`, `cli`, `bench`, `site` and `deps`. The scope and breaking marker are optional. Pull requests are squash-merged, so CI checks the title.

Every commit must carry a `Signed-off-by: Name <email>` matching its author, certifying the [Developer Certificate of Origin 1.1](https://developercertificate.org/). Run `git commit -s`. AI-assisted contributions must disclose the tool and work in the PR and include an `Assisted-by:` or `Co-authored-by:` commit trailer. The author remains responsible for understanding and checking every line.

## License and conduct

Contributions use [BSD-2-Clause](https://github.com/Yikai-Liao/sumfold/blob/main/LICENSE). New source files carry the corresponding SPDX comment. Be respectful, focus criticism on the work, and avoid harassment, discrimination and sharing private information. Maintainers may remove abusive contributions.
