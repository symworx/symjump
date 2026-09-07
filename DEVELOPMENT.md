# Development

How to work in **symjump**. For agent rules see [AGENTS.md](AGENTS.md). For product rules see [docs/DESIGN.md](docs/DESIGN.md).

This repo is a sibling of [symworx/symworx](https://github.com/symworx/symworx), not a workspace member. Do not add it to that `Cargo.toml`.

## Install name

| Surface | Name |
|---|---|
| crates.io / `cargo install` | **`symjump`** (the workspace; binary is `sjmp`) |
| Binary on PATH | `sjmp` |
| Config | `~/.config/symjump/favorites.toml` |

Not `symworx.jump` — that is npm/PyPI namespacing. Cargo cannot use a dotted package name like that, and it would look like a workspace member of the science stack.

```bash
cargo install --path crates/sjmp
eval "$(sjmp init bash)"   # skip automatically when INSIDE_EMACS
```

## Crates

| Crate | Role |
|---|---|
| `symjump-config` | TOML types + expand `~` |
| `symjump-core` | resolve / pin / kids / exec strings |
| `sjmp` | CLI + bash hook |

## Prerequisites

- **Rust 1.86+**, edition **2024** (`rust-version` in `Cargo.toml`)
- `fzf` for the picker in the bash hook (numbered picks use `transform` / `rebind`; older fzf still jumps on `1`–`9` but digits will not filter)
- Optional: `zoxide`, `toolbox`, `grok`

## Branches

Work on `feature/cli-build`. PR to `develop`.

```bash
git clone git@github.com:symworx/symjump.git
cd symjump
git checkout feature/cli-build
cargo test --workspace
cargo run -p sjmp -- list
```

## Release path

`develop` → `stage` → `release/vX.Y.Z` → `main` → manual tag.

## Related trees

- SymWorx: https://github.com/symworx/symworx
