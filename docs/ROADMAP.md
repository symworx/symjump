# Roadmap

## 0. Docs (this branch)

- [x] Repo + branch layout
- [x] Design: sidecar, UX, config, org placement
- [x] Meta bindings (`M-p` / `M-P`); no Control
- [x] `M-x` verb palette + toolbox enter; skip when `INSIDE_EMACS`

## 1. Spike (still `feature/initial-build`)

- [ ] Rust bin `sjmp`: read/write `favorites.toml`
- [ ] `list`, `jump`, `pin`, `kids`
- [ ] Git-root detection
- [ ] bash hook + `cdw` wrapper snippet for sysmgmt
- [ ] Bind `M-p` / `M-P`; bind `M-x` only if `INSIDE_EMACS` is unset

## 2. Picker

- [ ] fzf integration (`sjmp pick`)
- [ ] Numbered picks when query empty
- [ ] Tab toggle favorites / kids

## 3. Verbs

- [ ] `sjmp toolbox` / `[[verbs.toolbox]]`
- [ ] `M-x t` → enter selected toolbox in current shell
- [ ] `M-p t` fallback for vterm
- [ ] Optional `spawn = "window"` per entry

## 4. Emulator backends

- [ ] generic `cd` (vterm / any shell)
- [ ] kitty `@ launch --cwd`
- [ ] foot / footclient `--working-directory`
- [ ] gnome-terminal / ptyxis
- [ ] wezterm `start --cwd`

## 5. Polish

- [ ] `sjmp init bash|zsh|fish`
- [ ] zoxide frequent source
- [ ] LICENSE (Apache-2.0 likely)
- [ ] Merge to `develop` when the spike is usable daily
