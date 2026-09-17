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

- [ ] Confirm strict mode and the runner: `tests/features_runner.rs` uses
      `.fail_on_skipped()`; `verify.command` is `./run-tests.sh` and
      `verify.e2e_command` is `./run-tests.sh --e2e`. No new runner is needed.
- [ ] Proof-of-strictness: write a temporary feature file with one undefined
      step, run it, confirm non-zero exit, then delete the probe. Paste the
      command and output here:
      ```
      ```
- [ ] Create the step skeletons in
      `tests/steps/adr_qualification_gate_steps.rs` with `unimplemented!()`
      bodies; `cargo check --locked --tests` succeeds. Paste the output:
      ```
      ```
- [ ] COMMIT: `<hash>` -- `test(adr-qualification-hardening): strict-mode proof and step skeletons`

## Scenarios

### adr-qualification-gate

- [ ] Scenario: `The guidance carries the three falsification tests`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the falsification tests block (three tests, non-examples,
        pass case, kind-not-count, no cap) to
        `assets/addons/arc42-docs/instructions.md`. Files:
        `assets/addons/arc42-docs/instructions.md`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(adr-qualification-gate): The guidance carries the three falsification tests`

- [ ] Scenario: `The design template keeps decision justification in the design`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the Technology Decisions prompt to
        `assets/templates/design.md`. Files: `assets/templates/design.md`.
        `cargo clean -p givn`, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(adr-qualification-gate): The design template keeps decision justification in the design`

- [ ] Scenario: `The impact assessment records the verdict before drafting`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the blank verdict placeholders and the placement rule to
        `assets/addons/arc42-docs/impact-assessment-template.md`; updated the
        stale template description in
        `assets/addons/arc42-docs/instructions.md`. Files: those two assets.
        `cargo clean -p givn`, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(adr-qualification-gate): The impact assessment records the verdict before drafting`

- [ ] Scenario: `The ADR template ships with no pre-passed qualification values`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: blanked the verdict fields in
        `assets/addons/arc42-docs/chapters/adr/adr-template.md` and added the
        fill-from-the-record instruction. Files:
        `assets/addons/arc42-docs/chapters/adr/adr-template.md`. `cargo clean
        -p givn`, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(adr-qualification-gate): The ADR template ships with no pre-passed qualification values`

- [ ] Scenario: `Design-review requires the cheaper-home challenge for a new ADR`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the challenge to
        `assets/instructions/design-review.md`, the matching branch in
        `assets/commands/givn-design-review.md`, and the pointer in
        `assets/skills/givn-design-review/SKILL.md.tmpl`; ran
        `givn skills sync` so the generated copies carry it. Files: those
        three assets plus `.opencode`, `.agents`, `.claude`. `cargo clean -p
        givn`, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(adr-qualification-gate): Design-review requires the cheaper-home challenge for a new ADR`

- [ ] Scenario: `The chapter-selection guidance does not normalize row 9`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: reworded the row-9 paragraph in
        `assets/addons/arc42-docs/instructions.md`. Files:
        `assets/addons/arc42-docs/instructions.md`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(adr-qualification-gate): The chapter-selection guidance does not normalize row 9`

## Final checks

- [ ] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      ```
- [ ] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      ```
- [ ] Run the change delta suite
      (`GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --locked --test features_runner`)
      → zero exit. Evidence:
      ```
      ```
- [ ] Grep every touched step-definition file for empty/no-op bodies → 0.
      Evidence:
      ```
      ```
- [ ] Confirm commit count matches scenario task count and every task has a
      recorded hash.
- [ ] Confirm `givn lint --change adr-qualification-hardening` has no parse
      errors (exit 0 or 2).
