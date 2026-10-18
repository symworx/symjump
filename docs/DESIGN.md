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
3. **`M-x` / `M-Enter` action palette** in that same terminal: toolbox, grok-build, exec.

What this project is not: a second Projectile, a zoxide replacement, an
in-agent `/cd`, a kids file manager in v1, a ratatui app until fzf fails.

tmux: Enter in the picker `cd`s the current pane. `a` opens the dashboard for `[ai] default` (grok, claude, or codex). `l` opens it for `[ai] local`. If `tmux` is not on `PATH`, sjmp prints that and stops. It does not install tmux.

## Placement

| Home | Why |
|---|---|
| `github.com/symworx/symjump` | Current GitHub home |
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
| `M-Enter` | same, when readline receives it | same palette (`\e\C-m` and `\e\C-j`) |
| `M-<key>` | same | jump the favorite whose `keys` is that letter (`M-w`, `M-j`, …) |
| Enter | places (`M-p`) and kids (`M-P`) pickers | `cd` to the highlighted directory |
| `a` | those pickers, query empty | tmux dashboard using `[ai] default` |
| `l` | those pickers, query empty | tmux dashboard using `[ai] local` |
| `t` | action palette, query empty | plain tmux session for `$PWD` |

`a` and `l` are ordinary filter letters once the query is non-empty, same rule as `1`–`9`. `o` is not bound.

`[ai] default` is `grok`, `claude`, `codex`, or `local`. `[ai] local` is the command for `l` (`ollama` unless you set another). Bare `ollama` opens a model list in the left pane (`ollama list` through fzf) and runs `ollama run <name>` only after a pick. No model is loaded before that. With no models installed, the pane says so and drops to a shell so you can `ollama pull <name>`. `ollama run <name>` in `[ai] local` starts that model directly. Any other local command is run as written (a vLLM command can use that until it has its own picker). `M-x` or `M-Enter`, then `t`, runs `sjmp_tmux` for the current directory: one shell, no agent layout. Outside tmux the session name is `sjmp-<directory>` (attach if it exists, otherwise create it). Inside tmux it is a new window in the current session. If tmux is missing, sjmp says so and stops. An `[[actions.agent]]` or toolbox entry whose key is `t` replaces this. `cmd = "sjmp_tmux '{path}'"` still picks a favorite first. `M-x` then `a` runs `sjmp_tmux_dash '{path}'`, which uses `default`. `M-x` then `l` runs `sjmp_tmux_dash '{path}' local` when that action is configured. `SJMP_AGENT_CMD` overrides `default` for one shell. It does not change `l`. A new shell picks up a changed `favorites.toml`.

The dashboard is one tmux window. The left pane runs that agent and leaves a shell when it exits. The right column is two shells. Inside tmux this is a new window in the current session. Outside tmux it is session `sjmp-<directory>`, then attach. The current pane's directory changes only on Enter.

`sjmp pin --keys` refuses letters that are already taken:

- sjmp chords: `p` (`M-p` places), `P` (`M-P` kids), `x` (`M-x` actions)
- readline emacs-mode: `b` / `f` (word motion), `d` (kill-word), `y` (yank-pop)
- any `keys` already used by another favorite

Also leave Control alone (`C-g`, `C-c`, `C-x`, `C-z`). Crowded but still allowed: `M-u` / `M-l` / `M-c` (case), `M-.` (last arg; not a letter so never a favorite Meta bind). `M-Enter` is bound to the action palette, same as `M-x`. Terminals differ on whether Enter arrives as CR or LF, so the hook binds both. A tmux root binding on `M-Enter` (a no-prefix split, for example) consumes the chord before readline. `M-x` is the actions chord that still reaches a shell inside that tmux. The hook stays off when `INSIDE_EMACS` is set, so this bind does not run in vterm until vterm itself writes the `ESC` byte.

### After `M-x` or `M-Enter` (action palette)

Type or hit the letter. Then pick a place if the action needs one.

| Key | Action | Effect |
|---|---|---|
| `t` | tmux | plain session for `$PWD`, unless key `t` is already configured |
| `g` | grok / build | pick favorite → `cd {path} && grok` |
| `e` | exec | configured `{cmd}` with `{path}` |
| `p` | places | same as `M-p` |
| `s` | ssh | later |

`M-x` then `g` is the daily Grok Build path: action first, place second.
Places and kids pickers keep Enter, `a`, and `l`. `t` is the action palette.

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
4. `M-p` places, `M-P` kids, `M-x` and `M-Enter` from `sjmp action list`
5. Numbered picks `1`–`9` only while the fzf query is empty
6. `sjmp action` / `action add` / `action rm` / `exec` / `toolbox`
7. Places and kids pickers: Enter `cd`s; `a` opens the default agent; `l` opens the local agent

Next: spawn backends other than tmux (kitty / foot / gnome / wezterm), zoxide, ratatui if fzf fails.

Do not start a VTE/GPU emulator.
