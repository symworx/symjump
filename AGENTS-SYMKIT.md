<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# AGENTS.md — engineering workspace

Instructions for AI coding agents in a **software** repository (apps,
libraries, CLIs). Specs live in the **product** harness; voice in
**creative**.

## Mission

Change the smallest honest slice. Match this repo’s language, layout, and
test runner. Prefer working, reviewable diffs over new frameworks.

## Source of engineering truth

1. Existing code, tests, and lockfiles
2. `AGENTS.md` / CONTRIBUTING / README in this repo
3. Written specs (`docs/prd/`, `features/`) when present
4. The human reviewing the diff

**Do not invent** a second stack, a marketplace, or “we shipped this” if
the tests or git do not show it.

## Layout (typical)

| Path | Role |
|:-----|:-----|
| `src/` | Library or application code |
| `tests/` | Automated tests (or the repo’s existing test tree) |
| `features/` | Gherkin, if the repo uses it |

Follow the tree that is already here. Do not add a parallel `src/` if the
project already has one.

## Agent behavior

- Read neighboring files before choosing names, errors, or test style.
- If a shared kernel is documented, wrap it — do not fork it.
- Tests are part of the change, not a follow-up promise.
- Product framing belongs to the `product` harness. Do not write a PRD
  unless asked.

## Related rules / skills

- `match-repo.md`
- Skills come from the installed role (`catalog.yaml`): `write-tests`,
  `write-docs`, `write-gherkin`, `repro-check`.
