# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Bare `[ai] local = "ollama"` lists models in the dashboard pane and runs `ollama run` only after a pick. `ollama run <name>` still starts that model. The sample `M-x` action `l` is `sjmp_tmux_dash '{path}' local`.

- `M-x` or `M-Enter`, then `t`, opens a plain tmux session for the current directory (`sjmp_tmux`). Inside tmux this is a new window. Outside tmux it attaches to `sjmp-<directory>` or creates that session. An action with key `t` replaces this default.

### Changed

- The bash hook also binds `M-Enter` to the action palette (`\e\C-m` and `\e\C-j`), same as `M-x`. The hook stays off when `INSIDE_EMACS` is set. A tmux root binding on `M-Enter` still wins over the shell.

## [0.2.0] - 2026-10-16

### Added

- Places and kids pickers: Enter `cd`s. `a`, while the query is empty, opens a tmux window whose left pane is `[ai] default` (`grok`, `claude`, `codex`, or `local`). `l` opens `[ai] local`. `SJMP_AGENT_CMD` overrides `a` only. If tmux is missing, sjmp tells you to install it.

## [0.1.0] - 2026-09-21

### Added

- First crates.io release: `symjump-config`, `symjump-core`, then `sjmp`.
- CLI: `list`, `jump`, `pin`, `unpin`, `kids`, `exec`, `action` / `action add` /
  `action rm`, `toolbox`, bash hook + `jmp`.
- Apache-2.0 `LICENSE`.

[Unreleased]: https://github.com/symworx/symjump/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/symworx/symjump/releases/tag/v0.1.0
