# AGENTS.md — symjump

Instructions for agentic tools in **this** repository.

Do not apply `symworx-tui` / RQA / `symworx-io` rules here. The human contributor owns every change.

## Project overview

Portable favorites + action launcher. Binary name: `sjmp`. Config: `~/.config/symjump/favorites.toml`.

Authoritative design: [docs/DESIGN.md](docs/DESIGN.md). Roadmap: [docs/ROADMAP.md](docs/ROADMAP.md).

## What to build (in order)

Shipped: CLI `list` / `jump` / `pin` / `unpin` / `kids`, closed TOML subset, bash hook + `jmp`, `M-p` / `M-P` / `M-Enter` in a normal shell and in vterm, `M-x` when readline receives it, numbered picks while the query is empty, `sjmp action` / `action add` / `action rm` / `exec` / `toolbox`.

Remaining:

1. Spawn backends (generic `cd`, kitty, foot, gnome/ptyxis, wezterm).
2. Native ratatui picker only if fzf is not enough.

Do **not** start a VTE/GPU terminal emulator. The bash hook runs when `$INSIDE_EMACS` is unset or `vterm`. Other Emacs shells still skip it.

## Bindings (do not change without asking)

| Chord | Action |
|---|---|
| `M-p` | places |
| `M-P` | kids of `$PWD` |
| `M-x` | actions when readline receives the key. vterm does not send it |
| `M-Enter` | actions in a bare terminal, tmux, and vterm. `\e\C-m` and `\e\C-j`. A tmux root binding can consume it before readline |
| letter | in that palette, query empty: run the action whose key is that letter. The filter matches the key and the label, not the kind word |
| `t` | that default, when key `t` is free: plain tmux for `$PWD` (`sjmp_tmux`). The row kind is `session`. A configured key `t` replaces it |
| `Esc` | on a later picker: back to the previous one. On the first picker: close |
| `M-<key>` | jump favorite with that `keys` letter |
| Enter | places and kids pickers: `cd` |
| `a` | those pickers, query empty: dashboard for `[ai] default` |
| `l` | those pickers, query empty: dashboard for `[ai] local`. Bare `ollama` lists models in the pane and does not load one until it is picked |

`pin --keys` must refuse `p`/`P`/`x` (sjmp), `b`/`f`/`d`/`y` (readline), and duplicate favorite keys. `M-Enter` is the actions chord that works in tmux and in vterm. Keep `M-x` as the readline alias. Do not move the vterm actions chord onto `M-x`.

If tmux is missing, `a` and `l` print that and stop. Do not download or install tmux. Do not bind `o` for this.

Never bind `C-g`, `C-c`, `C-x`, `C-z`, or Emacs's own `M-x`. vterm must keep `M-x`. The hook may bind readline `M-x` in the shell. It does not bind Emacs keys. `M-w` in an editing buffer stays copy. In vterm and in a bare terminal, `M-w` is the favorite jump when that letter is pinned.

Ask first before changing chords or input priority in the picker.

## Working style

- Incremental, visible steps. Docs-only is valid on `feature/cli-build`.
- Prefer fzf + a small Rust bin over a custom TUI.
- Prefer calling `zoxide query` over inventing frecency.
- New dependencies: ask first (especially GPU, async runtimes, extra TUI kits).
- Feature PRs target `worx`.

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
