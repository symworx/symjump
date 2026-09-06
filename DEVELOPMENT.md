# Development

How to work in **symjump**. For agent rules see [AGENTS.md](AGENTS.md). For product rules see [docs/DESIGN.md](docs/DESIGN.md).

This repo is a sibling of [symworx/symworx](https://github.com/symworx/symworx), not a workspace member. Do not add it to that `Cargo.toml`.

## Prerequisites

- Rust stable (MSRV TBD; expect 1.85+ / edition 2024 to match SymWorx when the crate lands)
- `fzf` for the first picker
- Optional: `zoxide` for the frequent-dirs source
- `toolbox` only if you use Fedora toolbox verbs

## Branches

| Branch | Role |
|---|---|
| `main` | stable |
| `develop` | integration; **PR target** |
| `stage` | pre-release FF from develop |
| `feature/*` | work |

Current design work: `feature/initial-build`.

```bash
git clone git@github.com:symworx/symjump.git
cd symjump
git checkout feature/initial-build   # until merged to develop
```

## Commands (once the crate exists)

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --bin sjmp -- list
```

Until then, documentation PRs are the expected contribution.

## Release path

Same shape as SymWorx, lighter until publish is real:

1. Land features on `develop`.
2. Fast-forward `develop` → `stage` when you want a promotion point.
3. `release/vX.Y.Z` from `stage`, changelog, PR to `main`.
4. Merge, **manually** tag `vX.Y.Z`.

No crates.io job until LICENSE + a real crate exist.

## Code style

- `cargo fmt` before commit.
- Focused PRs.
- Binding or backend changes must update DESIGN.md and README.

## Related trees

- SymWorx contributing / agents (science stack): https://github.com/symworx/symworx
- Personal shell kit: `ntberry/sysmgmt` (`bash/bashrc.d/`)
