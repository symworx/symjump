# symjump design

## Problem

Directory hops today live as one-off shell helpers, chiefly:

```bash
cdw() { cd "$HOME/src/$1" ; }
```

plus destination aliases for toolbox containers.

That does not scale across machines or emulators, and it is the wrong layer
for a growing list of labs/repos.

## Value / scope lock

Emacs already switches projects (`project.el`, consult-dir, `C-x p`). Do **not**
compete with that. **All** sjmp chords stay off when `$INSIDE_EMACS` is set,
including `M-x`. There is no vterm hook in v1 (`M-p t` is not bound).

The gap is the *other* session: a real terminal (often **tmux**), where Grok
Build / Codex / Claude start. Those tools load `AGENTS.md` / `CLAUDE.md` from
**startup cwd**.

What this project is for:

1. A portable `favorites.toml`.
2. `cd` in Foot / Kitty / tmux — `M-p` when not in Emacs.
3. **`M-x` action palette** in that same terminal: toolbox, grok-build, exec.

What this project is not: a second Projectile, a zoxide replacement, an
in-agent `/cd`, a kids file manager in v1, a ratatui app until fzf fails.

tmux: `cd` in the current pane first. Optional: `tmux new-window -c {path}`.

## Placement

| Home | Why |
|---|---|
| `github.com/symworx/symjump` | Sibling of the science stack |
| Not `symworx/symworx` workspace | Not biosignal/dynamics |
| User's shell kit | `eval "$(sjmp init bash)"` — git/docker/PATH stay there |

## Architecture

```
favorites.toml + actions (toolbox, agent exec)
        ↓
sjmp (CLI + fzf)
        ↓
  cd | tmux | toolbox | exec {cmd} | kitty/foot/…
        ↓
shell hook — skip entire hook when INSIDE_EMACS
```

## Bindings: Meta, not Control

Do not take `C-g`, `C-c`, `C-x`, `C-z`. tmux owns its prefix.
Document `alt-sends-escape`.

| Chord | Where | Action |
|---|---|---|
| `M-p` | terminal / tmux only | places |
| `M-P` | same | kids of `$PWD` |
| **`M-x`** | same, **never in Emacs** | action palette |
| `M-<key>` | same | jump the favorite whose `keys` is that letter (`M-w`, `M-j`, …) |

`sjmp pin --keys` refuses letters that are already taken:

- sjmp chords: `p` (`M-p` places), `P` (`M-P` kids), `x` (`M-x` actions)
- readline emacs-mode: `b` / `f` (word motion), `d` (kill-word), `y` (yank-pop)
- any `keys` already used by another favorite

Also leave Control alone (`C-g`, `C-c`, `C-x`, `C-z`). Crowded but still allowed: `M-u` / `M-l` / `M-c` (case), `M-.` (last arg; not a letter so never a favorite Meta bind).
| `M-RET` | inside a picker | new tmux window / tab at path |

### After `M-x` (action palette)

Type or hit the letter. Then pick a place if the action needs one.

| Key | Action | Effect |
|---|---|---|
| `t` | toolbox | list `[[actions.toolbox]]` → `toolbox enter <name>` |
| `g` | grok / build | pick favorite → `cd {path} && grok` |
| `e` | exec | configured `{cmd}` with `{path}` |
| `p` | places | same as `M-p` |
| `s` | ssh | later |

`M-x` then `g` is the daily Grok Build path: action first, place second.
`M-p` then `e`/`t` remains valid *inside* the places picker.

```toml
[keys]
leader = "M-p"
kids   = "M-P"
actions = "M-x"   # bind only if INSIDE_EMACS is unset
```

```toml
[[actions.agent]]
label = "grok-build"
keys = "g"
cmd = "grok"

[[actions.agent]]
label = "codex"
keys = "c"
cmd = "codex -C {path}"

[[actions.toolbox]]
label = "python"
name = "dev-python"
keys = "p"
```

A `tb` → `toolbox` alias, if any, stays in the user's shell kit. This repo
does not ship it.

## Git vs kids

`.git` → project root. `sjmp kids` lists immediate subdirs (skipping
`target`, `node_modules`, `.git`, venv). `M-P` runs that list through fzf.

## Config sketch

`~/.config/symjump/favorites.toml` — `root`, `[keys]`, `[[favorites]]`,
`[[actions.toolbox]]`, `[[actions.agent]]` as above. Use zoxide if present.

See [examples/favorites.toml](../examples/favorites.toml).

## Shell-kit split

**Move here:** directory jump helpers and destination-style aliases.

**Stay in the user's shell kit:** git/docker/PATH/readline, and command
aliases such as a `tb` wrapper around `toolbox`.

```bash
jmp() { sjmp jump "$@"; }
```

The bash hook already defines `jmp` that way (empty args → fzf).

## Status

Shipped on `feature/cli-build`:

1. `sjmp list` / `jump` / `pin` / `unpin` / `kids`
2. Closed TOML-subset parser (no serde)
3. bash hook + `jmp`; skip entire hook when `INSIDE_EMACS`; first `sjmp` on the default path writes `~/.config/symjump/favorites.toml` if missing
4. `M-p` places, `M-P` kids, `M-x` from `sjmp action list`
5. Numbered picks `1`–`9` only while the fzf query is empty
6. `sjmp action` / `action add` / `action rm` / `exec` / `toolbox`

Next: optional `tmux new-window -c {path}` on `M-RET`.

Later: spawn backends (kitty / foot / gnome / wezterm), zoxide, ratatui
if fzf fails, LICENSE file.

Do not start a VTE/GPU emulator.
