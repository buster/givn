# Tasks: use-case-model-scoping

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/use-case-model-scoping/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- `make-usecases-canonical`: a capability has exactly one use-case owner; the
  corpus remains inspectable when a narrative is invalid.
- `author-change`: generated guidance is synchronized from canonical assets.
- `instruction-integrity`: instruction and skill guidance stays complete and
  machine-readable.
- Support-fragment capabilities keep fragment ownership and carry no
  independent user goal or inventory.
- Confirmed Personas: `agent--technical-role` (guidance, template) and
  `givn-maintainer--owner` (model document, criteria). Actors in Gherkin stay
  the maintainer, reviewer, author, and agent.
- Phrase drift is the top risk: copy the definition, the eight rule titles,
  the ten criterion titles, and the authoring phrases verbatim from design.md
  into each artifact.

Capabilities and step files:

- `canonical-usecase-contract` — `tests/steps/spec_discovery_deterministic_steps.rs`: new steps reading `docs/use-case-model.md`, `docs/workflow.md`, `docs/reference.md`, and the in-process template
- `ideation-skill` — reused generic steps in `tests/steps/prose_consolidation_steps.rs`
- `instruction-integrity` — `tests/steps/instruction_integrity_steps.rs`: new steps reading the review, design-review, and specs assets

No scenario in this change is tagged `@e2e`: no new consumer action is
introduced, and the new scenarios are guidance-content and document variants.

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
      `tests/steps/spec_discovery_deterministic_steps.rs` (19 steps) and
      `tests/steps/instruction_integrity_steps.rs` (18 steps) with
      `unimplemented!()` bodies. Evidence:
      ```
      cargo check --locked --tests
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.04s
      ```
- [x] COMMIT: `<hash>` -- `test(use-case-model-scoping): strict-mode proof and step skeletons`

## Scenarios

### canonical-usecase-contract

- [x] Scenario: `The durable use-case model publishes the canonical definition and rules`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/use-case-model-scoping/specs cargo test --locked --test features_runner -- --name "The durable use-case model publishes the canonical definition and rules"
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        ```
  - [x] GREEN: wrote `docs/use-case-model.md` (definition, eight rules,
        authoring requirements, ten criteria, both worked examples with
        criteria tie-ins) and added the pointers to `docs/workflow.md` and
        `docs/reference.md`; implemented the twelve steps. Files:
        `docs/use-case-model.md`, `docs/workflow.md`, `docs/reference.md`,
        `tests/steps/spec_discovery_deterministic_steps.rs`. Run → zero.
        Evidence:
        ```
        ✔ Given the durable use-case model document is inspected
        ✔ When a maintainer reads the definition and the scoping rules
        ✔ Then the document defines a use case as a contract between one primary actor and the system
        ✔ And ... the promotion rule ... placement test ... ten review rejection criteria
        ✔ And ... the worked split of an observing mega-leaf
        ✔ And ... the promotion of a conjunction-goal leaf into a summary and its split
        ✔ And ... both criteria tie-ins
        ✔ And ... the workflow and reference documents point at the use-case model
        1 feature
        1 scenario (1 passed)
        12 steps (12 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        12 steps (12 passed)
        ```
  - [x] COMMIT: `<hash>` -- `feat(canonical-usecase-contract): The durable use-case model publishes the canonical definition and rules`

- [ ] Scenario: `The generated use-case document template carries the authoring guidance`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the primary-actor convention, the numbered main-flow
        pairing, the extension-format bullet, and the promotion note to
        `src/gherkin/usecase_md.rs::usecase_md_template()`, preserving all six
        fixture placeholders. Files: `src/gherkin/usecase_md.rs`. Run → zero.
        Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(canonical-usecase-contract): The generated use-case document template carries the authoring guidance`

### ideation-skill

- [ ] Scenario: `The ideation guidance carries the canonical definition and the eight scoping rules`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the definition and the eight rules to the Use Case
        Definition section of `assets/skills/givn-ideation/SKILL.md.tmpl`,
        keeping every existing heading and phrase. Files:
        `assets/skills/givn-ideation/SKILL.md.tmpl`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(ideation-skill): The ideation guidance carries the canonical definition and the eight scoping rules`

- [ ] Scenario: `The ideation guidance carries the authoring requirements and the handoff mapping`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the authoring requirements to Use Case Elaboration and
        the leaf-document mapping to Handoff. Files:
        `assets/skills/givn-ideation/SKILL.md.tmpl`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(ideation-skill): The ideation guidance carries the authoring requirements and the handoff mapping`

### instruction-integrity

- [ ] Scenario: `The review instruction carries the use-case scope rejection criteria`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the ten criteria to `assets/instructions/review.md`.
        Files: `assets/instructions/review.md`. `cargo clean -p givn`, then the
        targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(instruction-integrity): The review instruction carries the use-case scope rejection criteria`

- [ ] Scenario: `The design-review guidance points at the use-case scope criteria`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the pointer to the use-case branch of
        `assets/instructions/design-review.md`. Files:
        `assets/instructions/design-review.md`. `cargo clean -p givn`, then the
        targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(instruction-integrity): The design-review guidance points at the use-case scope criteria`

- [ ] Scenario: `The specs guidance carries the capability placement test`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the placement test to `assets/instructions/specs.md` and
        the reference to `assets/skills/givn-spec/SKILL.md.tmpl`. Files:
        `assets/instructions/specs.md`,
        `assets/skills/givn-spec/SKILL.md.tmpl`. `cargo clean -p givn`, then
        the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(instruction-integrity): The specs guidance carries the capability placement test`

## Final checks

- [ ] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      ```
- [ ] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      ```
- [ ] Run the change delta suite
      (`GIVN_FEATURES=givn/changes/use-case-model-scoping/specs cargo test --locked --test features_runner`)
      → zero exit. Evidence:
      ```
      ```
- [ ] Grep every touched step-definition file for empty/no-op bodies → 0.
      Evidence:
      ```
      ```
- [ ] Confirm commit count matches scenario task count and every task has a
      recorded hash.
- [ ] Confirm `givn lint --change use-case-model-scoping` has no parse errors
      (exit 0 or 2).
