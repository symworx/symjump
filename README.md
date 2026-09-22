# symjump

Pinned places and a few actions for a real terminal. Binary: `sjmp`.

Not an emulator, not a project.el replacement, not your git/docker aliases.
`eval "$(sjmp init bash)"` is the hook; it stays off when `$INSIDE_EMACS` is set.

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
eval "$(sjmp init bash)"   # defines jmp; binds M-p / M-P / M-x
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

Meta, not Control. Bound outside Emacs only. See [docs/DESIGN.md](docs/DESIGN.md).

| Chord | Action |
|---|---|
| `M-p` | favorites / places |
| `M-P` | kids of `$PWD` |
| `M-x` | action palette (`sjmp action list`) |
| `M-<key>` | jump the favorite with that `keys` letter |
| `M-RET` | new tmux window/tab (not bound yet) |

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
