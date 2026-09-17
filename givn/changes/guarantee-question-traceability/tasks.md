# Tasks: guarantee-question-traceability

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/guarantee-question-traceability/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- The qualifier literal is pinned as `(open: Q<N>)` in every surface.
- The disposition rule is blocking only for ideation-sourced changes;
  otherwise advisory.
- The ledger links back to the seed question ID (`Decides D2 (seed Q5)`).
- Confirmed Personas: `agent--technical-role` and `givn-maintainer--owner`.

Capabilities and step files:

- `ideation-skill` — reused generic steps in `tests/steps/prose_consolidation_steps.rs`
- `instruction-integrity` — `tests/steps/instruction_integrity_steps.rs`: new Givens (instruction plus command) and assertion steps; the template scenario reuses `givn eject` and the generic file-contains step

## Setup

- [x] Confirm strict mode and the runner: `.fail_on_skipped()`; `verify.command` is `./run-tests.sh`.
- [x] Proof-of-strictness: temporary `tests/strict-mode-probe.feature` → non-zero
      exit, then deleted. Evidence:
      ```
      GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --locked --test features_runner
      1 scenario (1 failed)
      1 step (1 failed)
      exit status: 1
      ```
- [x] Created the step skeletons (14 steps in
      `tests/steps/instruction_integrity_steps.rs`) with `unimplemented!()`
      bodies. Evidence:
      ```
      cargo check --locked --tests
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.92s
      ```
- [x] COMMIT: `4ef5e023` -- `test(guarantee-question-traceability): strict-mode proof and step skeletons`

## Scenarios

### ideation-skill

- [ ] Scenario: `The ideation guidance carries the open-question qualifier rule`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/guarantee-question-traceability/specs cargo test --locked --test features_runner -- --name "The ideation guidance carries the open-question qualifier rule"
        Step panicked. Captured output: givn-ideation guidance missing "(open: Q<N>)"
        1 scenario (1 failed)
        ```
  - [ ] GREEN: added the `(open: Q<N>)` qualifier rule to `assets/skills/givn-ideation/SKILL.md.tmpl`. Files: that asset. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `7590e8fd` -- `feat(ideation-skill): The ideation guidance carries the open-question qualifier rule`

### instruction-integrity

- [ ] Scenario: `The proposal instruction carries the question disposition rule`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/guarantee-question-traceability/specs cargo test --locked --test features_runner -- --name "The proposal instruction carries the question disposition rule"
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        ```
  - [ ] GREEN: added the disposition rule to `assets/instructions/proposal.md`, `assets/commands/givn-propose.md`, and `assets/templates/proposal.md`. Files: those three assets. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        ```
  - [x] COMMIT: `c35ec246` -- `feat(instruction-integrity): The proposal instruction carries the question disposition rule`

### instruction-integrity

- [ ] Scenario: `The design and design-review guidance carry the decision ledger`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/guarantee-question-traceability/specs cargo test --locked --test features_runner -- --name "The design and design-review guidance carry the decision ledger"
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        ```
  - [ ] GREEN: added the ledger contract to `assets/instructions/design.md`, `assets/templates/design.md`, `assets/instructions/design-review.md`, `assets/commands/givn-design-review.md`, and `assets/templates/design-review.md`. Files: those five assets. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `76c3e4e6` -- `feat(instruction-integrity): The design and design-review guidance carry the decision ledger`

### instruction-integrity

- [ ] Scenario: `The review instruction rejects an unqualified guarantee with an open prerequisite`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/guarantee-question-traceability/specs cargo test --locked --test features_runner -- --name "The review instruction rejects an unqualified guarantee with an open prerequisite"
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)
        ```
  - [ ] GREEN: added the open-prerequisite line to `assets/instructions/review.md`. Files: that asset. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        ```
  - [x] COMMIT: `d5beffee` -- `feat(instruction-integrity): The review instruction rejects an unqualified guarantee with an open prerequisite`

### instruction-integrity

- [ ] Scenario: `The proposal design and design-review templates carry the ledger shapes`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        restored the three templates from the setup revision (4ef5e023):
        Step panicked. Captured output: givn/artifacts/proposal/proposal.md lacks "Disposition"
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        ```
  - [ ] GREEN: covered by the template edits in the earlier scenarios; this scenario ejects and asserts them. Files: none. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] COMMIT: `6b5672bf` -- `feat(instruction-integrity): The proposal design and design-review templates carry the ledger shapes`

## Final checks

- [x] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      Summary [30.538s] 528 tests run: 528 passed, 0 skipped
      ```
- [x] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      Summary [5.567s] 88 tests run: 88 passed, 440 skipped
      ```
- [x] Run the change delta suite → zero exit. Evidence:
      ```
      2 features
      5 scenarios (5 passed)
      29 steps (29 passed)
      ```
- [x] Grep every touched step-definition file for empty/no-op bodies → 0. Evidence:
      ```
      grep -c "unimplemented!\|todo!()" tests/steps/instruction_integrity_steps.rs
      0
      ```
- [x] Confirm commit count matches scenario task count and every task has a
      recorded hash: setup `4ef5e023`, scenarios `7590e8fd`, `c35ec246`,
      `76c3e4e6`, `d5beffee`, `6b5672bf`.
- [x] Confirm `givn lint --change guarantee-question-traceability` has no
      parse errors (exit 0). Evidence:
      ```
      givn lint: 2 file(s) checked — clean
      exit 0
      ```
