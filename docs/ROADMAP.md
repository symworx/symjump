# Roadmap

## 0. Docs

- [x] Repo + branch layout
- [x] Design: sidecar, UX, config, org placement
- [x] Meta bindings; no Control; skip Emacs
- [x] `M-x` action palette (toolbox + grok-build); not bound in Emacs
- [x] Scope lock: tmux + Grok Build cwd; not a second Projectile
- [x] Public-safe wording (no personal-system paths or private-repo names)

## 1. Spike (done on `feature/cli-build`)

- [x] Rust bin `sjmp`: `favorites.toml` (closed TOML subset, no serde)
- [x] `list`, `jump`, `pin`, `kids`
- [x] bash hook + `cdw`
- [x] Bind `M-p` and **`M-x`** only if `INSIDE_EMACS` unset
- [x] fzf picker (basic)
- [x] `sjmp exec` / `sjmp action` / `sjmp toolbox`

## 2. Hook completeness

- [x] Bind `M-P` → `sjmp kids` + fzf
- [x] Numbered picks `1`–`9` only while the query is empty
- [x] Drive `M-x` from `sjmp action list` (not a hardcoded `g`/`t`/`e` menu)
- [ ] Optional: `tmux new-window -c {path}` on `M-RET`

`sjmp action` / `exec` / `toolbox` print the command line; the shell hook runs it.

## 3. Later

- [ ] kitty / foot / gnome spawn (pane `cd` first)
- [ ] zoxide frequent source
- [ ] ratatui only if fzf fails
- [ ] LICENSE file; merge to `develop`
