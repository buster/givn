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
- [x] COMMIT: `9839d373` -- `test(use-case-model-scoping): strict-mode proof and step skeletons`

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
  - [x] COMMIT: `6a9ab493` -- `feat(canonical-usecase-contract): The durable use-case model publishes the canonical definition and rules`

- [x] Scenario: `The generated use-case document template carries the authoring guidance`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/use-case-model-scoping/specs cargo test --locked --test features_runner -- --name "The generated use-case document template carries the authoring guidance"
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        ```
  - [x] GREEN: added the primary-actor convention, the numbered main-flow
        pairing, the extension-format bullet, and the promotion note to
        `src/gherkin/usecase_md.rs::usecase_md_template()`, preserving all six
        fixture placeholders; implemented the seven steps. Files:
        `src/gherkin/usecase_md.rs`,
        `tests/steps/spec_discovery_deterministic_steps.rs`. Run → zero.
        Evidence:
        ```
        ✔ Then the template names the primary-actor convention
        ✔ And the template shows the extension format
        ✔ And the template numbers the main-flow example
        ✔ And the template carries the promotion note
        ✔ And the template preserves the fixture placeholders
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] COMMIT: `5d7a362d` -- `feat(canonical-usecase-contract): The generated use-case document template carries the authoring guidance`

### ideation-skill

- [x] Scenario: `The ideation guidance carries the canonical definition and the eight scoping rules`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)
        ```
  - [x] GREEN: added the definition and the eight rules to the Use Case
        Definition section of `assets/skills/givn-ideation/SKILL.md.tmpl`,
        keeping every existing heading and phrase. Files:
        `assets/skills/givn-ideation/SKILL.md.tmpl`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ✔ And ... "contract between one primary actor and the system"
        ✔ And ... the eight rule titles
        ✔ And ... "promotion rule", "three or more distinct actor goals", "Placement test"
        1 feature
        1 scenario (1 passed)
        14 steps (14 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        14 steps (14 passed)
        ```
  - [x] COMMIT: `0a27df00` -- `feat(ideation-skill): The ideation guidance carries the canonical definition and the eight scoping rules`

- [x] Scenario: `The ideation guidance carries the authoring requirements and the handoff mapping`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)
        ```
  - [x] GREEN: added the authoring requirements to Use Case Elaboration and
        the leaf-document mapping to Handoff (rewrapped so the asserted phrase
        stays on one line). Files:
        `assets/skills/givn-ideation/SKILL.md.tmpl`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ✔ And ... "the first actor is the primary actor"
        ✔ And ... "Supporting:"
        ✔ And ... "protects <guarantee>"
        ✔ And ... "never an extension"
        ✔ And ... "never to a capability inside another goal"
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] COMMIT: `b77d193a` -- `feat(ideation-skill): The ideation guidance carries the authoring requirements and the handoff mapping`

### instruction-integrity

- [x] Scenario: `The review instruction carries the use-case scope rejection criteria`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        ```
  - [x] GREEN: added the ten criteria to `assets/instructions/review.md`;
        implemented the ten steps. Files: `assets/instructions/review.md`,
        `tests/steps/instruction_integrity_steps.rs`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ✔ Given the review instructions are requested
        ✔ When the reviewer reads the use-case scope criteria
        ✔ Then ... all ten rejection criteria
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
  - [x] COMMIT: `e4d10039` -- `feat(instruction-integrity): The review instruction carries the use-case scope rejection criteria`

- [x] Scenario: `The design-review guidance points at the use-case scope criteria`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        ```
  - [x] GREEN: added the pointer to the use-case branch of
        `assets/instructions/design-review.md`; implemented the three steps.
        Files: `assets/instructions/design-review.md`,
        `tests/steps/instruction_integrity_steps.rs`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        ```
  - [x] COMMIT: `9a9ae567` -- `feat(instruction-integrity): The design-review guidance points at the use-case scope criteria`

- [x] Scenario: `The specs guidance carries the capability placement test`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        ```
  - [x] GREEN: added the placement test to `assets/instructions/specs.md` and
        the reference to `assets/skills/givn-spec/SKILL.md.tmpl`; implemented
        the four steps. Files: `assets/instructions/specs.md`,
        `assets/skills/givn-spec/SKILL.md.tmpl`,
        `tests/steps/instruction_integrity_steps.rs`. `cargo clean -p givn`,
        then the targeted run → zero. Evidence:
        ```
        ✔ When an author reads the capability placement guidance
        ✔ Then the guidance carries the placement test
        ✔ And the guidance rejects a channel as an answer
        ✔ And the guidance requires goal-shaped capability names
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `77db712c` -- `feat(instruction-integrity): The specs guidance carries the capability placement test`

## Final checks

- [x] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      Summary [29.383s] 507 tests run: 507 passed, 0 skipped
      ```
- [x] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      Summary [5.572s] 88 tests run: 88 passed, 419 skipped
      ```
- [x] Run the change delta suite
      (`GIVN_FEATURES=givn/changes/use-case-model-scoping/specs cargo test --locked --test features_runner`)
      → zero exit. Evidence:
      ```
      3 features
      7 scenarios (7 passed)
      60 steps (60 passed)
      ```
- [x] Grep every touched step-definition file for empty/no-op bodies → 0.
      Evidence:
      ```
      grep -n "unimplemented!\|todo!()" tests/steps/spec_discovery_deterministic_steps.rs tests/steps/instruction_integrity_steps.rs
      (no matches)
      ```
- [x] Confirm commit count matches scenario task count and every task has a
      recorded hash: one setup commit (`9839d373`) plus seven scenario commits
      (`6a9ab493`, `5d7a362d`, `0a27df00`, `b77d193a`, `e4d10039`, `9a9ae567`,
      `77db712c`).
- [x] Confirm `givn lint --change use-case-model-scoping` has no parse errors
      (exit 0). Evidence:
      ```
      givn lint: 3 file(s) checked — clean
      exit 0
      ```
