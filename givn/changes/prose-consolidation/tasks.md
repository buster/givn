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

- [x] Scenario: `Canonical policy classifies output-affecting flags as variants`
  - [x] RED-equivalent: immediate GREEN characterization through existing CLI
        subprocess steps; the changed canonical instruction is the observable
        asset under test.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] GREEN: asserted the canonical variant wording through the real CLI.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] REFACTOR: reran targeted scenario.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] COMMIT: `5e38ef1` -- `Canonical policy classifies output-affecting flags as variants`

- [x] Scenario: `Design and review instructions defer to the canonical policy`
  - [x] RED-equivalent: immediate GREEN characterization after the canonical
        asset update; the real subprocess and combined-output assertions pass.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] GREEN: ran both instruction commands through the real subprocess and
        assert canonical delegation.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] REFACTOR: reran targeted scenario.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] COMMIT: `80b6575` -- `Design and review instructions defer to the canonical policy`

## E2E Setup

- [x] **E2E setup**: configure/prove `./run-tests.sh --e2e`, confirm the real
      CLI subprocess harness, and enable all three skill targets for the
      generated-guidance scenario.
      ```text
      Full active-delta count: 3 scenarios (3 passed), 13 steps (13 passed)
      E2E active-delta count: 2 scenarios (2 passed), 17 steps (17 passed)
      E2E count is strictly smaller than full count.
      ```

## E2E Scenarios

- [x] @e2e Scenario: `Canonical E2E policy is emitted by specs instructions`
  - [x] RED-equivalent: existing CLI bindings provide an immediate GREEN
        characterization of the changed canonical instruction.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] GREEN: asserted canonical output and absence of the actual legacy wording
        through the real `givn instructions` subprocess.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] REFACTOR: reran the E2E scenario.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] COMMIT: `cf63571` -- `Canonical E2E policy is emitted by specs instructions`

- [x] @e2e Scenario: `Generated guidance points to the canonical E2E policy`
  - [x] RED-equivalent: immediate GREEN characterization after the delegated
        asset guidance was implemented; real skills-sync subprocess passes.
        ```text
        1 feature
        1 scenario (1 passed)
        13 steps (13 passed)
        ```
  - [x] GREEN: asserted successful `givn skills sync` CLI output, all configured
        skill targets, the opencode command, canonical delegation, and absence
        of legacy policy wording.
        ```text
        1 feature
        1 scenario (1 passed)
        13 steps (13 passed)
        ```
  - [x] REFACTOR: reran the E2E scenario.
        ```text
        1 feature
        1 scenario (1 passed)
        13 steps (13 passed)
        ```
  - [x] COMMIT: `ad5a962` -- `Generated guidance points to the canonical E2E policy`

## Final Gates

- [ ] Active non-E2E scenarios pass.
- [ ] Active E2E scenarios pass.
- [ ] Full permanent suite and E2E subset pass after all assets are merged.
- [ ] Allowlist parity scan finds no legacy wording and all non-canonical
      assets delegate to `givn instructions specs`.
