# symjump

Cross-emulator directory favorites and jump list.

**Binary:** `sjmp`  
**Org:** [symworx](https://github.com/symworx) sibling repo — not part of the science workspace.  
**Status:** private, design-first (`feature/initial-build`).

A launcher for *places*, not a terminal emulator and not a replacement for git/docker aliases.

## What it is

- Pinned favorites + optional frecency (zoxide or internal)
- One shortcut opens a list; Tab / j-k / `1`–`9` / type-to-filter; Enter jumps
- Subdir listing when the target is **not** a git project
- Works with Kitty, foot, GNOME Terminal/Ptyxis, WezTerm, and Emacs vterm via a **sidecar + shell hook**, not an in-emulator plugin

## What it is not

- Not a GPU terminal (see Kitty / foot / Alacritty / WezTerm)
- Not in the `symworx` crate workspace (biosignal / load / dynamics)
- Not a Bitterbeta or cSYMd product
- Does not replace `gs` / `gacp` / `d` / PATH setup in `ntberry/sysmgmt`

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

## Docs on this branch

- [docs/DESIGN.md](docs/DESIGN.md) — architecture, UX, config shape, sysmgmt split
- [docs/ROADMAP.md](docs/ROADMAP.md) — first implementation steps

## License

TBD (likely Apache-2.0 to match the personal symworx stack).
