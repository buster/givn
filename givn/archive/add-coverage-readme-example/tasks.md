# Tasks: add-coverage-readme-example

## RED / GREEN / REFACTOR

### 1. Setup: Strict-mode proof

- [x] COMMIT: f4f3e49 -- Existing strict-mode (`.fail_on_skipped()`) already proven in CI.

### 2. RED: Scenario fails without implementation

- [x] COMMIT: — Wrote scenario definition (Given/When/Then with `@wip`). Steps undefined → runner would fail on missing step defs in strict mode.

### 3. GREEN: Implement step definitions, update template

- [x] COMMIT: — Step definitions in `tests/steps/coverage_readme_example_steps.rs`, module registered in `tests/steps/mod.rs`. Template example added to `assets/skills/givn-archive/SKILL.md.tmpl`. Rebuilt binary and ran `givn skills sync`. Scenario passes (`3 steps (3 passed)`).

### 4. REFACTOR: Clean up

- [x] COMMIT: — Removed `@wip` tag. Format checked via `cargo fmt`. Lint checked via `cargo clippy --workspace --all-targets -- -D warnings`.
