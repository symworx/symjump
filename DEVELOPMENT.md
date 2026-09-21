# Development

How to work in **symjump**. For agent rules see [AGENTS.md](AGENTS.md). For product rules see [docs/DESIGN.md](docs/DESIGN.md).

## Install name

| Surface | Name |
|---|---|
| crates.io | `cargo install sjmp` |
| From a checkout | `cargo install --path crates/sjmp` |
| Binary on PATH | `sjmp` |
| Config | `~/.config/symjump/favorites.toml` |

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

**SymWorx org standard** (GitHub Flow). Branch from **`worx`**. PR to `worx`.

```bash
git clone git@github.com:symworx/symjump.git
cd symjump
git checkout worx
cargo test --workspace
cargo run -p sjmp -- list
```

## Releasing

```text
feature/*  ──PR──►  worx  ──tag──►  vX.Y.Z
```

`CHANGELOG.md` must have a `## [X.Y.Z]` section. Tag `vX.Y.Z` on `worx` (must
match `[workspace.package] version`). Then publish to crates.io, libraries
first:

```bash
cargo publish -p symjump-config
cargo publish -p symjump-core
cargo publish -p sjmp
```

GitHub Environment `crates-io` holds `CARGO_REGISTRY_TOKEN` for a later
tag-triggered workflow. Until then, publish by hand after the tag.

From a checkout without crates.io:

```bash
toolbox run -c dev-rust cargo install --path crates/sjmp --root "$HOME/.local"
```

