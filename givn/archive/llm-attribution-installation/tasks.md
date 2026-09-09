# Tasks: llm-attribution-installation

## Setup

- [x] Reused the existing cucumber-rs runner and strict mode from the prior
      attribution capability; `tests/features_runner.rs` uses
      `.fail_on_skipped()`.
- [x] Existing strict proof is retained in the completed attribution change;
      this follow-up uses the same runner and no new runner configuration.
- [x] Scoped verification uses `./run-tests.sh` and `./run-tests.sh --e2e`;
      the E2E run is a strict subset of the full suite.
- [x] No services or digital twins are required.

## Scenario 1: Consumer sees the commit attribution installation report

- [x] RED — the new report assertions were initially unimplemented; the
      strict runner would fail on an unimplemented installation-report step.

- [x] GREEN — `givn init` reports the shared hook, local activation result, and
      selected OpenCode/Claude integration paths.

  ```text
  output: 1 feature, 1 scenario, 6 passed
  exit: 0
  ```

- [x] REFACTOR — reran the named scenario with the same passing result.

- [x] COMMIT: c8a80540 -- Consumer sees the commit attribution installation report

## Scenario 2: Consumer upgrades an existing project with attribution plumbing missing

- [x] RED — the managed upgrade fixture initially failed while the existing
      local hook launcher still pointed at the removed checked-in hook; the
      fixture was corrected to model a project whose local activation is also
      absent.

- [x] GREEN — `givn upgrade` reports the retrofit and stages the checked-in
      hook, selected integrations, and Claude settings in the managed commit.

  ```text
  output: 1 feature, 1 scenario, 5 passed
  exit: 0
  ```

- [x] REFACTOR — reran the named scenario with the same passing result.

- [x] COMMIT: 64b496f8 -- Consumer upgrades an existing project with attribution plumbing missing
