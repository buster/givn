# Tasks: workflow-enforcement-wip-cleanup

This is a metadata-only characterization migration. The existing bindings and
observable behavior are already GREEN. A meaningful production RED phase is
not applicable; each scenario uses `givn lint --change` as the metadata check,
then runs the existing scenario against the active delta. Each scenario gets
one atomic commit containing its tag transition, evidence, and task checkbox.

## Strict-Mode Setup

- [x] **Setup**: prove that `tests/features_runner.rs` uses
      `.fail_on_skipped()`, and run a temporary undefined-step probe that must
      exit non-zero. No new runner or step-definition file is needed; the
      existing capability binding is `tests/steps/workflow_enforcement_steps.rs`.
      Stub convention, if ever needed: Rust `unimplemented!()`.
      ```text
      Command:
      GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --test features_runner -- --name '^Undefined steps fail the runner$'

      Output:
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      thread 'main' panicked ... cucumber-0.23.0 ... 1 step failed
      error: test failed, to rerun `--test features_runner`
      ```

## Scenarios

- [x] Scenario: `Integrity hook passes when tasks.md commit hashes are present`
  - [x] RED-equivalent: `givn lint --change workflow-enforcement-wip-cleanup`
        reported 16 WIP findings, including this scenario; existing behavior is
        already implemented.
        ```text
        [WIP] scenario 'Integrity hook passes when tasks.md commit hashes are present' is tagged @wip
        givn lint: 1 file(s) checked, 16 finding(s)
        ```
  - [x] GREEN: removed `@wip` from this scenario only and ran the anchored
        active-delta command from design.md.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: reran the anchored command; no behavior change.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `36dccd7` -- `Integrity hook passes when tasks.md commit hashes are present`

- [x] Scenario: `Integrity hook fails on placeholder commit hash`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Integrity hook fails on placeholder commit hash' is tagged @wip
        givn lint: 1 file(s) checked, 15 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `45e3ec0` -- `Integrity hook fails on placeholder commit hash`

- [x] Scenario: `Integrity hook fails on missing step-definition file`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Integrity hook fails on missing step-definition file' is tagged @wip
        givn lint: 1 file(s) checked, 14 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] COMMIT: `02737df` -- `Integrity hook fails on missing step-definition file`

- [x] Scenario: `Integrity hook fails on empty step-definition file`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Integrity hook fails on empty step-definition file' is tagged @wip
        givn lint: 1 file(s) checked, 13 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] COMMIT: `6c2e450` -- `Integrity hook fails on empty step-definition file`

- [x] Scenario: `Integrity hook fails on Step Definitions section that is not a table`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Integrity hook fails on Step Definitions section that is not a table' is tagged @wip
        givn lint: 1 file(s) checked, 12 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `5e6b8a3` -- `Integrity hook fails on Step Definitions section that is not a table`

- [x] Scenario: `Integrity hook passes when arc42 is disabled`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Integrity hook passes when arc42 is disabled' is tagged @wip
        givn lint: 1 file(s) checked, 11 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `5348c61` -- `Integrity hook passes when arc42 is disabled`

- [x] Scenario: `Integrity hook fails on unedited arc42 chapter`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Integrity hook fails on unedited arc42 chapter' is tagged @wip
        givn lint: 1 file(s) checked, 10 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        9 steps (9 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        9 steps (9 passed)
        ```
  - [x] COMMIT: `11556c7` -- `Integrity hook fails on unedited arc42 chapter`

- [x] Scenario: `Integrity hook passes on edited arc42 chapter`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Integrity hook passes on edited arc42 chapter' is tagged @wip
        givn lint: 1 file(s) checked, 9 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] COMMIT: `2eb6888` -- `Integrity hook passes on edited arc42 chapter`

- [x] Scenario: `Integrity hook skips arc42 check when slug cannot be resolved`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Integrity hook skips arc42 check when slug cannot be resolved' is tagged @wip
        givn lint: 1 file(s) checked, 8 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] COMMIT: `cdabeeb` -- `Integrity hook skips arc42 check when slug cannot be resolved`

- [x] Scenario: `Archive re-runs verify against merged permanent specs and passes`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Archive re-runs verify against merged permanent specs and passes' is tagged @wip
        givn lint: 1 file(s) checked, 7 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] COMMIT: `aa89a3d` -- `Archive re-runs verify against merged permanent specs and passes`

- [x] Scenario: `Archive blocked when merged permanent specs fail verify`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'Archive blocked when merged permanent specs fail verify' is tagged @wip
        givn lint: 1 file(s) checked, 6 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        11 steps (11 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        11 steps (11 passed)
        ```
  - [x] COMMIT: `f7e3dbc` -- `Archive blocked when merged permanent specs fail verify`

- [x] Scenario: `New-capability merge failure produces clear recovery instructions`
  - [x] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        [WIP] scenario 'New-capability merge failure produces clear recovery instructions' is tagged @wip
        givn lint: 1 file(s) checked, 5 finding(s)
        ```
  - [x] GREEN: removed this scenario's `@wip`; ran its anchored active-delta command.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: reran the anchored command.
        ```text
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `4816e01` -- `New-capability merge failure produces clear recovery instructions`

- [ ] Scenario: `Design.md template includes Internal Primitives section`
  - [ ] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        <paste lint output>
        ```
  - [ ] GREEN: remove this scenario's `@wip`; run its anchored active-delta command.
        ```text
        <paste runner output>
        ```
  - [ ] REFACTOR: rerun the anchored command.
        ```text
        <paste runner output>
        ```
  - [ ] COMMIT: `<hash>` -- `Design.md template includes Internal Primitives section`

- [ ] Scenario: `Archive blocked when delta spec has non-removed @wip scenario`
  - [ ] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        <paste lint output>
        ```
  - [ ] GREEN: remove this scenario's `@wip`; run its anchored active-delta command.
        ```text
        <paste runner output>
        ```
  - [ ] REFACTOR: rerun the anchored command.
        ```text
        <paste runner output>
        ```
  - [ ] COMMIT: `<hash>` -- `Archive blocked when delta spec has non-removed @wip scenario`

- [ ] Scenario: `Archive allowed when @wip only on a @givn.removed placeholder`
  - [ ] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        <paste lint output>
        ```
  - [ ] GREEN: remove this scenario's `@wip`; run its anchored active-delta command.
        ```text
        <paste runner output>
        ```
  - [ ] REFACTOR: rerun the anchored command.
        ```text
        <paste runner output>
        ```
  - [ ] COMMIT: `<hash>` -- `Archive allowed when @wip only on a @givn.removed placeholder`

- [ ] Scenario: `Check tasks blocked when delta spec has @wip`
  - [ ] RED-equivalent: lint output with this scenario still tagged `@wip`.
        ```text
        <paste lint output>
        ```
  - [ ] GREEN: remove this scenario's `@wip`; run its anchored active-delta command.
        ```text
        <paste runner output>
        ```
  - [ ] REFACTOR: rerun the anchored command.
        ```text
        <paste runner output>
        ```
  - [ ] COMMIT: `<hash>` -- `Check tasks blocked when delta spec has @wip`

## Final Gates

- [ ] Full active-delta suite passes with `GIVN_FEATURES` set to this change.
- [ ] Full permanent non-E2E suite passes with `./run-tests.sh`.
- [ ] Existing E2E subset passes with `./run-tests.sh --e2e`.
- [ ] Post-archive metadata checks prove no scenario-level `@wip` remains and
      fixture literals remain unchanged.
