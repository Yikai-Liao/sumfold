<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Continuous integration

Every push to `main` and every pull request runs the same small acceptance surface. Rust tests cover Linux, macOS and Windows on stable, plus Linux at the manifest MSRV, Rust 1.89. Each cell checks the full workspace and doctests, then the direct-only library configuration. SIMD is present in every cell; it is a mandatory dependency. The Linux quality job checks rustfmt, Clippy with warnings denied, rustdoc with warnings denied, the source archive and Python policy-script tests/lints.

The tooling job installs the pinned pnpm toolchain, checks Biome, runs benchmark-plan tests and TypeScript checks, and builds VitePress with dead-link checking. It runs the smoke benchmark and uploads raw JSONL as a short-lived artifact. Those timing rows are diagnostics, not an enforced speed threshold.

PR metadata checks enforce Conventional Commit titles and author-matching DCO trailers. The workflow runs the base branch's policy script with read-only contents access and fetches head objects without executing PR code. Dependabot commits need valid author-matching sign-offs too; no unaudited bot exception exists.

Documentation deployment uses GitHub Pages artifacts with `pages:write` and OIDC only in the deploy job. Pull requests build but cannot deploy. The Pages base comes from the repository name; root Markdown documents render from their original files. Branch protection requires Rust matrix cells, quality, tooling and PR policy. Release preparation is manual, creates a source archive plus checksums, and does not publish a crate.

There is no unsafe project code, so a nightly Miri lane is not part of initial acceptance. If unsafe code is ever proposed, change that policy and add Miri and appropriate sanitizer coverage in the same reviewed change. There is no dedicated benchmark worker or continuous performance acceptance yet; see the [roadmap](../../ROADMAP.md).
