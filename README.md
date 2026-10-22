# symjump

Pinned places and a few actions for a real terminal. Binary: `sjmp`.

Not an emulator, not a project.el replacement, not your git/docker aliases.
`eval "$(sjmp init bash)"` is the hook. It runs in a normal shell and in Emacs vterm (`INSIDE_EMACS=vterm`). Other Emacs shells skip it.

## Why

Tired of aliases? I was.  
Prefer keybindings? I do.  

My bash aliases were getting out of hand and while it worked for me, I started to see 
some drift and some outdated/unused aliases. I wanted a simpler solution so I took some 
learnings from [omarchy](https://omarchy.org) and created `symjump`. 

## Install

```bash
cargo install sjmp
sjmp                       # first run writes ~/.config/symjump/favorites.toml
eval "$(sjmp init bash)"   # defines jmp; binds M-p / M-P / M-x / M-Enter
sjmp pin --current --label src --keys r
sjmp action add agent --cmd grok --keys g --label grok-build
sjmp action add toolbox --name dev-python --keys p --label python
sjmp list
```

From a git checkout: `cargo install --path crates/sjmp`.

`cargo install` only puts `sjmp` on `PATH`. The binary creates the config file
on first run (default path only; `--config` is left alone) and will not
overwrite an existing file. If `sjmp` is missing from `PATH`, the hook does
nothing.

```bash
sjmp action rm grok-build
sjmp action rm agent python   # pass agent|toolbox when the query matches both
```

`sjmp --help`, `sjmp pin --help`, and `sjmp action --help` cover the rest.
A fuller sample is [examples/favorites.toml](examples/favorites.toml).

## Bindings

Meta, not Control. The same chords work in a bare terminal, in tmux, and in Emacs vterm. vterm keeps `M-x` for Emacs, so actions there are `M-Enter`. See [docs/DESIGN.md](docs/DESIGN.md).

| Chord | Action |
|---|---|
| `M-p` | favorites / places |
| `M-P` | kids of `$PWD` |
| `M-Enter` | action palette in a bare terminal, tmux, and vterm |
| `M-x` | same palette when readline receives it (vterm keeps this key) |
| `M-<key>` | jump the favorite with that `keys` letter |
| Enter | in the places or kids picker: `cd` |
| `a` | in those pickers, query empty: dashboard for `[ai] default` (grok, claude, or codex) |
| `l` | in those pickers, query empty: dashboard for `[ai] local` |
| letter | in the action palette, query empty: run the action with that key. `t` is a plain tmux session unless that key is already set |
| Esc | on a later picker: back to the previous one. On the first picker: close |

The dashboard is one tmux session, `sjmp-<directory>`. `a` starts `[ai] default`. `l` starts `[ai] local` (default `ollama`). Bare `ollama` lists local models in the left pane and loads one only after you pick it. `ollama run <name>` starts that model. Two shells sit on the right. If that session already exists, sjmp switches to it and does not add a window. If tmux is not installed, sjmp says so and stops.

`M-x` or `M-Enter`, then a letter, runs the action with that key while the query is empty. The filter matches the key and the label. It does not match the kind word. `t` opens a plain tmux session for the current directory. No agent and no extra panes. That default row is kind `session`. This switches to `sjmp-<directory>`, or creates that session. It does not add a window to the session you are already in. An action whose key is already `t` replaces that default. `sjmp_tmux '{path}'` still asks for a favorite, then opens that directory.

`M-Enter` or `M-x`, then `a`, asks for a place and then opens the default dashboard when the action is `sjmp_tmux_dash '{path}'`. `Esc` there returns to the action palette. `Esc` on that palette closes it. `M-Enter` or `M-x`, then `l`, opens the local one when the action is `sjmp_tmux_dash '{path}' local`. `SJMP_AGENT_CMD` overrides that default for one shell, not `l`. Other actions are unchanged: Enter there still runs the highlighted action. Inside tmux, a root binding on `M-Enter` runs first. This config leaves `M-Enter` unbound, so it reaches the shell. In vterm, `M-Enter` is the actions chord. `M-x` stays Emacs.

`pin --keys` refuses `p` / `P` / `x` (sjmp chords), `b` / `f` / `d` / `y`
(readline), and letters already used by another favorite.

The fzf picker accepts numbered picks `1`–`9` only while the query is empty.

**SymWorx org standard** (GitHub Flow):

| Branch | Role |
|---|---|
| `worx` | Default. Feature PRs land here. |
| `release/vX.Y.Z` | Optional freeze |

## Docs

- [docs/DESIGN.md](docs/DESIGN.md) — architecture, chords, config
- [docs/ROADMAP.md](docs/ROADMAP.md) — shipped vs next
- [CONTRIBUTING.md](CONTRIBUTING.md) — PRs target `worx`
- [DEVELOPMENT.md](DEVELOPMENT.md) — build and release path

## License

[Apache License 2.0](LICENSE).
