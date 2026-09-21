# sjmp

Pinned places and a few actions for a real terminal. This crate is the `sjmp`
CLI for [symjump](https://github.com/symworx/symjump).

Not an emulator, not a project.el replacement, not your git/docker aliases.
`eval "$(sjmp init bash)"` is the hook; it stays off when `$INSIDE_EMACS` is set.

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

Depends on `symjump-config` and `symjump-core` (published in that order, then
this crate).

## License

Apache License 2.0.
