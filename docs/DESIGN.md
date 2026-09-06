# symjump design

## Problem

Directory hops today live as one-off shell helpers, chiefly:

```bash
cdw() { cd "$HOME/worx/$1" ; }
```

in `ntberry/sysmgmt` (`bash/bashrc.d/environment.sh`), plus commented toolbox destinations.

That does not scale across machines or emulators, and it is the wrong layer for a growing list of labs/repos.

## Value / scope lock

Emacs already switches projects (`project.el`, consult-dir, `C-x p`). Do **not**
compete with that. **All** sjmp chords stay off when `$INSIDE_EMACS` is set,
including `M-x`.

The gap is the *other* session: a real terminal (often **tmux**), where Grok
Build / Codex / Claude start. Those tools load `AGENTS.md` / `CLAUDE.md` from
**startup cwd**.

What this project is for:

1. A portable `favorites.toml`.
2. `cd` in Foot / Kitty / tmux — `M-p` when not in Emacs.
3. **`M-x` verb palette** in that same terminal: toolbox, grok-build, exec.

What this project is not: a second Projectile, a zoxide replacement, an
in-agent `/cd`, a kids file manager in v1, a ratatui app until fzf fails.

tmux: `cd` in the current pane first. Optional: `tmux new-window -c {path}`.

## Placement

| Home | Why |
|---|---|
| `github.com/symworx/symjump` | Sibling of the science stack |
| Not `symworx/symworx` workspace | Not biosignal/dynamics |
| Not Bitterbeta / PalEm / cSYMd | Personal OSS tool |
| sysmgmt | `eval "$(sjmp init bash)"` |

## Architecture

```
favorites.toml + verbs (toolbox, agent exec)
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
| `M-P` | same | kids of `$PWD` (can defer) |
| **`M-x`** | same, **never in Emacs** | verb palette |
| `M-p t` / `M-p g` | only if you insist in vterm | toolbox / grok; default is *no hook* in Emacs |
| `M-RET` | inside a picker | new tmux window / tab at path |

### After `M-x` (verb palette)

Type or hit the letter. Then pick a place if the verb needs one.

| Key | Verb | Effect |
|---|---|---|
| `t` | toolbox | list `[[verbs.toolbox]]` → `toolbox enter <name>` |
| `g` | grok / build | pick favorite → `cd {path} && grok` |
| `e` | exec | configured `{cmd}` with `{path}` |
| `p` | places | same as `M-p` |
| `s` | ssh | later |

`M-x` then `g` is the daily Grok Build path: verb first, place second.
`M-p` then `e`/`t` remains valid *inside* the places picker.

```toml
[keys]
leader = "M-p"
kids   = "M-P"
verbs  = "M-x"    # bind only if INSIDE_EMACS is unset
```

```toml
[[verbs.agent]]
label = "grok-build"
keys = "g"
cmd = "grok"

[[verbs.agent]]
label = "codex"
keys = "c"
cmd = "codex -C {path}"

[[verbs.toolbox]]
label = "python"
name = "dev-python"
keys = "p"
```

Keep `alias tb='toolbox'` in sysmgmt.

## Git vs kids

`.git` → project root. Kids listing is optional v1.

## Config sketch

`~/.config/symjump/favorites.toml` — `root`, `[keys]`, `[[favorites]]`,
`[[verbs.toolbox]]`, `[[verbs.agent]]` as above. Use zoxide if present.

## sysmgmt split

Move: `cdw`, destination aliases. Keep: `gs`, `tb`, `d`, PATH, readline.

```bash
cdw() { sjmp jump "$@"; }
```

## First code path

1. `sjmp list` / `jump` / `pin`
2. fzf + `M-p` (skip Emacs)
3. **`M-x` verb palette** + `sjmp verb` / `sjmp exec` (Grok Build first)
4. `sjmp toolbox`
5. tmux `-c` if pane `cd` is not enough
6. ratatui last

Do not start a VTE/GPU emulator.
