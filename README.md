# symjump

Cross-emulator directory favorites and jump list.

**Binary:** `sjmp`  
**Org:** [symworx](https://github.com/symworx) sibling repo — not part of the science workspace.  
**Status:** private, design-first (`feature/initial-build`).

A launcher for *places* (and later *verbs* like toolbox enter), not a terminal emulator and not a replacement for git/docker aliases.

## Why this project

I was screen-sharing with friends and colleagues, jumping between directories with aliases and shortcuts. I sent them my bashrc. It worked — after they rewrote paths and names for their machines. Not a huge tax, but enough to be annoying. I started thinking about a list that was not a private shell dialect.

After trying [Omarchy](https://omarchy.org), I had a clearer picture: zoxide + fzf + a terminal-agnostic hook travel; a pile of `cdw` / `tb-python` aliases do not. I wanted something more portable — pinned places and a few verbs (toolbox) that install the same way on Kitty, foot, or Emacs vterm.

This is not a bid to replace Omarchy’s `cd`/`z` stack. It sits *on top*: favorites you chose, plus destinations your bashrc should not have to encode.

## What it is

- Pinned favorites + optional frecency (zoxide or internal)
- `M-p` opens the places list; Tab / j-k / `1`–`9` / type-to-filter; Enter jumps
- Subdir listing when the target is **not** a git project
- `M-x` verb palette in a bare terminal (toolbox, later ssh/exec); **not bound inside Emacs**
- Works with Kitty, foot, GNOME Terminal/Ptyxis, WezTerm, and Emacs vterm via a **sidecar + shell hook**, not an in-emulator plugin

## Bindings (Meta, not Control)

| Chord | Where | Action |
|---|---|---|
| `M-p` | everywhere | favorites / places |
| `M-P` | everywhere | kids of `$PWD` |
| `M-x` | Kitty / foot / GNOME / WezTerm only | verb palette (`t` = toolbox) |
| `M-p t` | Emacs vterm | toolbox list (`M-x` stays Emacs) |
| `M-RET` | inside picker | new terminal window/tab at selection |

See [docs/DESIGN.md](docs/DESIGN.md) for the full table and `[[verbs.toolbox]]` config.

## What it is not

- Not a GPU terminal (see Kitty / foot / Alacritty / WezTerm)
- Not in the `symworx` crate workspace (biosignal / load / dynamics)
- Not a Bitterbeta or cSYMd product
- Does not replace `gs` / `gacp` / `d` / `tb` / PATH setup in `ntberry/sysmgmt`
- Replaces `cdw` and commented `tb-python`-style destination aliases only

## Names

| Surface | Name |
|---|---|
| Repo / crate | `symjump` |
| Binary | `sjmp` |
| Shell alias | keep `cdw` as a wrapper |

Avoided: SymTerm (reads as an emulator), SymKey (crypto), binary `sym` (taken).

## Branches

| Branch | Role |
|---|---|
| `main` | stable / empty-ish product line |
| `develop` | integration |
| `stage` | pre-release |
| `feature/initial-build` | this design + first implementation |

Feature PRs target **`develop`**, not `main`. Path: `develop` → `stage` → `main` (same idea as [symworx](https://github.com/symworx/symworx)).

## Docs

- [docs/DESIGN.md](docs/DESIGN.md) — architecture, Meta bindings, toolbox verbs, config, sysmgmt split
- [docs/ROADMAP.md](docs/ROADMAP.md) — first implementation steps
- [CONTRIBUTING.md](CONTRIBUTING.md) — how to contribute
- [DEVELOPMENT.md](DEVELOPMENT.md) — build / branch / release notes
- [AGENTS.md](AGENTS.md) — guidelines for agentic tools

## License

TBD (likely Apache-2.0 to match the personal symworx stack).
