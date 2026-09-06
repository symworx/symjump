# Roadmap

## 0. Docs (this branch)

- [x] Repo + branch layout
- [x] Design: sidecar, UX, config, org placement
- [x] Meta bindings; no Control; skip Emacs
- [x] `M-x` verb palette (toolbox + grok-build); not bound in Emacs
- [x] Scope lock: tmux + Grok Build cwd; not a second Projectile

## 1. Spike

- [ ] Rust bin `sjmp`: `favorites.toml`
- [ ] `list`, `jump`, `pin`
- [ ] bash hook + `cdw`
- [ ] Bind `M-p` and **`M-x`** only if `INSIDE_EMACS` unset
- [ ] fzf picker

## 2. Verbs (`M-x` then letter)

- [ ] `M-x g` — `[[verbs.agent]]` Grok Build (`cd {path} && grok`)
- [ ] `M-x t` — toolbox enter
- [ ] `M-x e` — generic exec
- [ ] `sjmp exec` / `sjmp verb`
- [ ] Optional: `tmux new-window -c {path}` on `M-RET`

## 3. Later

- [ ] kids listing
- [ ] kitty / foot / gnome spawn (pane `cd` first)
- [ ] zoxide frequent source
- [ ] ratatui only if fzf fails
- [ ] LICENSE; merge to `develop`
