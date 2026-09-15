# symjump

Cross-emulator directory favorites and jump list.

**Binary:** `sjmp`  
**Org:** [symworx](https://github.com/symworx) sibling 

A launcher for *places* and *actions* (toolbox enter, agent exec), not a
terminal emulator and not a replacement for git/docker aliases.

## Why this project

I was screen-sharing with friends and colleagues, jumping between directories with aliases and shortcuts. I sent them my bashrc. It worked — after they rewrote paths and names for their machines. Not a huge tax, but enough to be annoying. I started thinking about a list that was not a private shell dialect.

After trying [Omarchy](https://omarchy.org), I had a clearer picture: zoxide + fzf + a terminal-agnostic hook travel; a pile of directory aliases do not. I wanted something more portable — pinned places and a few actions (toolbox) that install the same way on Kitty, foot, or a tmux pane.

It sits *on top*: favorites you chose, plus destinations your bashrc should not have to encode.

## What it is (shipped)

- Pinned favorites in `~/.config/symjump/favorites.toml`
- CLI: `sjmp list` / `jump` / `pin` / `unpin` / `kids` / `action` / `action rm` / `exec` / `toolbox`
- Help: `sjmp --help`, `sjmp pin --help`, `sjmp action --help`
- `eval "$(sjmp init bash)"` defines `jmp`, binds `M-p` (places), `M-P` (kids), and `M-x` (actions from config), and **skips the whole hook** when `$INSIDE_EMACS` is set
- fzf picker: numbered picks `1`–`9` only while the query is empty

Not wired yet: `M-RET` new window, zoxide, spawn backends. See [docs/ROADMAP.md](docs/ROADMAP.md).

## Install

```bash
cargo install --path crates/sjmp
sjmp                       # first run writes ~/.config/symjump/favorites.toml
eval "$(sjmp init bash)"   # shell hook; skip when INSIDE_EMACS
sjmp pin --current --label src --keys r
sjmp action add agent --cmd grok --keys g --label grok-build
sjmp action add toolbox --name dev-python --keys p --label python
sjmp list
```

Remove an action by label or key (`agent` / `toolbox` if both match):

```bash
sjmp action rm grok-build
sjmp action rm agent python
```

`cargo install` only puts `sjmp` on `PATH` — Cargo has no post-install hook.
The binary creates the config dir/file the first time you run it (default
path only; `--config` is left alone). It will not overwrite an existing file.
`sjmp init` prints the path; `sjmp init bash` prints the hook.

A fuller sample is [examples/favorites.toml](examples/favorites.toml).

If `sjmp` is missing from `PATH`, the hook does nothing.

## Bindings (Meta, not Control)

Intended chords ([docs/DESIGN.md](docs/DESIGN.md)). Bound outside Emacs only.

| Chord | Where | Action |
|---|---|---|
| `M-p` | terminal / tmux only | favorites / places |
| `M-P` | same | kids of `$PWD` |
| `M-x` | same, never in Emacs | action palette (`sjmp action list`) |
| `M-<key>` | same | jump favorite `keys` (`M-w`, `M-j`, …) |

`pin --keys` rejects `p` / `P` / `x` (sjmp chords), `b` / `f` / `d` / `y` (readline), and letters already used by another favorite.
| `M-RET` | inside a picker | new tmux window/tab (not bound yet) |

See DESIGN for the full table and `[[actions.toolbox]]` / `[[actions.agent]]` config.

## What it is not

- Not a GPU terminal (see Kitty / foot / Alacritty / WezTerm)
- Not a member of the `symworx` crate workspace (biosignal / load / dynamics)
- Not a second Projectile — Emacs already switches projects; this hook stays off there
- Does not replace git/docker/PATH setup in the user's shell kit
- Replaces directory-jump helpers and destination-style aliases only

## Names

| Surface | Name |
|---|---|
| Repo / crate | `symjump` |
| Binary | `sjmp` |
| Shell function | `jmp` |

Avoided: SymTerm (reads as an emulator), SymKey (crypto), binary `sym` (taken).

## Branches

| Branch | Role |
|---|---|
| `main` | stable / empty-ish product line |
| `develop` | integration |
| `stage` | pre-release |
| `feature/cli-build` | CLI spike + this docs pass |

Feature PRs target **`develop`**, not `main`. Path: `develop` → `stage` → `main` (same idea as [symworx](https://github.com/symworx/symworx)).

## Docs

- [docs/DESIGN.md](docs/DESIGN.md) — architecture, Meta bindings, toolbox actions, config, shell-kit split
- [docs/ROADMAP.md](docs/ROADMAP.md) — shipped vs next
- [CONTRIBUTING.md](CONTRIBUTING.md) — how to contribute
- [DEVELOPMENT.md](DEVELOPMENT.md) — build / branch / release notes
- [AGENTS.md](AGENTS.md) — guidelines for agentic tools

## License

Apache-2.0 intended (matches `Cargo.toml`). LICENSE file at public release.
