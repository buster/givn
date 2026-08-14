# Tasks: prose-consolidation

The canonical E2E policy lives in `assets/instructions/specs.md`. Every
non-canonical asset listed in design.md must delegate to that source. The
capability has three regular regression scenarios and two real-CLI `@e2e`
scenarios. Check each task immediately after its evidence and commit hash are
complete.

## Setup

- [x] **Setup**: prove existing strict mode and create/register
      `tests/steps/prose_consolidation_steps.rs`.
      The runner uses `.fail_on_skipped()` in `tests/features_runner.rs` and
      Rust stubs use `unimplemented!()`.
      ```text
      Command:
      GIVN_FEATURES=tests/strict-mode-prose-probe.feature cargo test --test features_runner -- --name '^Undefined prose step fails the runner$'
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun `--test features_runner`
      ```

## Non-E2E Scenarios

- [x] Scenario: `Project override replaces the canonical specs instruction`
  - [x] RED-equivalent: existing cascade behavior is a legitimate immediate
        GREEN characterization; all steps reuse existing CLI bindings plus the
        new override assertion.
        ```text
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] GREEN: implemented the real subprocess override assertion in
        `tests/steps/prose_consolidation_steps.rs`.
        Files: `tests/steps/prose_consolidation_steps.rs`.
        ```text
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: reran targeted scenario.
        ```text
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `7187b03` -- `Project override replaces the canonical specs instruction`

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
