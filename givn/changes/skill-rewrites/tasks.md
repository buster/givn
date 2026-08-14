# Tasks: skill-rewrites

This change rewrites embedded workflow guidance. The deterministic gates and
retrieval engine are not modified. Existing guidance assertions are reused;
`tests/steps/skill_rewrites_steps.rs` supplies the combined instruction step.

## Setup

- [x] **Setup**: prove strict mode with an undefined-step probe, register
      `skill_rewrites_steps.rs`, and confirm the existing runner's
      `.fail_on_skipped()` mechanism.
      ```text
      GIVN_FEATURES=tests/strict-mode-skill-probe.feature cargo test --test features_runner -- --name '^Undefined skill step fails the runner$'
      1 feature; 1 scenario (1 failed); 1 step (1 failed)
      error: test failed, to rerun `--test features_runner`
      ```

## Non-E2E Scenarios

- [x] Scenario: `Review instructions require deterministic gate dispositions`
  - [x] RED-equivalent: immediate GREEN characterization after instruction
        choreography was added; real CLI output passes.
        ```text
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] GREEN: asserted review instruction gate choreography through the CLI.
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
  - [x] COMMIT: `433cdb8` -- `Review instructions require deterministic gate dispositions`

- [x] Scenario: `Design and step instructions require the Black-Box-First decision`
  - [x] RED-equivalent: immediate GREEN characterization through the real
        design/tasks instruction subprocesses.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] GREEN: asserted both instruction outputs through the real subprocess.
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
  - [x] COMMIT: `20423bd` -- `Design and step instructions require the Black-Box-First decision`

- [x] Scenario: `Spec instructions use deterministic gates before retrieval exists`
  - [x] RED-equivalent: immediate GREEN characterization of the deterministic
        pre-retrieval instruction through the real CLI.
        ```text
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] GREEN: asserted deterministic-gate guidance and no retrieval command.
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
  - [x] COMMIT: `77534f5` -- `Spec instructions use deterministic gates before retrieval exists`

## E2E Setup

- [x] **E2E setup**: prove `./run-tests.sh --e2e` and the real CLI subprocess;
      enable all configured skill targets through the fixture.
      ```text
      Full active count: 3 scenarios (3 passed), 13 steps (13 passed)
      E2E active count: 1 scenario (1 passed), 9 steps (9 passed)
      E2E count is strictly smaller than full count.
      ```

## E2E Scenario

- [x] @e2e Scenario: `Synchronized guidance explains how to resolve the deterministic gates`
  - [x] RED-equivalent: immediate GREEN characterization after gate-aware
        guidance was added; real skills-sync subprocess passes.
        ```text
        1 feature
        1 scenario (1 passed)
        9 steps (9 passed)
        ```
  - [x] GREEN: asserted successful skills-sync CLI output, review dispositions,
        split-or-keep, removed+added, Black-Box-First, and no retrieval step.
        ```text
        1 feature
        1 scenario (1 passed)
        9 steps (9 passed)
        ```
  - [x] REFACTOR: reran targeted E2E scenario.
        ```text
        1 feature
        1 scenario (1 passed)
        9 steps (9 passed)
        ```
  - [x] COMMIT: `7e3f470` -- `Synchronized guidance explains how to resolve the deterministic gates`

## Final Gates

- [x] Active non-E2E suite passes: 3 scenarios, 13 steps, all passed.
- [x] Active E2E suite passes: 1 scenario, 9 steps, all passed.
- [x] Full permanent suite and E2E subset pass after archive baseline:
      166 non-E2E scenarios and 24 E2E scenarios, all passed.
- [x] No retrieval instruction appears before Change 5.
      ```text
      pre-retrieval positive-command scan: clean
      ```
