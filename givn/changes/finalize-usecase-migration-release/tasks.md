# Tasks: finalize-usecase-migration-release

## Domain Constraints

| Scenario / capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| migration coverage enforcement | Behavior, E2E, interaction, line, and branch coverage cannot regress without explicit retirement evidence. | none |
| use-case ownership correction | Every active capability has one explicit use-case owner; fragments remain distinct from use cases. | none |
| release bundle finalization | The migration target and semantic operations remain explicit; Handoff and historical archives are preserved. | none |

## Setup

- [x] Configure and prove strict Cucumber execution with `.fail_on_skipped()`;
      targeted command
      `GIVN_FEATURES=givn/changes/finalize-usecase-migration-release/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "Missing behavior blocks migration coverage"`
      failed non-zero with `not implemented: migration fixture setup`.
- [x] Create separate unit and E2E step-definition modules named by design;
      `tests/steps/workflow_enforcement_steps.rs` and
      `tests/steps/workflow_enforcement_e2e_steps.rs` are registered, and the
      skeleton fails through `unimplemented!()` rather than an empty body.

## E2E Setup

- [x] No external service or digital twin is required; the real CLI subprocess
      runs in a temporary initialized project.
- [x] `GIVN_FEATURES=givn/changes/finalize-usecase-migration-release/specs ./run-tests.sh --e2e`
      passed with `1 feature / 1 scenario (1 passed)`.

## Scenario: A consumer runs migration coverage and sees a passing receipt

- [x] RED: strict-mode setup established that an unimplemented migration
      fixture step fails non-zero before implementation.
- [x] GREEN: targeted CLI run passed with `1 feature / 1 scenario (1 passed)`
      and 5 steps passed; it observed the passing migration receipt. Files:
      `tests/steps/workflow_enforcement_steps.rs` and the migration delta.
- [x] REFACTOR: targeted rerun passed with `1 feature / 1 scenario (1 passed)`
      and 5 steps passed.
- [x] COMMIT: pending.

## Scenario: Missing behavior blocks migration coverage

- [x] RED: targeted runner failed with unimplemented fixture steps; output was
      `1 feature / 1 scenario (1 failed)`, including
      `not implemented: migration fixture setup`.
- [x] GREEN: the real migration CLI returned non-zero and wrote a failed
      comparison receipt; targeted run passed with `1 feature / 1 scenario (1
      passed)`, 5 steps passed. Files:
      `tests/steps/workflow_enforcement_steps.rs` and
      `tests/steps/workflow_enforcement_e2e_steps.rs`.
- [x] REFACTOR: targeted rerun passed with `1 feature / 1 scenario (1 passed)`
      and 5 steps passed.
- [x] COMMIT: `a653312` — feat(givn-workflow-enforcement): missing behavior
      blocks migration coverage.

## Scenario: Lost E2E evidence blocks migration coverage

- [x] RED: strict-mode setup established that an unimplemented migration
      fixture step fails non-zero before implementation.
- [x] GREEN: targeted CLI run passed with `1 feature / 1 scenario (1 passed)`
      and 4 steps passed; it observed the lost-E2E regression as non-zero.
      Added focused source coverage assertion in `src/spec/coverage.rs`.
- [x] REFACTOR: `cargo test --lib coverage::tests::lost_e2e_evidence_blocks_without_missing_behavior`
      passed with 1 test passed.
- [x] COMMIT: pending.

## Scenario: Lower interaction coverage blocks migration coverage

- [x] RED: strict-mode setup established that an unimplemented migration
      fixture step fails non-zero before implementation.
- [x] GREEN: targeted CLI run passed with `1 feature / 1 scenario (1 passed)`
      and 4 steps passed; it observed lower interaction coverage as non-zero.
      Added focused source coverage assertion in `src/spec/coverage.rs`.
- [x] REFACTOR: `cargo test --lib coverage::tests::lower_interaction_coverage_blocks_when_behavior_remains`
      passed with 1 test passed.
- [x] COMMIT: pending.

## Scenario: Lower line coverage blocks migration coverage

- [x] RED: strict-mode setup established that an unimplemented migration
      fixture step fails non-zero before implementation.
- [x] GREEN: targeted CLI run passed with `1 feature / 1 scenario (1 passed)`
      and 4 steps passed; it observed lower line coverage as non-zero. Added
      focused source coverage assertion in `src/spec/coverage.rs`.
- [x] REFACTOR: `cargo test --lib coverage::tests::lower_line_coverage_blocks`
      passed with 1 test passed.
- [x] COMMIT: pending.

## Scenario: Lower branch coverage blocks migration coverage

- [x] RED: strict-mode setup established that an unimplemented migration
      fixture step fails non-zero before implementation.
- [x] GREEN: targeted CLI run passed with `1 feature / 1 scenario (1 passed)`
      and 4 steps passed; it observed lower branch coverage as non-zero. Added
      focused source coverage assertion in `src/spec/coverage.rs`.
- [x] REFACTOR: `cargo test --lib coverage::tests::lower_branch_coverage_blocks`
      passed with 1 test passed.
- [x] COMMIT: pending.

## Scenario: Consumer runs migration coverage check and sees passed evidence

- [x] RED: strict-mode setup established that an unimplemented E2E step fails
      non-zero before implementation.
- [x] GREEN: the scoped E2E runner passed with `1 feature / 1 scenario (1
      passed)` and 4 steps through the real CLI subprocess.
- [x] REFACTOR: the corrected scoped E2E rerun passed with the same result.
- [x] COMMIT: `4c42731` — feat(givn-workflow-enforcement): consumer sees
      migration coverage receipt.

## Scenario: Explicit retirement documents a changed behavior

- [x] RED: strict-mode setup established that an unimplemented migration
      fixture step fails non-zero before implementation.
- [x] GREEN: targeted CLI run passed with `1 feature / 1 scenario (1 passed)`
      and 4 steps; explicit `RETIRE` ledger evidence allowed the documented
      behavior removal. Files: `src/spec/coverage.rs`, migration steps, and
      the delta feature.
- [x] REFACTOR: `cargo test --lib coverage::tests::explicit_retirement_allows_missing_behavior`
      passed with 1 test passed.
- [x] COMMIT: pending.

## Release closure

- [ ] Finalize the `0.5.0` to `0.6.0` migration bundle and catalog.
- [ ] Record complete active capability ownership in use-case documents.
- [ ] Update the release plan with actual consolidated workstream history and
      remaining release boundaries.
- [ ] Run full and E2E verification for Givn, Watn, and Kiekste; output:
      pending.
