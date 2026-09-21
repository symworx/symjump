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
# Host has no rustup. Build/install from the rust toolbox onto ~/.local/bin:
toolbox run -c dev-rust cargo install --path crates/sjmp --root "$HOME/.local"
sjmp                       # first run writes ~/.config/symjump/favorites.toml
eval "$(sjmp init bash)"   # skip automatically when INSIDE_EMACS
```

## Crates

| Crate | Role |
|---|---|
| `symjump-config` | TOML types + expand `~` |
| `symjump-core` | resolve / pin / unpin / kids / exec strings |
| `sjmp` | CLI + bash hook |

## Prerequisites

- **Rust 1.86+**, edition **2024** (`rust-version` in `Cargo.toml`)
- `fzf` for the picker in the bash hook (numbered picks use `transform` / `rebind`; older fzf still jumps on `1`–`9` but digits will not filter)
- Optional: `zoxide`, `toolbox`, `grok`

## Branches

GitHub Flow. Branch from **`worx`**. PR to `worx`.

Until GitHub finishes renaming `develop` → `worx`, the default is still named `develop`.

```bash
git clone git@github.com:symworx/symjump.git
cd symjump
git checkout worx   # develop until GitHub renames the default
cargo test --workspace
cargo run -p sjmp -- list
```

## Release path

```text
feature/*  ──PR──►  worx  ──tag──►  vX.Y.Z
```

## Related trees

- SymWorx: https://github.com/symworx/symworx
