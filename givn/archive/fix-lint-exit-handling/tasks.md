# Tasks — fix-lint-exit-handling

- [x] **Make the delta spec discoverable by the test runner**
  - Create file-level symlink.

- [x] **Scenario: The lint exit-code contract is preserved after refactor**
  - RED: Remove `@wip`. All steps reused. Run verify → should PASS
    immediately (existing exit codes are correct). This is a regression
    guard — it passes before AND after the refactor.
  - GREEN: Refactor `src/cli/lint.rs` to return `Result<i32>` and
    `src/main.rs` to map the code. Run verify → must still PASS (all
    existing scenarios + the new one).
  - REFACTOR: no seam needed.
