# Tasks: arc42-review-alignment

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

## Strict-mode proof (do first)

- [x] **Proof-of-strictness**: the runner uses `.fail_on_skipped()` in
      `tests/features_runner.rs`. Run the change-scoped suite with the new
      capability step still unimplemented and confirm a non-zero exit.
      ```text
      GIVN_FEATURES=givn/changes/arc42-review-alignment/specs cargo test --test features_runner -- --name "The review template includes the Arc42 conformance block"
      → exit 101; cucumber reported 1 failed step and captured `not implemented: TODO`.
      ```
- [x] **Step skeleton**: `tests/steps/arc42_review_alignment_steps.rs` is the
      one capability-specific step file, registered in `tests/steps/mod.rs`.
      New steps use `unimplemented!("TODO")`, never empty bodies.

## Non-@e2e scenarios

### Arc42-disabled review instructions mark conformance as not applicable

- [x] Scenario: `Arc42-disabled review instructions mark conformance as not applicable`
  - [x] **Design constraints:** Use the existing `givn instructions review`
        cascade. The conditional wording must make the disabled-addon result
        explicitly not applicable; no Arc42 source inference or new CLI state
        is allowed.
  - [x] **RED:** Remove `@wip` from this scenario and run only this scenario.
        The strict runner must fail before the guidance change is present.
        ```text
        GIVN_FEATURES=givn/changes/arc42-review-alignment/specs cargo test --test features_runner -- --name "Arc42-disabled review instructions mark conformance as not applicable"
        → exit 101; stdout lacked the expected `N/A when the arc42 addon is disabled` phrase.
        ```
  - [x] **GREEN:** Add the conditional Arc42 conformance instructions to
        `assets/instructions/review.md`. Files: `assets/instructions/review.md`.
        Run only this scenario; it must pass.
        ```text
        GIVN_FEATURES=givn/changes/arc42-review-alignment/specs cargo test --test features_runner -- --name "Arc42-disabled review instructions mark conformance as not applicable"
        → exit 0; 1 scenario, 5 steps passed.
        ```
  - [x] **REFACTOR:** Review wording and rerun only this scenario; it must
        still pass.
        ```text
        Same targeted command → exit 0; 1 scenario, 5 steps passed.
        ```
  - [x] COMMIT: 8aeccc9 -- Arc42-disabled review instructions mark conformance as not applicable

## E2E Smoke Tests

- [x] Bring up the local environment with the design's change-scoped Cucumber
      command. This is CLI-only: a temporary initialized project, no external
      services, no digital twins, and no network dependency.
      ```text
      GIVN_FEATURES=givn/changes/arc42-review-alignment/specs cargo test --test features_runner
      → initialized temporary projects and completed all CLI actions successfully.
      ```
- [x] Configure and prove the e2e runner with
      `cargo test --test features_runner -- --tags @e2e`. It is distinct from
      `cargo test --test features_runner`; record full-suite and e2e-only
      scenario counts, with e2e strictly smaller.
      ```text
      Full: `cargo test --test features_runner` → 4 scenarios passed.
      E2E: `cargo test --test features_runner -- --tags @e2e` → 3 scenarios passed.
      3 < 4; commands are distinct and the e2e filter is active.
      ```

### Arc42-enabled review instructions require conformance evidence

- [x] @e2e Scenario: `Arc42-enabled review instructions require conformance evidence`
  - [x] **Design constraints:** The real CLI subprocess must emit the review
        procedure covering durable Arc42 chapters, `design.md`, `tasks.md`,
        completed implementation evidence, mismatch handling, and the explicit
        `ARC42 CONFORMANCE: CLEAN` sign-off. Do not add a binary parser.
  - [x] **RED:** Remove `@wip`, use strict step bindings, and run this scenario
        only through the configured e2e command. It must fail before the asset
        update is present.
        ```text
        GIVN_FEATURES=givn/changes/arc42-review-alignment/specs cargo test --test features_runner -- --name "Arc42-enabled review instructions require conformance evidence"
        → exit 101; stdout lacked `ARC42 CONFORMANCE: CLEAN` after the temporary
        pre-implementation wording was used.
        ```
  - [x] **GREEN:** Update `assets/instructions/review.md` and any aligned
        embedded review assets. Files: `assets/instructions/review.md`,
        `assets/templates/review.md`, `assets/skills/givn-review/SKILL.md.tmpl`,
        `assets/commands/givn-review.md`, and affected Arc42 docs. Run this
        scenario only through the real CLI; it must pass.
        ```text
        Same targeted command → exit 0; 1 scenario, 10 steps passed.
        ```
  - [x] **REFACTOR:** Remove duplication or clarify wording without changing
        the contract; rerun this scenario only.
        ```text
        Same targeted command → exit 0; 1 scenario, 10 steps passed.
        ```
  - [x] COMMIT: 9205a4b -- Arc42-enabled review instructions require conformance evidence

### The review template includes the Arc42 conformance block

- [x] @e2e Scenario: `The review template includes the Arc42 conformance block`
  - [x] **Design constraints:** `givn eject review:template` must be the real
        boundary. The step must validate the destination reported by CLI
        stdout, then inspect the ejected file for the conformance table,
        `design.md`, `tasks.md`, and the clean-marker contract.
  - [x] **RED:** Run this scenario alone with the capability step unimplemented;
        the strict e2e runner must fail.
        ```text
        GIVN_FEATURES=givn/changes/arc42-review-alignment/specs cargo test --test features_runner -- --name "The review template includes the Arc42 conformance block"
        → exit 101; captured `not implemented: TODO` from the strict runner.
        ```
  - [x] **GREEN:** Implement real assertions in
        `tests/steps/arc42_review_alignment_steps.rs`, registered in
        `tests/steps/mod.rs`. No production parser or alternate file read.
        Run this scenario alone; it must pass.
        ```text
        Same targeted command → exit 0; 1 scenario, 8 steps passed. The step
        validated the path reported by `givn eject` before reading the file.
        ```
  - [x] **REFACTOR:** Keep the path/output validation and rerun this scenario.
        ```text
        Same targeted command → exit 0; 1 scenario, 8 steps passed.
        ```
  - [x] COMMIT: 2f371fb -- The review template includes the Arc42 conformance block

### Generated review guidance stays aligned after skill sync

- [x] @e2e Scenario: `Generated review guidance stays aligned after skill sync`
  - [x] **Design constraints:** `givn skills sync` must propagate the same
        Arc42 conditional procedure to `.agents`, `.claude`, `.opencode`, and
        the generated slash command. The scenario checks generated output, not
        embedded asset files directly.
  - [x] **RED:** Run this scenario alone with the generated-target assertion
        unimplemented; strict e2e execution must fail.
        ```text
        GIVN_FEATURES=givn/changes/arc42-review-alignment/specs cargo test --test features_runner -- --name "Generated review guidance stays aligned after skill sync"
        → exit 101; captured `not implemented: TODO` from the strict runner.
        ```
  - [x] **GREEN:** Implement the generated-target assertions in
        `tests/steps/arc42_review_alignment_steps.rs` and run `givn skills sync`
        through the scenario's real CLI boundary. It must pass.
        ```text
        Same targeted command → exit 0; 1 scenario, 5 steps passed. All four
        generated agent targets were inspected.
        ```
  - [x] **REFACTOR:** Keep all generated target checks and rerun this scenario.
        ```text
        Same targeted command → exit 0; 1 scenario, 5 steps passed.
        ```
  - [x] COMMIT: b2c4ad6 -- Generated review guidance stays aligned after skill sync

---

**Done when:** every box is checked with evidence and a commit hash, both
`cargo test --test features_runner` and
`cargo test --test features_runner -- --tags @e2e` exit 0, and the e2e count is
strictly smaller than the full-suite count.
