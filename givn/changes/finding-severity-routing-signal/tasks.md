# Tasks: finding-severity-routing-signal

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/finding-severity-routing-signal/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- The lint exit-code contract is preserved; severity is descriptive.
- The recorded routing decision stays mandatory; route stays advisory.
- Route fixture arithmetic is pinned in design.md (tied pair 1.5 each;
  aligned 4.4 vs mismatched 0.2).
- Confirmed Personas: `agent--technical-role` and `givn-maintainer--owner`.

Capabilities and step files:

- `lint-command` — reused generic steps in `tests/steps/mod.rs`
- `spec-explore-change` — `tests/steps/spec_discovery_deterministic_steps.rs`: two corpus Givens and one proposal Given
- `instruction-integrity` — `tests/steps/instruction_integrity_steps.rs`: one When and two Thens

## Setup

- [x] Confirm strict mode and the runner: `.fail_on_skipped()`; `verify.command` is `./run-tests.sh`.
- [x] Proof-of-strictness: temporary probe → non-zero exit, then deleted. Evidence:
      ```
      GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --locked --test features_runner
      1 scenario (1 failed)
      1 step (1 failed)
      exit status: 1
      ```
- [x] Created the step skeletons (three route Givens, three instruction steps)
      with `unimplemented!()` bodies. Evidence:
      ```
      cargo check --locked --tests
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.90s
      ```
- [x] COMMIT: `<hash>` -- `test(finding-severity-routing-signal): strict-mode proof and step skeletons`

## Scenarios

### lint-command

- [ ] Scenario: `A blocking finding renders its severity`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the severity classification and rendering to `src/cli/lint.rs`. Files: `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): A blocking finding renders its severity`

### lint-command

- [ ] Scenario: `An advisory finding renders its severity and keeps the exit code`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the severity classification. Files: `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): An advisory finding renders its severity and keeps the exit code`

### lint-command

- [ ] Scenario: `The JSON output exposes the finding severity`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the severity JSON enrichment in both print branches. Files: `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): The JSON output exposes the finding severity`

### spec-explore-change

- [ ] Scenario: `A tied route reports no signal`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: implemented the confidence rules in `src/spec/route.rs` and the tied corpus Given. Files: `src/spec/route.rs`, `tests/steps/spec_discovery_deterministic_steps.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(spec-explore-change): A tied route reports no signal`

### spec-explore-change

- [ ] Scenario: `A goal-aligned capability ranks above a lexically similar candidate`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: implemented ownership-weighted scoring and the aligned corpus Given; added the route unit test first. Files: `src/spec/route.rs`, `tests/steps/spec_discovery_deterministic_steps.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(spec-explore-change): A goal-aligned capability ranks above a lexically similar candidate`

### spec-explore-change

- [ ] Scenario: `A proposal with no matching surface reports no signal`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the confidence rules and the removed/added delta. Files: `src/spec/route.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(spec-explore-change): A proposal with no matching surface reports no signal`

### instruction-integrity

- [ ] Scenario: `The propose guidance carries the no-signal routing rule`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the rule to the instruction, command, and template; implemented the steps. Files: `assets/instructions/proposal.md`, `assets/commands/givn-propose.md`, `assets/templates/proposal.md`, `tests/steps/instruction_integrity_steps.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(instruction-integrity): The propose guidance carries the no-signal routing rule`

## Final checks

- [ ] Run the full suite (`./run-tests.sh`) → zero exit.
- [ ] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit.
- [ ] Run the change delta suite → zero exit.
- [ ] Grep every touched step-definition file for empty/no-op bodies → 0.
- [ ] Confirm commit count matches scenario task count and every task has a
      recorded hash.
- [ ] Confirm `givn lint --change finding-severity-routing-signal` has no
      parse errors.
