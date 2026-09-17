# AGENTS.md — symjump

Instructions for agentic tools in **this** repository.

Do not apply `symworx-tui` / RQA / `symworx-io` rules here. The human contributor owns every change.

## Project overview

Portable favorites + action launcher. Binary name: `sjmp`. Config: `~/.config/symjump/favorites.toml`.

Authoritative design: [docs/DESIGN.md](docs/DESIGN.md). Roadmap: [docs/ROADMAP.md](docs/ROADMAP.md).

## What to build (in order)

Shipped: CLI `list` / `jump` / `pin` / `unpin` / `kids`, closed TOML subset, bash hook + `jmp`, `M-p` / `M-P` / `M-x` skipped when `INSIDE_EMACS`, numbered picks while the query is empty, `sjmp action` / `action add` / `action rm` / `exec` / `toolbox`.

Remaining:

1. Optional `M-RET` → `tmux new-window -c {path}`.
2. Spawn backends (generic `cd`, kitty, foot, gnome/ptyxis, wezterm).
3. Native ratatui picker only if fzf is not enough.

Do **not** start a VTE/GPU terminal emulator. Do **not** bind anything when `$INSIDE_EMACS` is set (no vterm `M-p t` in v1).

## Bindings (do not change without asking)

| Chord | Action |
|---|---|
| `M-p` | places |
| `M-P` | kids of `$PWD` |
| `M-x` | actions, bare terminal only |
| `M-<key>` | jump favorite with that `keys` letter |

`pin --keys` must refuse `p`/`P`/`x` (sjmp), `b`/`f`/`d`/`y` (readline), and duplicate favorite keys.
| `M-RET` | new window/tab at selection |

Never bind `C-g`, `C-c`, `C-x`, `C-z`, or global `M-x` / `M-w` under Emacs. The hook stays off when `$INSIDE_EMACS` is set, so terminal `M-w` does not steal Emacs `M-w`.

Ask first before changing chords or input priority in the picker.

## Working style

- Incremental, visible steps. Docs-only is valid on `feature/cli-build`.
- Prefer fzf + a small Rust bin over a custom TUI.
- Prefer calling `zoxide query` over inventing frecency.
- New dependencies: ask first (especially GPU, async runtimes, extra TUI kits).
- Feature PRs target `develop`, not `main`.

## When to ask vs implement

**Ask first:** keybindings, Emacs/vterm behavior, new actions, new deps, org/license changes.

**Just do:** doc fixes that match DESIGN.md, typo/clarity, scaffolding that implements an already-specified CLI flag.

## File map

| Path | Role |
|---|---|
| `docs/DESIGN.md` | product + UX source of truth |
| `docs/ROADMAP.md` | implementation order |
| `CONTRIBUTING.md` | human contributors |
| `DEVELOPMENT.md` | build / branches |
| `README.md` | why + surface summary |
| `examples/favorites.toml` | sample config |
| `crates/symjump-config` | TOML-subset types + `~` expand |
| `crates/symjump-core` | resolve / pin / kids / exec strings |
| `crates/sjmp` | CLI + bash hook |

## Relation to the user's shell kit

Git/docker/PATH and command aliases stay in the user's shell kit. This repo
replaces directory-jump helpers (`jmp`) and destination-style aliases — not
the rest of the shell.

<!-- BEGIN symkit harness (do not edit this block) -->
Read [`AGENTS-SYMKIT.md`](AGENTS-SYMKIT.md) and follow it as additional always-on project rules from the installed symkit harness. Instructions in this `AGENTS.md` take precedence when they conflict.
<!-- END symkit harness -->
