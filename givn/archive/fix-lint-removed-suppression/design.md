# Design: fix-lint-removed-suppression

## Problem

The `@givn.removed` suppression check in `src/cli/lint.rs` uses
`s.position.line <= wip_line` against ANY removed scenario in the feature.
This suppresses `@wip` findings on unrelated non-removed scenarios that
appear later in the same file.

## Solution

Replace the line-based check with an exact match: a `@wip` finding is
suppressed only when the `@wip` tag belongs to the `@givn.removed` scenario
itself. Since `collect_wip` in `src/gherkin/parse.rs` reports the scenario's
position line as the `@wip` finding line, we check if the finding line
matches a removed scenario's position line exactly:

```rust
let is_in_removed = feature.scenarios.iter().any(|s| {
    is_removed_placeholder(s) && s.position.line == wip_line
});
```

Change `<=` to `==`.

## Step definitions

Two new Given steps (fixture builders, no docstrings — avoids the merge
engine's docstring-dropping bug):

- `a feature file with a removed-wip placeholder at "<path>"` — writes a
  .feature file containing a single `@givn.removed @wip` scenario.
- `a feature file with a removed scenario and a separate wip scenario at
  "<path>"` — writes a .feature file with a `@givn.removed` scenario
  followed by a separate `@wip` scenario.

## Files affected

- `src/cli/lint.rs` — change `<=` to `==` on line 61.
- `tests/steps/mod.rs` — add two fixture-builder Given steps.

## Verify command

```
cargo test --test features_runner
```
