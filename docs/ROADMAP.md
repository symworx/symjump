# Roadmap

## 0. Docs (this branch)

- [x] Repo + branch layout
- [x] Design: sidecar, UX, config, org placement
- [x] Meta bindings; no Control; skip Emacs
- [x] Toolbox verbs
- [x] Scope lock: tmux + Grok Build cwd; not a second Projectile

## 1. Spike

- [ ] Rust bin `sjmp`: `favorites.toml`
- [ ] `list`, `jump`, `pin`
- [ ] bash hook + `cdw`; `M-p` only if `INSIDE_EMACS` unset
- [ ] fzf picker

## 2. Verbs that justify the tool

- [ ] `[[verbs.agent]]` — `cd {path} && grok` (Grok Build)
- [ ] `sjmp exec --cmd … --path …`
- [ ] `[[verbs.toolbox]]` / `sjmp toolbox`
- [ ] Optional: `tmux new-window -c {path}` on `M-RET`

## 3. Later

- [ ] kids listing
- [ ] kitty / foot / gnome spawn (pane `cd` first)
- [ ] zoxide frequent source
- [ ] ratatui only if fzf fails
- [ ] LICENSE; merge to `develop`
