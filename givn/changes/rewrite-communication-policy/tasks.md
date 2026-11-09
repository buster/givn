# Tasks: rewrite-communication-policy

## Setup

- [x] Runner and strict-mode proof
  - Design constraints: `tests/features_runner.rs` runs the change's `specs/` via `GIVN_FEATURES` and calls `.fail_on_skipped()`. `verify.command` stays `./run-tests.sh --non-e2e`; no `givn/commands.yaml` edit.
  - [x] Proof-of-strictness: a temporary `probe/strictness.feature` carried one scenario with an undefined step.
    ```
    GIVN_FEATURES=givn/changes/rewrite-communication-policy/specs/probe cargo test --locked --test features_runner
     ✘  Given a step binding that does not exist
        Step doesn't match any function
    1 scenario (1 failed), 1 step (1 failed); error: test failed
    ```
    The probe was removed; the change suite then ran 7/7.
  - [x] Step-definition home: `tests/steps/default_all_addons_steps.rs` (capability `enable-optional-addons`); the five new Then bindings and the block-extraction helper live there.

## Batch: communication-block

- [x] Scenario: The communication block binds documentation to named artifacts
  - Design constraints: `assets/addons/communication-policy/policy.md` is the only source; `AGENTS.md` carries the byte-identical rendered block. The documentation section names tables, the spec/`--help` boundary, and the Quick Start boundary; it names no repository documentation file and no line budget; the fenced block stays at or below 300 words. The delta scenario is regular because addon enablement normalizes to the existing end-to-end scenario.
  - [x] Implement the scenario and its tests. Files: `assets/addons/communication-policy/policy.md`, `AGENTS.md`, `src/cli/addons.rs`, `tests/steps/default_all_addons_steps.rs`, `givn/specs/initialize-project/usecase.md` (change-side narrative).
    ```
    GIVN_FEATURES=givn/changes/rewrite-communication-policy/specs cargo test --locked --test features_runner -- --name '^The communication block binds documentation to named artifacts$'
    1 scenario (1 passed)
    7 steps (7 passed)

    GIVN_FEATURES=givn/specs/initialize-project cargo test --locked --test features_runner -- --name '^Communication policy replaces absolute mode without a compatibility alias$'
    1 scenario (1 passed)
    5 steps (5 passed)
    ```
  - [x] REFACTOR: `cargo fmt --all`; both scenarios re-ran and still pass.
  - [x] COMMIT: `f9644a9f` -- Batch: communication-block

## End-to-end smoke

- [x] No new end-to-end scenario. The change is a content invariant of the existing addon-enablement action; the design's Interaction Coverage Matrix records the normalization to `Consumer enables the outline addon and it is listed as enabled`. `./run-tests.sh --e2e` runs at archive.

**Done when:** every box checked with a batch commit, `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e` exit 0 on the full suite.
