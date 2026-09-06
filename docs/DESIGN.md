# symjump design

## Problem

Directory hops today live as one-off shell helpers, chiefly:

```bash
cdw() { cd "$HOME/worx/$1" ; }
```

in `ntberry/sysmgmt` (`bash/bashrc.d/environment.sh`), plus commented toolbox destinations.

That does not scale across machines or emulators, and it is the wrong layer for a growing list of labs/repos.

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
favorites.toml + frecency
        ↓
sjmp (CLI + TUI picker)
        ↓
  backends: kitty | foot | gnome/ptyxis | wezterm | generic (cd / print path)
        ↓
shell hook (bash/zsh/fish)  +  optional Kitty kitten later
```

Detect context via `$KITTY_WINDOW_ID`, `$WEZTERM_PANE`, `$TERM`, `XDG_CURRENT_DESKTOP`.
foot and GNOME only need `--working-directory` (or `footclient`).

## Bindings: Meta, not Control

Ctrl is already loaded (Emacs, readline, tmux, shells). Do not take `C-g`
(keyboard-quit), `C-c`, `C-x`, or `C-z`.

Use **Meta** (`M-`, Alt / ESC-prefix), same family as Emacs `M-x` / `M-.`.
In bash that is `\ep` for `M-p`. In vterm, Meta still reaches the shell if
`vterm-keymap-exceptions` does not steal it — keep the chord off Emacs's
core map.

Terminal caveat: some emulators send Alt as ESC+key. That is fine for a
picker leader; document `alt-sends-escape` (Kitty / foot) so `M-p` is one
chord, not a stray ESC.

### v1 chords (nouns — places)

| Key | Emacs-ish | Action |
|---|---|---|
| `M-p` | places | Open favorites picker |
| `M-P` | places / kids | Open picker on subdirs of `$PWD` |
| *(inside picker)* `Tab` | | Toggle favorites ↔ kids |
| `j` `k` / arrows | | Move |
| `1`–`9` | | Pick when query is empty |
| type or `/` | | Filter |
| `RET` | | `cd` |
| `M-RET` | | New emulator window/tab there (not `C-RET`) |
| `p` | pin | Pin `$PWD` (only in picker) |
| `ESC` | | Close |

`M-p` is the one global bind. Everything else is modal inside the picker
so it cannot clash with Emacs or git aliases.

### Later: verbs (optional prefix)

Yes — treat verbs as a **second layer**, not more global Ctrl chords.

Pattern: `M-p` then a letter, or a rare `M-s` prefix (`s` = sjmp) only if
one chord is not enough.

| After `M-p` (or `M-s`) | Verb | Meaning |
|---|---|---|
| `p` / `RET` | places | favorites list (default) |
| `d` | dirs / kids | subdirs of `$PWD` |
| `i` | pin | pin current dir |
| `w` | window | spawn new terminal at selection |
| `t` | toolbox | toolbox destinations (sysmgmt leftovers) |
| `s` | ssh | host list (not v1) |
| `x` | exec | run configured command in that cwd |

That is Emacs `C-x` / `M-x` thinking: one prefix, discoverable verbs,
nothing stolen from Control.

Do **not** bind verbs as global `M-c`, `M-x`, `M-w` — those are Emacs.
If a verb needs a global chord, use `M-p` + letter only.

Config should list chords so sysmgmt does not hard-code them:

```toml
[keys]
leader = "M-p"          # open picker
kids   = "M-P"          # optional second chord
# prefix verbs live inside the picker; not global
```

## Git vs kids

- Path contains `.git` → treat as a project; do not explode into `src/` / `target/` / `.venv`.
- No `.git` → list immediate subdirs (skip hidden, `node_modules`, `target` unless asked).
- Optional: Enter on a non-repo drills in; `h` goes up.

## Config sketch

`~/.config/symjump/favorites.toml`

```toml
root = "~/worx"

[keys]
leader = "M-p"
kids   = "M-P"

[frequent]
source = "zoxide"   # or "internal"
max = 20

[[favorites]]
label = "worx"
path = "~/worx"
keys = "w"

[[favorites]]
label = "symworx"
path = "~/worx/symworx"
keys = "s"
```

Actions: `spawn { cwd, cmd? }`, `open_favorites`, `pin_cwd` (need OSC 7 or `$PWD` from the hook).

Do not invent frecency if zoxide is present; call `zoxide query --list --score`.

## sysmgmt split

**Move into symjump:** `cdw`, named project dirs, toolbox *destinations* as optional spawn entries.

**Keep in sysmgmt:** git aliases (`gs`, `gacp`, …), docker/toolbox *commands* (`d`, `tb`), `paths.sh`, venv helpers, readline history binds.

Launcher = nouns (places). Aliases = verbs — until/unless those verbs are
picker-prefixed as above. Do not delete `gs` in favor of `M-p g`.

```bash
cdw() { sjmp jump "$@"; }
# no args → picker
```

SSH host aliases can later be a second list with an `ssh` backend; not v1.

## First code path (when implementation starts)

1. `sjmp list` / `sjmp jump <id>` / `sjmp pin` — no TUI
2. Pipe `sjmp list` to `fzf` from the bash hook bound to `M-p`
3. `sjmp kids` for non-git subdirs (`M-P` or Tab)
4. Spawn backends (`M-RET` in picker)
5. Native ratatui picker only if fzf is insufficient
6. Verb prefix only after the single-chord flow is daily-driver quality

Do not start by writing a VTE/GPU emulator.
