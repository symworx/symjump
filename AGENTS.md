# AGENTS.md — symjump

Instructions for agentic tools in **this** repository.

This is **not** the SymWorx science workspace. Do not apply `symworx-tui` / RQA / `symworx-io` rules here. The human contributor owns every change.

## Project overview

Portable favorites + verb launcher. Binary name: `sjmp`. Config: `~/.config/symjump/favorites.toml`.

Sibling of [symworx/symworx](https://github.com/symworx/symworx); separate repo and release train.

Authoritative design: [docs/DESIGN.md](docs/DESIGN.md). Roadmap: [docs/ROADMAP.md](docs/ROADMAP.md).

## What to build (in order)

1. CLI: `list` / `jump` / `pin` / `kids` reading TOML — no TUI required.
2. bash hook + `cdw` wrapper; bind `M-p` / `M-P`.
3. fzf picker; numbered picks only while the query is empty.
4. `sjmp toolbox` + `M-x` verb palette **only if `INSIDE_EMACS` is unset**; vterm uses `M-p t`.
5. Spawn backends (generic `cd`, kitty, foot, gnome/ptyxis, wezterm).
6. Native ratatui picker only if fzf is not enough.

Do **not** start a VTE/GPU terminal emulator.

## Bindings (do not change without asking)

| Chord | Action |
|---|---|
| `M-p` | places |
| `M-P` | kids of `$PWD` |
| `M-x` | verbs, bare terminal only |
| `M-p t` | toolbox inside Emacs |
| `M-RET` | new window/tab at selection |

Never bind `C-g`, `C-c`, `C-x`, `C-z`, or global `M-x` / `M-w` under Emacs.

Ask first before changing chords or input priority in the picker.

## Working style

- Incremental, visible steps. Docs-only is valid on `feature/initial-build`.
- Prefer fzf + a small Rust bin over a custom TUI.
- Prefer calling `zoxide query` over inventing frecency.
- New dependencies: ask first (especially GPU, async runtimes, extra TUI kits).
- Feature PRs target `develop`, not `main`.

## When to ask vs implement

**Ask first:** keybindings, Emacs/vterm behavior, new verbs, new deps, org/license changes.

**Just do:** doc fixes that match DESIGN.md, typo/clarity, scaffolding that implements an already-specified CLI flag.

## File map

| Path | Role |
|---|---|
| `docs/DESIGN.md` | product + UX source of truth |
| `docs/ROADMAP.md` | implementation order |
| `CONTRIBUTING.md` | human contributors |
| `DEVELOPMENT.md` | build / branches |
| `README.md` | why + surface summary |

Rust crate layout does not exist yet. When it does: binary `sjmp`, library only if a backend needs to be shared.

## Relation to sysmgmt

`ntberry/sysmgmt` keeps git/docker/PATH/`tb` command aliases. This repo replaces `cdw` and destination-style `tb-python` entries — not `gs` or `tb` itself.

<!-- BEGIN symkit harness (do not edit this block) -->
Read [`AGENTS-SYMKIT.md`](AGENTS-SYMKIT.md) and follow it as additional always-on project rules from the installed symkit harness. Instructions in this `AGENTS.md` take precedence when they conflict.
<!-- END symkit harness -->
