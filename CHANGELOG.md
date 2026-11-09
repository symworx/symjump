# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

## [0.3.0] - 2026-11-09

### Added

- `[dashboard]` sets the dashboard geometry. `direction` is `vertical` (left | right) or `horizontal` (top / bottom). Each `[[dashboard.pane]]` has a `size` along that split, and an optional `sub` the other way. Percents sum to 100. The default is `66/34` with `50/50` on the right pane. `sjmp config` walks the direction, the split counts, and the percentages. The layout is read when the dashboard opens.

- After an agent action asks for a place, Tab lists one level of children of the highlighted favorite (`sjmp kids`). The rows are numbered `01`, `02`, and so on. Type that number or the name and press Enter. `01` selects the first row and `10` selects row 10. Enter still uses the pin. Esc on the child list returns to the pins. A pin with no children stays on the place list.

### Changed 

## [0.2.1] - 2026-10-23

### Added

- Bare `[ai] local = "ollama"` lists models in the dashboard pane and runs `ollama run` only after a pick. `ollama run <name>` still starts that model. The sample `M-x` action `l` is `sjmp_tmux_dash '{path}' local`.

- `M-x` or `M-Enter`, then `t`, opens a plain tmux session for the current directory (`sjmp_tmux`). The session is `sjmp-<directory>`. If it exists, sjmp switches to it. An action with key `t` replaces this default.

### Changed

- `sjmp_tmux` (plain `t`) does not add a window to the current session. Each directory is session `sjmp-<directory>`. Opening it again switches to that session.
- The agent dashboard adds a tmux window. Inside tmux the other windows stay. Outside tmux the window is added to `sjmp-<directory>`.
- `Esc` on a later sjmp picker returns to the previous one. `Esc` on the first picker closes it. After `M-Enter` then `a`, `Esc` on the place list returns to the action palette.
- The bash hook calls `bind` only in an interactive shell. `toolbox run` and `bash -c` no longer warn `line editing not enabled`.
- In the action palette, a letter runs the action with that key while the query is empty. The filter matches the key and the label. The built-in plain tmux row is kind `session`, so `a` does not match it through the word `agent`.
- The bash hook also binds `M-Enter` to the action palette (`\e\C-m` and `\e\C-j`), same as `M-x`. The hook stays off when `INSIDE_EMACS` is set to something other than `vterm`. A tmux root binding on `M-Enter` still wins over the shell.
- The hook runs in Emacs vterm. `M-Enter` is the actions chord there. `M-x` stays Emacs because vterm does not send it.

## [0.2.0] - 2026-10-16

### Added

- Places and kids pickers: Enter `cd`s. `a`, while the query is empty, opens a tmux window whose left pane is `[ai] default` (`grok`, `claude`, `codex`, or `local`). `l` opens `[ai] local`. `SJMP_AGENT_CMD` overrides `a` only. If tmux is missing, sjmp tells you to install it.

## [0.1.0] - 2026-09-21

### Added

- First crates.io release: `symjump-config`, `symjump-core`, then `sjmp`.
- CLI: `list`, `jump`, `pin`, `unpin`, `kids`, `exec`, `action` / `action add` /
  `action rm`, `toolbox`, bash hook + `jmp`.
- Apache-2.0 `LICENSE`.

[Unreleased]: https://github.com/symworx/symjump/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/symworx/symjump/releases/tag/v0.3.0
[0.2.1]: https://github.com/symworx/symjump/releases/tag/v0.2.1
[0.2.0]: https://github.com/symworx/symjump/releases/tag/v0.2.0
[0.1.0]: https://github.com/symworx/symjump/releases/tag/v0.1.0
