# Tasks — fix-lint-severity-ranking

Implementation tasks for fixing the severity-ranking inversion in
`givn lint`. One scenario to implement (a `@givn.added` scenario replacing
the old `@givn.removed` characterization scenario).

## Setup

- [x] **Make the delta spec discoverable by the test runner**
  - Create a file-level symlink: `givn/specs/lint-command/fix-lint-severity-ranking.feature`
    → `../../changes/fix-lint-severity-ranking/specs/lint-command/lint-command.feature`
    (the runner globs `givn/specs/**/*.feature`; a file-level symlink is
    required because cucumber-rs 0.23 ignores symlinked directories).
  - Run `cargo test --test features_runner` → the new scenario should be RED
    (it asserts exit 1 but the code currently exits 2). The existing suite
    should still pass.

## Characterization cycles

- [x] **Scenario: A parse error outranks a @wip finding**
  - RED: Remove `@wip` from this scenario only. Run `cargo test --test
    features_runner` → must FAIL (asserts exit 1, but code exits 2). All
    steps are reused — no new step definitions needed.
  - GREEN: Fix the severity logic in `src/cli/lint.rs`. Replace the
    `max_severity: u8` counter with `has_parse_error: bool` and
    `has_wip: bool`. At the end of `run()`, determine the exit code with
    explicit priority: if `has_parse_error` → exit 1; else if `has_wip` →
    exit 2; else exit 0. Run `cargo test --test features_runner` → this
    scenario passes; all other scenarios still pass.
  - REFACTOR: no seam needed.
