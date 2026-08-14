# Tasks: prose-consolidation

The canonical E2E policy lives in `assets/instructions/specs.md`. Every
non-canonical asset listed in design.md must delegate to that source. The
capability has three regular regression scenarios and two real-CLI `@e2e`
scenarios. Check each task immediately after its evidence and commit hash are
complete.

## Setup

- [ ] **Setup**: prove existing strict mode and create/register
      `tests/steps/prose_consolidation_steps.rs`.
      The runner uses `.fail_on_skipped()` in `tests/features_runner.rs` and
      Rust stubs use `unimplemented!()`.
      ```text
      Command:
      <paste undefined-step probe command and non-zero output>
      ```

## Non-E2E Scenarios

- [ ] Scenario: `Project override replaces the canonical specs instruction`
  - [ ] RED: remove `@wip`; target the scenario with the design.md command.
        ```text
        <paste non-zero output>
        ```
  - [ ] GREEN: implement real subprocess assertions for the override variant.
        Files: `tests/steps/prose_consolidation_steps.rs`.
        ```text
        <paste zero-exit output>
        ```
  - [ ] REFACTOR: rerun targeted scenario.
        ```text
        <paste zero-exit output>
        ```
  - [ ] COMMIT: `<hash>` -- `Project override replaces the canonical specs instruction`

- [ ] Scenario: `Canonical policy classifies output-affecting flags as variants`
  - [ ] RED: remove `@wip`; target the scenario with the design.md command.
        ```text
        <paste non-zero output>
        ```
  - [ ] GREEN: assert the canonical variant wording through the real CLI.
        ```text
        <paste zero-exit output>
        ```
  - [ ] REFACTOR: rerun targeted scenario.
        ```text
        <paste zero-exit output>
        ```
  - [ ] COMMIT: `<hash>` -- `Canonical policy classifies output-affecting flags as variants`

- [ ] Scenario: `Design and review instructions defer to the canonical policy`
  - [ ] RED: remove `@wip`; target the scenario with the design.md command.
        ```text
        <paste non-zero output>
        ```
  - [ ] GREEN: run both instruction commands through the real subprocess and
        assert canonical delegation.
        ```text
        <paste zero-exit output>
        ```
  - [ ] REFACTOR: rerun targeted scenario.
        ```text
        <paste zero-exit output>
        ```
  - [ ] COMMIT: `<hash>` -- `Design and review instructions defer to the canonical policy`

## E2E Setup

- [ ] **E2E setup**: configure/prove `./run-tests.sh --e2e`, confirm the real
      CLI subprocess harness, and enable all three skill targets for the
      generated-guidance scenario.
      ```text
      Full count: <paste>
      E2E count: <paste; strictly smaller>
      ```

## E2E Scenarios

- [ ] @e2e Scenario: `Canonical E2E policy is emitted by specs instructions`
  - [ ] RED: remove `@wip`; target with `./run-tests.sh --e2e` and the exact
        Cucumber name filter.
        ```text
        <paste non-zero output>
        ```
  - [ ] GREEN: assert canonical output and absence of the actual legacy wording
        through the real `givn instructions` subprocess.
        ```text
        <paste zero-exit output>
        ```
  - [ ] REFACTOR: rerun the E2E scenario.
        ```text
        <paste zero-exit output>
        ```
  - [ ] COMMIT: `<hash>` -- `Canonical E2E policy is emitted by specs instructions`

- [ ] @e2e Scenario: `Generated guidance points to the canonical E2E policy`
  - [ ] RED: remove `@wip`; target with `./run-tests.sh --e2e` and the exact
        Cucumber name filter.
        ```text
        <paste non-zero output>
        ```
  - [ ] GREEN: assert successful `givn skills sync` CLI output, all configured
        skill targets, the opencode command, canonical delegation, and absence
        of legacy policy wording.
        ```text
        <paste zero-exit output>
        ```
  - [ ] REFACTOR: rerun the E2E scenario.
        ```text
        <paste zero-exit output>
        ```
  - [ ] COMMIT: `<hash>` -- `Generated guidance points to the canonical E2E policy`

## Final Gates

- [ ] Active non-E2E scenarios pass.
- [ ] Active E2E scenarios pass.
- [ ] Full permanent suite and E2E subset pass after all assets are merged.
- [ ] Allowlist parity scan finds no legacy wording and all non-canonical
      assets delegate to `givn instructions specs`.
