# Tasks: repository-wide-ownership-e2e

## Setup

- [x] Confirmed `tests/features_runner.rs` uses `.fail_on_skipped()` and
  forwards `GIVN_FEATURES` to the Gherkin runner. The proof command is
  `GIVN_FEATURES=givn/changes/repository-wide-ownership-e2e/specs cargo test --test features_runner -- --tags @e2e`.
- [x] Added and registered the one capability step file
  `tests/steps/repository_ownership_fixtures_steps.rs` plus the shared helper
  file `tests/steps/repository_ownership_fixtures_support.rs`. New step bodies
  start with `unimplemented!()`.
- [x] Proved strictness with the active delta using the command above. It
  exited non-zero with `3 scenarios (3 failed)`, each failure reporting
  `Step panicked. Captured output: not implemented`, followed by
  `3 steps failed`.
- [x] Confirmed no external infrastructure is required; the TempDir-only
  fixture has
  no server, database, provider, network, or digital twin dependency.
- [x] Prove the active scope count with the unfiltered command and the E2E
  projection. The unfiltered command reported `1 feature`, `3 scenarios (3
  passed)`, and `16 steps (16 passed)`. The `--tags @e2e` projection reported
  the same `1 feature`, `3 scenarios (3 passed)`, and `16 steps (16 passed)`;
  all three active scenarios are intentionally `@e2e`, so the equal-count
  exception is explicit. The `--tags 'not @e2e'` projection reported `0
  features`, `0 scenarios`, and `0 steps`.

## Scenario: Repository-wide review accepts explicit overlap dispositions

- [x] RED: Remove `@wip` from this scenario only, implement pending step stubs,
  and run
  `GIVN_FEATURES=givn/changes/repository-wide-ownership-e2e/specs cargo test --test features_runner -- --name "Repository-wide review accepts explicit overlap dispositions"`.
  Recorded non-zero output: `1 scenario (1 failed)`, `1 step (1 failed)`,
  with `Step panicked. Captured output: not implemented` and `1 step failed`.
- [x] GREEN: Create the isolated two-capability fixture, write its exact
  disposition table, invoke the real `givn check review --change
  fixture-consolidation` subprocess, and assert exit status plus exact stdout.
  Targeted run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
  Implementation files: `tests/steps/repository_ownership_fixtures_steps.rs`,
  `tests/steps/repository_ownership_fixtures_support.rs`, and the
  `GivnWorld` field in `tests/steps/mod.rs`.
- [x] REFACTOR: Removed the unused support import and formatted the new step
  modules without changing behavior. The named rerun passed with `1 scenario
  (1 passed)` and `5 steps (5 passed)`.
- [x] COMMIT: `test(e2e): Repository-wide review accepts explicit overlap dispositions`
  Hash: `4369d77`.

## Scenario: Archive publishes consolidated permanent specifications

- [x] RED: Remove `@wip` from this scenario only and run
  `GIVN_FEATURES=givn/changes/repository-wide-ownership-e2e/specs cargo test --test features_runner -- --name "Archive publishes consolidated permanent specifications"`.
  RED output: `1 scenario (1 failed)`, `2 steps (1 passed, 1 failed)`, with
  `Step panicked. Captured output: not implemented` for the archive command.
- [x] GREEN: Invoke the real `givn archive --change fixture-consolidation`
  subprocess in a fresh fixture and assert CLI output first, then canonical
  title presence, obsolete title absence, and repository-wide duplicate-title
  absence. The implementation uses
  `tests/steps/repository_ownership_fixtures_steps.rs` and
  `tests/steps/repository_ownership_fixtures_support.rs`; the targeted run
  passed with `1 scenario (1 passed)` and `7 steps (7 passed)`.
- [x] REFACTOR: Reused the shared `invoke_archive` helper for successful and
  failing archive steps and kept duplicate-title/tree assertions in the shared
  helper. After `cargo fmt --all`, the named rerun passed with `1 scenario (1
  passed)` and `7 steps (7 passed)`.
- [x] COMMIT: `test(e2e): Archive publishes consolidated permanent specifications`
  Hash: `5bbeadb`.

## Scenario: Pre-merge archive failure preserves the permanent specification tree

- [ ] RED: Remove `@wip` from this scenario only and run
  `GIVN_FEATURES=givn/changes/repository-wide-ownership-e2e/specs cargo test --test features_runner -- --name "Pre-merge archive failure preserves the permanent specification tree"`.
  Record the required non-zero failure output.
- [ ] GREEN: Create the failing fixture variant with `verify.command: "false"`,
  invoke the real archive subprocess, assert non-zero CLI status, and compare
  the complete permanent tree byte-for-byte with its pre-command snapshot.
  Record the implementation files changed.
- [ ] REFACTOR: Re-run the named failure scenario after consolidating snapshot
  helpers and record the passing expected-failure output.
- [ ] COMMIT: `test(e2e): Pre-merge archive failure preserves the permanent specification tree`
  Hash: `________________`.

## Final Gate Evidence

- [ ] `givn lint --change repository-wide-ownership-e2e` is clean after all
  `@wip` tags are removed.
- [ ] `./run-tests.sh` passes the permanent non-E2E suite.
- [ ] `./run-tests.sh --e2e` passes the permanent E2E suite.
- [ ] The active delta's unfiltered, non-E2E, and E2E commands pass with the
  recorded all-E2E equal-count exception.
- [ ] `./measure-coverage.sh` and `./merge-coverages.sh` produce fresh reports;
  the E2E and merged Cobertura files show non-zero `src/cli/check.rs` and
  `src/cli/archive.rs` line coverage.
- [ ] `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings`
  pass.
- [ ] Run `givn check review --change repository-wide-ownership-e2e` and record
  verify, verify-e2e, integrity, overlap, and net-delta output.
- [ ] Write `review.md` with the fabrication audit, interaction matrix,
  Arc42 conformance, coverage evidence, and `REVIEW: PASS`.
- [ ] Run `givn archive --change repository-wide-ownership-e2e` and record the
  post-merge permanent-tree, E2E, and coverage gates.
