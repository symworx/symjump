# Contributing to symjump

Thank you for your interest. This is a small sibling of [SymWorx](https://github.com/symworx/symworx): a portable directory / action launcher (`sjmp`), **not** the biosignal workspace.

## Philosophy

- **Portable over personal.** Config and hooks should work on someone else’s machine without rewriting their bashrc.
- **Sidecar, not an emulator plugin.** Kitty / foot / GNOME / tmux are spawn targets, not hosts for a shared ABI.
- **Places vs actions.** Places live here. Git/docker *commands* stay in the user’s shell kit. Toolbox *destinations* can live here.
- **Meta, not Control.** Do not steal `C-g`, `C-c`, `C-x`, `C-z`. Do not bind `M-x` when `$INSIDE_EMACS` is set.

Same quality bar as SymWorx: explicit errors, little `unsafe`, no surprise dependencies.

## AI-assisted contributions

AI tools (Grok, Claude, Copilot, …) are fine. Contributors must:

1. Be able to explain the change.
2. Meet the quality and keybinding rules above.
3. Own the submitted code.

See [AGENTS.md](AGENTS.md).

## Ways to contribute

- Issues (bugs, ports, emulator backends)
- PRs (docs-only is welcome)
- Tests with behavior changes
- Review

No need to wait for an issue assignment.

## Getting started

1. Fork / clone.
2. Read [docs/DESIGN.md](docs/DESIGN.md) and [DEVELOPMENT.md](DEVELOPMENT.md).
3. Branch from **`worx`**: `git checkout -b feature/your-change`.
4. Keep the change focused; update docs when behavior or bindings change.
5. Open a PR against **`worx`**.

Until GitHub renames the default branch, PRs still target **`develop`**.

```bash
cargo test --workspace
cargo run -p sjmp -- list
```

## Release path

GitHub Flow: feature PRs go to **`worx`**. A release is a version bump on
`worx`, then a **manual** tag `vX.Y.Z`:

```text
feature/*  ──PR──►  worx  ──tag──►  vX.Y.Z
```

Same shape as SymWorx; no crates.io publish until LICENSE is set.

## Pull requests

- One logical change when possible.
- Tests when adding behavior.
- Update DESIGN / README if you touch bindings or backends.
- Stay engaged on review comments.

## Code of Conduct

[Contributor Covenant v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

Apache-2.0 intended (`Cargo.toml`). LICENSE file at public release.

## Questions

Open a Discussion or Issue on this repository.
