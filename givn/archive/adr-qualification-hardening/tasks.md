# Tasks: adr-qualification-hardening

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- The qualification gate stays authoring guidance; the AND semantics, routing
  values, and verdict schema do not change.
- The tests classify by kind, not count; no numeric ADR cap.
- The cheaper-home challenge must name an existing canonical home (no new
  decision-log artifact).
- Confirmed Personas: `agent--technical-role` (guidance) and
  `givn-maintainer--owner` (rule corpus). Actors in Gherkin stay the author,
  maintainer, and reviewer.
- Every new Then-step asserts at least one phrase unique to the new text; each
  RED task records the failing run against the unedited asset.

Capabilities and step files:

- `adr-qualification-gate` — `tests/steps/adr_qualification_gate_steps.rs`: new Givens for the ADR template and design template, plus the new When/Then steps

No scenario in this change is tagged `@e2e`: the two existing `@e2e` CLI
scenarios cover the same actions, and the new scenarios are guidance-content
variants.

## Setup

- [x] Confirm strict mode and the runner: `tests/features_runner.rs` uses
      `.fail_on_skipped()`; `verify.command` is `./run-tests.sh` and
      `verify.e2e_command` is `./run-tests.sh --e2e`. No new runner is needed.
- [x] Proof-of-strictness: wrote a temporary `tests/strict-mode-probe.feature`
      with one undefined step, ran it, confirmed non-zero exit, then deleted
      the probe. Evidence:
      ```
      GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --locked --test features_runner
      1 scenario (1 failed)
      1 step (1 failed)
      exit status: 1
      ```
- [x] Created the step skeletons in
      `tests/steps/adr_qualification_gate_steps.rs` (27 steps) with
      `unimplemented!()` bodies. Evidence:
      ```
      cargo check --locked --tests
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.53s
      ```
- [x] COMMIT: `61aca78b` -- `test(adr-qualification-hardening): strict-mode proof and step skeletons`

## Scenarios

### adr-qualification-gate

- [x] Scenario: `The guidance carries the three falsification tests`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --locked --test features_runner -- --name "The guidance carries the three falsification tests"
Step panicked. Captured output: not implemented
1 scenario (1 failed)
2 steps (1 passed, 1 failed)
        ```
  - [x] GREEN: added the falsification tests block (three tests, non-examples,
        pass case, kind-not-count, no cap) to
        `assets/addons/arc42-docs/instructions.md`; implemented the eight steps.
        Files: `assets/addons/arc42-docs/instructions.md`,
        `tests/steps/adr_qualification_gate_steps.rs`. `cargo clean -p givn`, then the targeted run → zero.
        Evidence:
        ```
        ✔ When an author reads the falsification tests
        ✔ Then the guidance fails a single-change reversible decision
        ✔ And ... the multi-change constraint, product-behavior exclusion, non-examples
        ✔ And ... still qualifies, kind-not-count, no numeric ADR cap
        1 feature
        1 scenario (1 passed)
        9 steps (9 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        ✔ When an author reads the falsification tests
        ✔ Then the guidance fails a single-change reversible decision
        ✔ And ... the multi-change constraint, product-behavior exclusion, non-examples
        ✔ And ... still qualifies, kind-not-count, no numeric ADR cap
        1 feature
        1 scenario (1 passed)
        9 steps (9 passed)
        ```
  - [x] COMMIT: `f882c945` -- `feat(adr-qualification-gate): The guidance carries the three falsification tests`

- [x] Scenario: `The design template keeps decision justification in the design`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --locked --test features_runner -- --name "The design template keeps decision justification in the design"
Step panicked. Captured output: not implemented
1 scenario (1 failed)
        ```
  - [x] GREEN: added the Technology Decisions prompt to
        `assets/templates/design.md`; implemented the four steps. Files:
        `assets/templates/design.md`,
        `tests/steps/adr_qualification_gate_steps.rs`. `cargo clean -p givn`, then the targeted run → zero.
        Evidence:
        ```
        ✔ Given the design template is available
        ✔ When an author reads the Technology Decisions guidance
        ✔ Then the template says justification belongs in the design
        ✔ And the template forbids ADR candidacy unless the falsification tests pass
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        ✔ Given the design template is available
        ✔ When an author reads the Technology Decisions guidance
        ✔ Then the template says justification belongs in the design
        ✔ And the template forbids ADR candidacy unless the falsification tests pass
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] COMMIT: `64f7fef9` -- `feat(adr-qualification-gate): The design template keeps decision justification in the design`

- [x] Scenario: `The impact assessment records the verdict before drafting`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --locked --test features_runner -- --name "The impact assessment records the verdict before drafting"
Step panicked. Captured output: not implemented
1 scenario (1 failed)
2 steps (1 passed, 1 failed)
        ```
  - [x] GREEN: added the blank verdict placeholders and the placement rule to
        `assets/addons/arc42-docs/impact-assessment-template.md`; updated the
        stale template description in `assets/addons/arc42-docs/instructions.md`;
        implemented the two steps. Files: those two assets plus
        `tests/steps/adr_qualification_gate_steps.rs`. `cargo clean -p givn`, then the targeted run → zero.
        Evidence:
        ```
        ✔ When a maintainer reads the verdict placement rule
        ✔ Then the marker requires the structured verdict before drafting
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        ✔ When a maintainer reads the verdict placement rule
        ✔ Then the marker requires the structured verdict before drafting
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        ```
  - [x] COMMIT: `79f634ca` -- `feat(adr-qualification-gate): The impact assessment records the verdict before drafting`

- [x] Scenario: `The ADR template ships with no pre-passed qualification values`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --locked --test features_runner -- --name "The ADR template ships with no pre-passed qualification values"
Step panicked. Captured output: not implemented
1 scenario (1 failed)
        ```
  - [x] GREEN: blanked the verdict fields in
        `assets/addons/arc42-docs/chapters/adr/adr-template.md` and added the
        fill-from-the-record instruction; implemented the four steps. Files:
        `assets/addons/arc42-docs/chapters/adr/adr-template.md`,
        `tests/steps/adr_qualification_gate_steps.rs`. `cargo clean -p givn`, then the targeted run → zero.
        Evidence:
        ```
        ✔ Given the ADR template is available
        ✔ When an author prepares to copy the template
        ✔ Then the template contains no pre-passed qualification verdict
        ✔ And the template instructs the author to fill the verdict from the record
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        ✔ Given the ADR template is available
        ✔ When an author prepares to copy the template
        ✔ Then the template contains no pre-passed qualification verdict
        ✔ And the template instructs the author to fill the verdict from the record
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] COMMIT: `11e369a2` -- `feat(adr-qualification-gate): The ADR template ships with no pre-passed qualification values`

- [x] Scenario: `Design-review requires the cheaper-home challenge for a new ADR`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --locked --test features_runner -- --name "Design-review requires the cheaper-home challenge for a new ADR"
Step panicked. Captured output: not implemented
1 scenario (1 failed)
        ```
  - [x] GREEN: added the challenge to
        `assets/instructions/design-review.md`, the matching branch in
        `assets/commands/givn-design-review.md`, and the pointer in
        `assets/skills/givn-design-review/SKILL.md.tmpl`; ran `givn skills sync`
        so the generated copies carry it; implemented the five steps. Files:
        those three assets plus `.opencode`, `.agents`, `.claude`, and
        `tests/steps/adr_qualification_gate_steps.rs`. `cargo clean -p givn`, then the targeted run → zero.
        Evidence:
        ```
        ✔ When a reviewer reads the ADR branch
        ✔ Then the instruction requires the cheaper-home challenge for every proposed new ADR
        ✔ And the design-review command requires the cheaper-home challenge
        ✔ And the design-review skill points at the requirement
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        ✔ When a reviewer reads the ADR branch
        ✔ Then the instruction requires the cheaper-home challenge for every proposed new ADR
        ✔ And the design-review command requires the cheaper-home challenge
        ✔ And the design-review skill points at the requirement
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `93dad8b5` -- `feat(adr-qualification-gate): Design-review requires the cheaper-home challenge for a new ADR`

- [x] Scenario: `The chapter-selection guidance does not normalize row 9`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --locked --test features_runner -- --name "The chapter-selection guidance does not normalize row 9"
Step panicked. Captured output: not implemented
1 scenario (1 failed)
2 steps (1 passed, 1 failed)
        ```
  - [x] GREEN: reworded the row-9 paragraph in
        `assets/addons/arc42-docs/instructions.md`; implemented the four steps.
        Files: `assets/addons/arc42-docs/instructions.md`,
        `tests/steps/adr_qualification_gate_steps.rs`. `cargo clean -p givn`, then the targeted run → zero.
        Evidence:
        ```
        ✔ When an author reads the chapter-selection guidance
        ✔ Then the guidance treats correlated rows as a double-check
        ✔ And the guidance says an affected chapter does not by itself qualify an ADR
        ✔ And the guidance does not normalize an ADR per change
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        ✔ When an author reads the chapter-selection guidance
        ✔ Then the guidance treats correlated rows as a double-check
        ✔ And the guidance says an affected chapter does not by itself qualify an ADR
        ✔ And the guidance does not normalize an ADR per change
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `00332ba9` -- `feat(adr-qualification-gate): The chapter-selection guidance does not normalize row 9`

## Final checks

- [x] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      Summary [29.601s] 514 tests run: 514 passed, 0 skipped
      ```
- [x] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      Summary [5.649s] 88 tests run: 88 passed, 426 skipped
      ```
- [x] Run the change delta suite
      (`GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --locked --test features_runner`)
      → zero exit. Evidence:
      ```
      1 feature
      6 scenarios (6 passed)
      30 steps (30 passed)
      ```
- [x] Grep every touched step-definition file for empty/no-op bodies → 0.
      Evidence:
      ```
      grep -c "unimplemented!\|todo!()" tests/steps/adr_qualification_gate_steps.rs
      0
      ```
- [x] Confirm commit count matches scenario task count and every task has a
      recorded hash: one setup commit (`61aca78b`) plus six scenario commits
      (`f882c945`, `64f7fef9`, `79f634ca`, `11e369a2`, `93dad8b5`, `00332ba9`).
- [x] Confirm `givn lint --change adr-qualification-hardening` has no parse
      errors (exit 0). Evidence:
      ```
      givn lint: 1 file(s) checked — clean
      exit 0
      ```
