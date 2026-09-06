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
compete with that. Bindings stay off when `$INSIDE_EMACS` is set.

The gap is the *other* session: a real terminal (often **tmux**), where Grok
Build / Codex / Claude start. Those tools load `AGENTS.md` / `CLAUDE.md` from
**startup cwd**. Jumping tmux windows does not change that cwd; launching the
agent from `$HOME` does the wrong tree.

What this project is for:

1. A portable `favorites.toml` (shareable; not a private bashrc).
2. `cd` (or print path) in Foot / Kitty / tmux — `M-p` when not in Emacs.
3. Verbs on a place: `toolbox enter`, and `exec` an agent in that directory
   (`grok` / Grok Build, `codex -C {path}`, `claude`).

What this project is not:

- A second Projectile.
- A replacement for zoxide frecency or fzf.
- An in-agent `/cd` (Claude/Codex already have that).
- A kids-of-non-git file manager in v1.
- A native ratatui app until fzf is clearly insufficient.

tmux: prefer `cd` in the **current pane**. Optional later: `tmux new-window -c
{path}` or named sessions per favorite. Do not invent a session manager.

## Placement

| Home | Why |
|---|---|
| `github.com/symworx/symjump` | Same personal OSS brand as symworx; separate release train |
| Not `symworx/symworx` workspace | Jumper is not biosignal/dynamics |
| Not Bitterbeta / PalEm | Not a consulting deliverable |
| Not cSYMd | Not lab/grant/course infra |
| sysmgmt stays the installer | `eval "$(sjmp init bash)"` next to `aliases.sh` |

## Architecture

Terminals do not share a plugin ABI.

```
favorites.toml + verbs (toolbox, agent exec)
        ↓
sjmp (CLI + fzf picker)
        ↓
  backends: generic cd | tmux pane/window | toolbox | exec {cmd} | kitty/foot/…
        ↓
shell hook (bash) — skip when INSIDE_EMACS
```

Detect context via `$TMUX`, `$KITTY_WINDOW_ID`, `$WEZTERM_PANE`, `$TERM`, `$INSIDE_EMACS`.

## Bindings: Meta, not Control

Do not take `C-g`, `C-c`, `C-x`, `C-z`. tmux already owns a prefix.

Use **Meta** (`M-p`). Document `alt-sends-escape`. No `M-x` under Emacs.

### v1 chords

| Key | Action |
|---|---|
| `M-p` | favorites (no-op if `INSIDE_EMACS`) |
| `M-P` | kids of `$PWD` (defer if it bloats v1) |
| `RET` | `cd` in this pane |
| `M-RET` | new tmux window (or emulator tab) at path |
| `e` in picker | exec configured agent in that cwd |
| `t` in picker | toolbox enter |

### Verbs

| Key | Verb | Effect |
|---|---|
| `t` | toolbox | `toolbox enter <name>` in current pane |
| `g` | grok / build | `cd {path} && grok` (or configured cmd) |
| `e` | exec | generic `{cmd}` with `{path}` |
| `s` | ssh | later |

```toml
[[verbs.agent]]
label = "grok-build"
keys = "g"
cmd = "grok"          # or full Grok Build invocation
# runs after cd to selected favorite

[[verbs.agent]]
label = "codex"
keys = "c"
cmd = "codex -C {path}"
```

Keep `alias tb='toolbox'` in sysmgmt. Picker replaces destination aliases only.

```toml
[keys]
leader = "M-p"
verbs  = "M-x"          # bound only if INSIDE_EMACS is unset
```

## Git vs kids

- Path contains `.git` → project; do not explode `src/` / `target/`.
- Kids listing is optional v1; do not block the CLI + fzf + agent exec path on it.

## Config sketch

`~/.config/symjump/favorites.toml`

```toml
root = "~/worx"

[keys]
leader = "M-p"

[frequent]
source = "zoxide"
max = 20

[[favorites]]
label = "symworx"
path = "~/worx/symworx"
keys = "s"

[[verbs.toolbox]]
label = "python"
name = "dev-python"
keys = "p"

[[verbs.agent]]
label = "grok-build"
keys = "g"
cmd = "grok"
```

Do not invent frecency if zoxide is present.

## sysmgmt split

**Move into symjump:** `cdw`, named project dirs, toolbox destinations, agent launch cwd.

**Keep in sysmgmt:** git aliases, `tb` / `d`, PATH, venv, readline.

```bash
cdw() { sjmp jump "$@"; }
```

## First code path

1. `sjmp list` / `jump` / `pin`
2. fzf + `M-p` (skip Emacs)
3. `sjmp exec -- cmd` / `[[verbs.agent]]` (Grok Build first)
4. `sjmp toolbox`
5. tmux `-c` window only if pane `cd` is not enough
6. ratatui last

Do not start a VTE/GPU emulator.
