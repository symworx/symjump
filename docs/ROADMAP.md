# Roadmap

## 0. Docs (this branch)

- [x] Repo + branch layout
- [x] Design: sidecar, UX, config, org placement

## 1. Spike (still `feature/initial-build`)

- [ ] Rust bin `sjmp`: read/write `favorites.toml`
- [ ] `list`, `jump`, `pin`, `kids`
- [ ] Git-root detection
- [ ] bash hook + `cdw` wrapper snippet for sysmgmt

## 2. Picker

- [ ] fzf integration (`sjmp pick`)
- [ ] Numbered picks when query empty
- [ ] Tab toggle favorites / kids

## 3. Emulator backends

- [ ] generic `cd` (vterm / any shell)
- [ ] kitty `@ launch --cwd`
- [ ] foot / footclient `--working-directory`
- [ ] gnome-terminal / ptyxis
- [ ] wezterm `start --cwd`

## 4. Polish

- [ ] `sjmp init bash|zsh|fish`
- [ ] zoxide frequent source
- [ ] LICENSE (Apache-2.0 likely)
- [ ] Merge to `develop` when the spike is usable daily
