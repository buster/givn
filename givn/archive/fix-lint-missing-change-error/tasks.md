# Tasks — fix-lint-missing-change-error

- [x] **Make the delta spec discoverable by the test runner**
  - Create file-level symlink.

- [x] **Scenario: A non-existent change id is not surfaced**
  - RED: Remove `@wip`. Run verify → must FAIL (code exits 0, scenario
    expects exit 1).
  - GREEN: Add change-dir existence check in `src/cli/lint.rs` `--change`
    branch. Return `anyhow::bail!("change '{id}' not found")` if the dir
    doesn't exist. Run verify → must PASS.
  - REFACTOR: no seam needed.
