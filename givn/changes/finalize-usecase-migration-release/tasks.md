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

## Scenario: A consumer runs migration coverage and sees a passing receipt

- [ ] RED: targeted runner fails with unimplemented migration fixture steps;
      output: pending.
- [ ] GREEN: invoke the real migration CLI and assert the passing receipt;
      files: pending.
- [ ] REFACTOR: targeted runner remains green; output: pending.
- [ ] COMMIT: pending.

## Scenario: Missing behavior blocks migration coverage

- [x] RED: targeted runner failed with unimplemented fixture steps; output was
      `1 feature / 1 scenario (1 failed)`, including
      `not implemented: migration fixture setup`.
- [ ] GREEN: assert non-zero exit and failed comparison receipt; files:
      pending.
- [ ] REFACTOR: targeted runner remains green; output: pending.
- [ ] COMMIT: pending.

## Scenario: Lost E2E evidence blocks migration coverage

- [ ] RED: targeted runner fails with unimplemented fixture steps; output:
      pending.
- [ ] GREEN: assert non-zero exit from lost E2E evidence; files: pending.
- [ ] REFACTOR: targeted runner remains green; output: pending.
- [ ] COMMIT: pending.

## Scenario: Lower interaction coverage blocks migration coverage

- [ ] RED: targeted runner fails with unimplemented fixture steps; output:
      pending.
- [ ] GREEN: assert non-zero exit from lower interaction coverage; files:
      pending.
- [ ] REFACTOR: targeted runner remains green; output: pending.
- [ ] COMMIT: pending.

## Scenario: Lower line coverage blocks migration coverage

- [ ] RED: targeted runner fails with unimplemented fixture steps; output:
      pending.
- [ ] GREEN: assert non-zero exit from lower line coverage; files: pending.
- [ ] REFACTOR: targeted runner remains green; output: pending.
- [ ] COMMIT: pending.

## Scenario: Lower branch coverage blocks migration coverage

- [ ] RED: targeted runner fails with unimplemented fixture steps; output:
      pending.
- [ ] GREEN: assert non-zero exit from lower branch coverage; files: pending.
- [ ] REFACTOR: targeted runner remains green; output: pending.
- [ ] COMMIT: pending.

## Release closure

- [ ] Finalize the `0.5.0` to `0.6.0` migration bundle and catalog.
- [ ] Record complete active capability ownership in use-case documents.
- [ ] Update the release plan with actual consolidated workstream history and
      remaining release boundaries.
- [ ] Run full and E2E verification for Givn, Watn, and Kiekste; output:
      pending.
