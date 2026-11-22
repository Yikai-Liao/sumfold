<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Release process

The repository is public; no crate or versioned release has been published. Workspace packages use `0.0.0` and `publish = false` until release acceptance is reviewed.

1. Complete the API, numeric-domain and research-boundary review. Update the roadmap and documentation, and confirm that every claimed property is implemented.
2. Run all contributor checks and inspect the current commit's platform/MSRV/tooling results. If claiming performance, attach controlled measurements with A/A ranges and exact input identity.
3. Choose versions, update the library and CLI manifests, add the CLI's registry dependency version and review `cargo package --list`. Enable publication only in a reviewed release change. Keep the benchmark crate private.
4. Obtain explicit maintainer authorization for crates.io publication or a versioned release. The manual source-preparation workflow is not publication authorization.
5. Publish the library before its CLI package, verify a fresh installation and build release notes from merged user-visible PR descriptions. Attach checksums, provenance and measurement artifacts, then tag the reviewed commit.

The `Prepare source` workflow accepts only a full commit hash already reachable from `main`, runs library checks and creates a Git archive and SHA-256 file. It uploads an artifact, never creates a GitHub release and never receives a registry token. A binary-distribution workflow will be added when the CLI has a stable version and release target policy.
