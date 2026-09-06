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
favorites.toml + frecency + verbs (toolbox, ssh, exec)
        ↓
sjmp (CLI + TUI picker)
        ↓
  backends: kitty | foot | gnome/ptyxis | wezterm | toolbox | generic (cd)
        ↓
shell hook (bash/zsh/fish)  +  optional Kitty kitten later
```

Detect context via `$KITTY_WINDOW_ID`, `$WEZTERM_PANE`, `$TERM`, `XDG_CURRENT_DESKTOP`, `$INSIDE_EMACS`.
foot and GNOME only need `--working-directory` (or `footclient`).

## Bindings: Meta, not Control

Ctrl is already loaded (Emacs, readline, tmux, shells). Do not take `C-g`
(keyboard-quit), `C-c`, `C-x`, or `C-z`.

Use **Meta** (`M-`, Alt / ESC-prefix), same family as Emacs `M-x` / `M-.`.
In bash that is `\ep` for `M-p`. Terminal caveat: document `alt-sends-escape`
(Kitty / foot) so Meta is one chord.

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
| `M-RET` | | New emulator window/tab there |
| `p` | pin | Pin `$PWD` (only in picker) |
| `ESC` | | Close |

### Verbs: `M-x` in a real terminal, not in Emacs

`M-x` is the right *shape* for “pick a verb, then a target” — toolbox enter,
ssh, exec. It must **not** be bound when `$INSIDE_EMACS` is set. In Emacs,
`M-x` is `execute-extended-command`; steal it and vterm/Emacs is broken.

| Context | Chord | Opens |
|---|---|---|
| Kitty / foot / GNOME / WezTerm (no Emacs) | `M-x` | verb palette |
| Emacs `vterm` / `INSIDE_EMACS` | do not bind `M-x` | use `M-p t` or `M-p x` |
| Any | `M-p` | places (always safe) |

Verb palette (after `M-x`, type or number):

| Key | Verb | Effect |
|---|---|---|
| `t` | toolbox | list configured toolboxes → `toolbox enter <name>` |
| `p` | places | same as `M-p` |
| `d` | dirs | kids of `$PWD` |
| `s` | ssh | host list (later) |
| `e` | exec | command in selected cwd (later) |

Selecting `t` then a row runs the enter command in the **current** shell
(so you land inside the toolbox), not a nested extra window, unless the
entry says `spawn = "window"`.

Commented sysmgmt aliases become config, not bash:

```toml
[[verbs.toolbox]]
label = "python"
name = "dev-python"
keys = "p"
# printf banner optional

[[verbs.toolbox]]
label = "rust"
name = "dev-rust"
keys = "r"
```

`sjmp toolbox` / `sjmp verb toolbox` is the CLI; `M-x t` is the UI.
Keep `alias tb='toolbox'` in sysmgmt for raw CLI. The picker replaces
`tb-python`, `tb-aws`, not `tb` itself.

```toml
[keys]
leader = "M-p"
kids   = "M-P"
verbs  = "M-x"          # bound only if INSIDE_EMACS is unset
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
verbs  = "M-x"

[frequent]
source = "zoxide"
max = 20

[[favorites]]
label = "worx"
path = "~/worx"
keys = "w"

[[favorites]]
label = "symworx"
path = "~/worx/symworx"
keys = "s"

[[verbs.toolbox]]
label = "python"
name = "dev-python"
keys = "p"
```

Actions: `spawn { cwd, cmd? }`, `open_favorites`, `pin_cwd`, `toolbox enter`.

Do not invent frecency if zoxide is present; call `zoxide query --list --score`.

## sysmgmt split

**Move into symjump:** `cdw`, named project dirs, toolbox *destinations*.

**Keep in sysmgmt:** git aliases (`gs`, `gacp`, …), docker/toolbox *commands* (`d`, `tb`), `paths.sh`, venv helpers, readline history binds.

```bash
cdw() { sjmp jump "$@"; }
# no args → picker
```

## First code path (when implementation starts)

1. `sjmp list` / `sjmp jump <id>` / `sjmp pin` — no TUI
2. fzf hook on `M-p`
3. `sjmp kids` (`M-P` or Tab)
4. `sjmp toolbox` + verb palette on `M-x` when not in Emacs
5. Spawn backends (`M-RET`)
6. ratatui only if fzf is insufficient

Do not start by writing a VTE/GPU emulator.
