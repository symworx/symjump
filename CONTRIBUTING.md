# Contributing to symjump

Thank you for your interest. This is a portable directory / action launcher (`sjmp`).

## Philosophy

- **Portable over personal.** Config and hooks should work on someone else’s machine without rewriting their bashrc.
- **Sidecar, not an emulator plugin.** Kitty / foot / GNOME / tmux are spawn targets, not hosts for a shared ABI.
- **Places vs actions.** Places live here. Git/docker *commands* stay in the user’s shell kit. Toolbox *destinations* can live here.
- **Meta, not Control.** Do not steal `C-g`, `C-c`, `C-x`, `C-z`. Do not bind `M-x` when `$INSIDE_EMACS` is set.

Quality bar: explicit errors, little `unsafe`, no surprise dependencies.

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

```bash
cargo test --workspace
cargo run -p sjmp -- list
```

## Release path

**SymWorx org standard** (GitHub Flow): feature PRs go to **`worx`**. A release is a version bump on
`worx`, then a **manual** tag `vX.Y.Z`:

```text
feature/*  ──PR──►  worx  ──tag──►  vX.Y.Z
```

Publish to crates.io from a `vX.Y.Z` tag on `worx`, in order: `symjump-config`,
then `symjump-core`, then `sjmp`. Details: [DEVELOPMENT.md](DEVELOPMENT.md#releasing).

## Pull requests

- One logical change when possible.
- Tests when adding behavior.
- Update DESIGN / README if you touch bindings or backends.
- Stay engaged on review comments.

## Code of Conduct

[Contributor Covenant v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

[Apache License 2.0](LICENSE).

## Questions

Open a Discussion or Issue on this repository.
