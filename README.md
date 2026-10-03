# Brokk PHP grammar release candidate

Unpublished candidate `brokk-tree-sitter-php` 0.24.3 for the proposed
`BrokkAi/tree-sitter-php` fork. Repository creation, crate ownership verification,
and publication are pending the authorized local release session.

Based on upstream v0.24.2 (`5b5627faaa290d89eb3d01b9bf47c3bb9e797dea`).
Backports Apollo Nicolson's upstream commit
`9700857ed4695afef44f9853a5e8bc7e1edb407e`, which permits asymmetric visibility
on constructor-promoted properties. Both PHP dialects are regenerated using
Tree-sitter CLI 0.25.8, matching the baseline generated parser version.
Upstream MIT licensing and author attribution are preserved.

Only the Brokk-namespaced Rust crate is intended for publication. Other language
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

## Release handoff

Do not publish until Brokk organization access and the crates.io account and
namespace are verified. Use the existing TypeScript fork's bootstrap convention:
an already authorized short-lived token for first publication; immutable
`v0.24.3` tag; clean tagged package; verify `.cargo_vcs_info.json` and archive
SHA-256; publish only `brokk-tree-sitter-php` 0.24.3. No token or trusted publisher
has been created or configured here. New persistent publisher configuration
requires separate confirmation. Verify the registry before updating Bifrost to
`tree-sitter-php = { package = "brokk-tree-sitter-php", version = "=0.24.3" }`.

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
