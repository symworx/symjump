# Roadmap

## 0. Docs

- [x] Repo + branch layout
- [x] Design: sidecar, UX, config
- [x] Meta bindings; no Control; skip Emacs shells other than vterm
- [x] `M-x` action palette (toolbox + grok-build); not bound in Emacs
- [x] Scope lock: tmux + Grok Build cwd; not a second Projectile
- [x] Public-safe wording (no personal-system paths or private-repo names)
- [x] Apache-2.0 `LICENSE`

## 1. CLI (shipped)

- [x] Rust bin `sjmp`: `favorites.toml` (closed TOML subset, no serde)
- [x] `list`, `jump`, `pin`, `unpin`, `kids`
- [x] bash hook + `jmp`
- [x] Bind `M-p` and **`M-x`** outside Emacs; also run the hook in vterm (`INSIDE_EMACS=vterm`), where actions are `M-Enter`
- [x] fzf picker (basic)
- [x] `sjmp exec` / `sjmp action` / `action add` / `action rm` / `sjmp toolbox`

## 2. Hook completeness

- [x] Bind `M-P` → `sjmp kids` + fzf
- [x] Numbered picks `1`–`9` only while the query is empty
- [x] Drive `M-x` from `sjmp action list` (not a hardcoded `g`/`t`/`e` menu)
- [x] Places and kids pickers: Enter `cd`s; `a` opens `[ai] default`; `l` opens `[ai] local`

`sjmp action` / `exec` / `toolbox` print the command line; the shell hook runs it.

## 3. Later

- [ ] kitty / foot / gnome spawn (pane `cd` first)
- [ ] zoxide frequent source
- [ ] ratatui only if fzf fails
