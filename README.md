# Brokk's PHP Grammar for Tree-sitter

This is the Brokk-owned and independently maintained fork of
[`tree-sitter/tree-sitter-php`](https://github.com/tree-sitter/tree-sitter-php).
`master` is Brokk's maintained release branch, following the convention of the
other Brokk grammar forks. Brokk maintains this fork for its code-intelligence
tooling and publishes the Rust package as
[`brokk-tree-sitter-php`](https://crates.io/crates/brokk-tree-sitter-php).

## Installation

```toml
[dependencies]
brokk-tree-sitter-php = "=0.24.4"
```

## Changes from upstream

`brokk-tree-sitter-php` 0.24.4 is based on upstream v0.24.2
(`5b5627faaa290d89eb3d01b9bf47c3bb9e797dea`).
Backports Apollo Nicolson's upstream commit
`9700857ed4695afef44f9853a5e8bc7e1edb407e`, which permits asymmetric visibility
on constructor-promoted properties. Both PHP dialects are regenerated using
Tree-sitter CLI 0.25.8, matching the baseline generated parser version.
[Backports Jan Mohr's upstream PR #306](https://github.com/tree-sitter/tree-sitter-php/pull/306), commit
`eddc18246a1226f17a08f61f0640568cb4f50e15`, which supports PHP 8.5 clone
arguments and preserves the legacy clone AST for the one-argument form. The
qualified upstream PR head is `5853b9157e1d043ef00d96529bd1e35a5ad15cec`;
its generated parser output was regenerated locally with the pinned CLI.
Upstream MIT licensing and author attribution are preserved.

Only the Brokk-namespaced Rust crate is published by this fork. Other language
bindings retain their upstream names and must not be published by this fork.
Rust native symbols are prefixed so the crate can coexist with upstream PHP.

## Validate

```sh
npm ci --ignore-scripts
npm rebuild tree-sitter-cli
make generate TS="$PWD/node_modules/.bin/tree-sitter"
git diff --exit-code -- php/src php_only/src
node_modules/.bin/tree-sitter test
npm run lint
cargo fmt --check
cargo test --locked
cargo package --locked
cargo publish --dry-run --locked
```

## Releases

Prepare, validate, and merge release changes on `master`; create the release
tag from that maintained line. Tags use `v` followed by the Cargo package
version and must remain immutable.
Package from the clean tagged commit, verify the archive's
`.cargo_vcs_info.json`, and record its SHA-256 before publication.
The initial 0.24.3 publication used authorized local Cargo credentials. The
Rust-only
`publish.yml` workflow is gated on `CARGO_TRUSTED_PUBLISHING=true`; enable it
only after a trusted publisher is configured for organization `BrokkAi`,
repository `tree-sitter-php`, workflow `publish.yml`, and environment `release`.
The publishing workflow verifies that the tagged commit is part of `master`.
Verify the registry release before updating dependent projects.

The existing `v0.24.3` tag and crates.io release remain at source
`707c476eacceb35f7b7ac5a2929f20fa6430285f`. The released implementation is now
reconciled onto `master` without rewriting upstream history. The former
`brokk-0.24` branch is a historical reference; future releases use `master`.

## Upstream README

# tree-sitter-php

[![CI][ci]](https://github.com/tree-sitter/tree-sitter-php/actions/workflows/ci.yml)
[![discord][discord]](https://discord.gg/w7nTvsVJhm)
[![matrix][matrix]](https://matrix.to/#/#tree-sitter-chat:matrix.org)
[![crates][crates]](https://crates.io/crates/tree-sitter-php)
[![npm][npm]](https://www.npmjs.com/package/tree-sitter-php)
[![pypi][pypi]](https://pypi.org/project/tree-sitter-php)

PHP grammar for [tree-sitter](https://github.com/tree-sitter/tree-sitter).

[ci]: https://img.shields.io/github/actions/workflow/status/tree-sitter/tree-sitter-php/ci.yml?logo=github&label=CI
[discord]: https://img.shields.io/discord/1063097320771698699?logo=discord&label=discord
[matrix]: https://img.shields.io/matrix/tree-sitter-chat%3Amatrix.org?logo=matrix&label=matrix
[npm]: https://img.shields.io/npm/v/tree-sitter-php?logo=npm
[crates]: https://img.shields.io/crates/v/tree-sitter-php?logo=rust
[pypi]: https://img.shields.io/pypi/v/tree-sitter-php?logo=pypi&logoColor=ffd242
