# Tasks: persona-identity-reuse

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/persona-identity-reuse/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- Preserved behaviors: user confirmation before use, the review duty, the cap
  (at most three segments per stakeholder type, about ten total), advisory
  findings with user arbitration, handoff promotion with no auto-promotion,
  and the actor/persona distinction.
- The lint advisory never changes the exit-code contract.
- ID-only persona matching; givn's own title-style entries warn (accepted).
- Confirmed Personas: `agent--technical-role` and `givn-maintainer--owner`.
- Two scenarios are legitimate immediate-GREEN guards (recorded below).

Capabilities and step files:

- `ideation-skill` — reused generic steps in `tests/steps/prose_consolidation_steps.rs`; new template steps in `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` or a suitable existing file
- `lint-command` — `tests/steps/mod.rs`: three new Givens
- `visual-review-gate` — `tests/steps/visual_review_gate_steps.rs` and the shared design-review Given: the fixture persona file

## Setup

- [ ] Confirm strict mode and the runner. No new runner is needed.
- [ ] Proof-of-strictness: temporary probe feature, non-zero exit, delete.
- [ ] Create the step skeletons with `unimplemented!()` bodies; `cargo check
      --locked --tests` succeeds.
- [ ] COMMIT: `<hash>` -- `test(persona-identity-reuse): strict-mode proof and step skeletons`

## Scenarios

### ideation-skill

- [ ] Scenario: `The ideation guidance carries the persona identity criteria and the reuse protocol`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the identity criteria, the reuse protocol, the header line, and the schema reference to `assets/skills/givn-ideation/SKILL.md.tmpl`; added the guardrails to `assets/commands/givn-ideation.md`. Files: those two assets. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(ideation-skill): The ideation guidance carries the persona identity criteria and the reuse protocol`

### ideation-skill

- [ ] Scenario: `The ideation guidance keeps the confirmed persona behaviors`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: GUARD (immediate GREEN): all steps reuse the existing generic steps; the assertions pin the preserved behaviors. Files: none. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(ideation-skill): The ideation guidance keeps the confirmed persona behaviors`

### ideation-skill

- [ ] Scenario: `The persona template carries the stable core and the running record`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: wrote `assets/templates/persona.md` (stable core, running record, topic stances). Files: `assets/templates/persona.md`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(ideation-skill): The persona template carries the stable core and the running record`

### lint-command

- [ ] Scenario: `A dangling persona reference is reported as an advisory`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: implemented `check_persona_references` in `src/cli/lint.rs` and the three Givens in `tests/steps/mod.rs`. Files: `src/cli/lint.rs`, `tests/steps/mod.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): A dangling persona reference is reported as an advisory`

### lint-command

- [ ] Scenario: `A persona reference resolved by a project persona is not reported`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the project persona resolution home. Files: `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): A persona reference resolved by a project persona is not reported`

### lint-command

- [ ] Scenario: `A persona reference resolved by a topic persona is not reported`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the topic persona resolution home. Files: `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): A persona reference resolved by a topic persona is not reported`

### lint-command

- [ ] Scenario: `The change scope reports a dangling persona reference`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the `--change` scan scope. Files: `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): The change scope reports a dangling persona reference`

### visual-review-gate

- [ ] Scenario: `A visual change with an unknown persona disposition fails review`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: implemented the ID resolution in `src/integrity_hook.rs::check_persona_disposition` and created the fixture persona file in the shared design-review Given. Files: `src/integrity_hook.rs`, `tests/steps/workflow_enforcement_steps.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A visual change with an unknown persona disposition fails review`

### visual-review-gate

- [ ] Scenario: `A visual change with a reasoned none fails review`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the resolution: no `none — ...` persona file exists. Files: `src/integrity_hook.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A visual change with a reasoned none fails review`

### visual-review-gate

- [ ] Scenario: `A change without a visual interface passes review with a reasoned none`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: GUARD (immediate GREEN): all steps reuse the existing generic steps; the exempt path never reaches the persona check. Files: none. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A change without a visual interface passes review with a reasoned none`

## Final checks

- [ ] Run the full suite (`./run-tests.sh`) → zero exit.
- [ ] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit.
- [ ] Run the change delta suite → zero exit.
- [ ] Grep every touched step-definition file for empty/no-op bodies → 0.
- [ ] Confirm commit count matches scenario task count and every task has a
      recorded hash.
- [ ] Confirm `givn lint --change persona-identity-reuse` has no parse errors.
