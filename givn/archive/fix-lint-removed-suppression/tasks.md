# Tasks — fix-lint-removed-suppression

- [x] **Make the delta spec discoverable by the test runner**
  - Create file-level symlink: `givn/specs/lint-command/fix-lint-removed-suppression.feature`
    → `../../changes/fix-lint-removed-suppression/specs/lint-command/lint-command.feature`.
  - Run `cargo test --test features_runner` → new scenarios RED (missing step defs).

- [x] **Scenario: A removed placeholder suppresses the @wip warning on itself**
  - RED: Remove `@wip` from this scenario only. Write the Given step
    `a feature file with a removed-wip placeholder at "<path>"`. Run verify →
    must FAIL (the step is new, and the code suppresses via `<=` which
    still works for this case, so the assertion should pass — if it passes,
    confirm it's real).
  - GREEN: This scenario should pass even with the current `<=` bug (the
    removed scenario IS the one with @wip, so `<=` happens to work). Confirm
    GREEN. Fix the code anyway (`<=` → `==`) in the next task.
  - REFACTOR: no seam needed.

- [x] **Scenario: A removed scenario does not suppress an unrelated later @wip scenario**
  - RED: Remove `@wip` from this scenario only. Write the Given step
    `a feature file with a removed scenario and a separate wip scenario at
    "<path>"`. Run verify → must FAIL (code exits 0 due to over-broad `<=`,
    scenario expects exit 2).
  - GREEN: Fix `src/cli/lint.rs` line 61: change `<=` to `==`. Run verify →
    must PASS (the unrelated @wip is no longer suppressed).
  - REFACTOR: no seam needed.
