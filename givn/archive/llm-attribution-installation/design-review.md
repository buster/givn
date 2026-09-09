# Design Review: llm-attribution-installation

## Scope

PASS. The change is limited to installation visibility and managed-upgrade
retrofit completeness. Existing attribution behavior is not redesigned.

## Decisions

- `givn init` reports the shared hook, local activation result, and selected
  OpenCode/Claude integrations.
- `givn skills sync` remains the single materialization path.
- `givn upgrade` stages `.claude/settings.json` with the other selected
  attribution paths after its clean-worktree preflight.
- Human commits, model selection, and attribution semantics remain unchanged.
- No ADR is qualified; the design and executable feature are the canonical
  owners.

## Testability

Both scenarios drive the real compiled CLI. The initialization scenario asserts
stdout and generated files. The upgrade scenario creates a clean Git project,
removes attribution files, runs `givn upgrade`, and inspects the managed commit
and regenerated files. Strict Cucumber mode remains `.fail_on_skipped()`.

## E2E Fidelity

The two normalized support interactions each have one `@e2e` scenario and use
real CLI/Git subprocesses. No external services or digital twins are required.

## Arc42 and Language

The change-level Arc42 assessment is complete. Durable chapters 01, 06, and 10
were updated; the existing attribution chapters and glossary provide the
remaining context. The terms Installation report and Attribution retrofit were
added to the glossary and are used consistently.

## Risk

The main risk is staging a user-owned Claude settings file. The upgrade path
requires a clean worktree, merges only the Givn hook entry, and stages the file
through the explicit managed path so the committed diff contains the retrofit
rather than unrelated uncommitted work.

DESIGN-REVIEW: PASS
