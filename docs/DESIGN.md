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
compete with that. The bash hook runs in a normal shell and in vterm
(`INSIDE_EMACS=vterm`). Other Emacs shells still skip it. vterm keeps `M-x`
for Emacs. `M-Enter` is the actions chord in both shells.

The gap is the *other* session: a real terminal (often **tmux**), where Grok
Build / Codex / Claude start. Those tools load `AGENTS.md` / `CLAUDE.md` from
**startup cwd**.

What this project is for:

1. A portable `favorites.toml`.
2. `cd` in Foot / Kitty / tmux / Emacs vterm — `M-p`.
3. **`M-Enter` action palette** in those shells (and `M-x` when readline receives it): toolbox, grok-build, exec.

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
shell hook — run unless INSIDE_EMACS is set to something other than vterm
```

## Bindings: Meta, not Control

Do not take `C-g`, `C-c`, `C-x`, `C-z`. tmux owns its prefix.
Document `alt-sends-escape`.

| Chord | Where | Action |
|---|---|---|
| `M-p` | terminal, tmux, and vterm | places |
| `M-P` | same | kids of `$PWD` |
| **`M-x`** | bare terminal and tmux. vterm keeps this for Emacs | action palette |
| `M-Enter` | terminal, tmux, and vterm | same palette (`\e\C-m` and `\e\C-j`) |
| `M-<key>` | same | jump the favorite whose `keys` is that letter (`M-w`, `M-j`, …) |
| Enter | places (`M-p`) and kids (`M-P`) pickers | `cd` to the highlighted directory |
| `a` | those pickers, query empty | tmux dashboard using `[ai] default` |
| `l` | those pickers, query empty | tmux dashboard using `[ai] local` |
| `t` | action palette, query empty | plain tmux session for `$PWD`, kind `session` |
| letter | action palette, query empty | run the action whose key is that letter |

`a` and `l` in the places and kids pickers are ordinary filter letters once the query is non-empty, same rule as `1`–`9`. `o` is not bound. In the action palette, the filter searches the key and the label. The kind word is not part of the match, so `a` does not rank every `agent` row. `Esc` on a later picker returns to the previous one. `Esc` on the first picker closes sjmp. After an action asks for a place, `Esc` returns to the action palette.

`[ai] default` is `grok`, `claude`, `codex`, or `local`. `[ai] local` is the command for `l` (`ollama` unless you set another). Bare `ollama` opens a model list in the left pane (`ollama list` through fzf) and runs `ollama run <name>` only after a pick. No model is loaded before that. With no models installed, the pane says so and drops to a shell so you can `ollama pull <name>`. `ollama run <name>` in `[ai] local` starts that model directly. Any other local command is run as written (a vLLM command can use that until it has its own picker). `M-x` or `M-Enter`, then a letter, runs the action with that key while the query is empty. `t` runs `sjmp_tmux` for the current directory when that key is free: one shell, no agent layout. That built-in row is kind `session`. The session name is `sjmp-<directory>`. If that session exists, sjmp switches to it. Otherwise it creates the session. Inside tmux this is `switch-client`, not a new window in the current session. If tmux is missing, sjmp says so and stops. An `[[actions.agent]]` or toolbox entry whose key is `t` replaces this. `cmd = "sjmp_tmux '{path}'"` still picks a favorite first. `M-x` then `a` runs `sjmp_tmux_dash '{path}'`, which uses `default`. `M-x` then `l` runs `sjmp_tmux_dash '{path}' local` when that action is configured. `SJMP_AGENT_CMD` overrides `default` for one shell. It does not change `l`. A new shell picks up a changed `favorites.toml`.

The dashboard opens a new tmux window. The left pane runs that agent and leaves a shell when it exits. The right column is two shells. Inside tmux, the window is added to the current session and the other windows stay. Outside tmux, it is added to session `sjmp-<directory>`, which is created the first time. The current pane's directory changes only on Enter.

`sjmp pin --keys` refuses letters that are already taken:

- sjmp chords: `p` (`M-p` places), `P` (`M-P` kids), `x` (`M-x` actions)
- readline emacs-mode: `b` / `f` (word motion), `d` (kill-word), `y` (yank-pop)
- any `keys` already used by another favorite

Also leave Control alone (`C-g`, `C-c`, `C-x`, `C-z`). Crowded but still allowed: `M-u` / `M-l` / `M-c` (case), `M-.` (last arg; not a letter so never a favorite Meta bind). `M-Enter` is the actions chord in a bare terminal, in tmux, and in vterm. `M-x` is the same palette when readline receives it. vterm does not send `M-x`, so Emacs `execute-extended-command` still runs there. Terminals differ on whether Enter arrives as CR or LF, so the hook binds both. A tmux root binding on `M-Enter` consumes the chord before readline. The hook runs when `INSIDE_EMACS` is unset or `vterm`, and it calls `bind` only in an interactive shell. comint and other Emacs shells still skip it.

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
actions = "M-x"   # readline alias; vterm uses M-Enter
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

A `[[actions.toolbox]]` entry works with either `toolbox` or `distrobox`.
The enter command prefers `toolbox` if it is on `PATH`, otherwise `distrobox`.
Distrobox already abstracts the underlying runtime (podman or docker), so a
separate docker enter action is not required for named environments.

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
3. bash hook + `jmp`; run when `INSIDE_EMACS` is unset or `vterm`; first `sjmp` on the default path writes `~/.config/symjump/favorites.toml` if missing
4. `M-p` places, `M-P` kids, `M-Enter` actions in the terminal and in vterm, `M-x` when readline receives it
5. Numbered picks `1`–`9` only while the fzf query is empty
6. `sjmp action` / `action add` / `action rm` / `exec` / `toolbox`
7. Places and kids pickers: Enter `cd`s; `a` opens the default agent; `l` opens the local agent

Next: spawn backends other than tmux (kitty / foot / gnome / wezterm), zoxide, ratatui if fzf fails.

Do not start a VTE/GPU emulator.
