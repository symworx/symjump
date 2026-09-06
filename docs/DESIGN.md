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

## UX

One shortcut (prefer shell leader so it works in vterm):

| Key | Action |
|---|---|
| `Alt-p` or `Ctrl-G` | Open list (favorites) |
| `Tab` | Toggle favorites ↔ subdirs of current/highlighted |
| `j` `k` / arrows | Move |
| `1`–`9` | Pick when query is empty |
| type or `/` | Filter (numbers stop being hotkeys) |
| `Enter` | `cd` there |
| `Ctrl-Enter` | new emulator window/tab there |
| `p` | pin `$PWD` |
| `Esc` | close |

### Git vs kids

- Path contains `.git` → treat as a project; do not explode into `src/` / `target/` / `.venv`.
- No `.git` → list immediate subdirs (skip hidden, `node_modules`, `target` unless asked).
- Optional: Enter on a non-repo drills in; `h` goes up.

## Config sketch

`~/.config/symjump/favorites.toml`

```toml
root = "~/worx"

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

Launcher = nouns (places). Aliases = verbs.

```bash
cdw() { sjmp jump "$@"; }
# no args → picker
```

SSH host aliases can later be a second list with an `ssh` backend; not v1.

## First code path (when implementation starts)

1. `sjmp list` / `sjmp jump <id>` / `sjmp pin` — no TUI
2. Pipe `sjmp list` to `fzf` from the bash hook
3. `sjmp kids` for non-git subdirs
4. Spawn backends
5. Native ratatui picker only if fzf is insufficient

Do not start by writing a VTE/GPU emulator.
