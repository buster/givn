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
- [x] COMMIT: `e29e9d3e` -- `test(finding-severity-routing-signal): strict-mode proof and step skeletons`

## Scenarios

### lint-command

- [x] Scenario: `A blocking finding renders its severity`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/finding-severity-routing-signal/specs cargo test --locked --test features_runner -- --name "A blocking finding renders its severity"
        Step panicked. Captured output: stderr missing "[ERROR] blocking"
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        ```
  - [x] GREEN: added the severity classification and rendering to `src/cli/lint.rs`. Files: `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
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
  - [x] COMMIT: `55c0987c` -- `feat(lint-command): A blocking finding renders its severity`

### lint-command

- [x] Scenario: `An advisory finding renders its severity and keeps the exit code`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        restored `src/cli/lint.rs` to the setup revision (e29e9d3e):
        Step panicked. Captured output: stderr missing "[SHAPE] advisory"
        1 scenario (1 failed)
        6 steps (5 passed, 1 failed)
        ```
  - [x] GREEN: covered by the severity classification. Files: `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `0ef3d685` -- `feat(lint-command): An advisory finding renders its severity and keeps the exit code`

### lint-command

- [x] Scenario: `The JSON output exposes the finding severity`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        restored `src/cli/lint.rs` to e29e9d3e:
        Step panicked. Captured output: stdout missing "severity"
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        ```
  - [x] GREEN: covered by the severity JSON enrichment in both print branches. Files: `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] COMMIT: `39016c10` -- `feat(lint-command): The JSON output exposes the finding severity`

### spec-explore-change

- [x] Scenario: `A tied route reports no signal`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        restored `src/spec/route.rs` to e29e9d3e:
        Step panicked. Captured output: stdout missing "no signal": recommendation: EXTEND lint-check (use case 'gates', score 4.00)
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        ```
  - [x] GREEN: implemented the confidence rules in `src/spec/route.rs` and the tied corpus Given. Files: `src/spec/route.rs`, `tests/steps/spec_discovery_deterministic_steps.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
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
  - [x] COMMIT: `6f62406b` -- `feat(spec-explore-change): A tied route reports no signal`

### spec-explore-change

- [x] Scenario: `A goal-aligned capability ranks above a lexically similar candidate`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        restored `src/spec/route.rs` to the lint revision (39016c10):
        Step panicked. Captured output: stdout missing "recommendation: EXTEND coverage-tool": recommendation: NEW in alignment
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        ```
  - [x] GREEN: implemented ownership-weighted scoring and the aligned corpus Given; added the route unit test first. Files: `src/spec/route.rs`, `tests/steps/spec_discovery_deterministic_steps.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `a99b1a6a` -- `feat(spec-explore-change): A goal-aligned capability ranks above a lexically similar candidate`

### spec-explore-change

- [x] Scenario: `A proposal with no matching surface reports no signal`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        restored `src/spec/route.rs` to 39016c10:
        Step panicked. Captured output: stdout missing "recommendation: no signal": recommendation: NEW (no use-case affinity signal)
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        ```
  - [x] GREEN: covered by the confidence rules and the removed/added delta. Files: `src/spec/route.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
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
  - [x] COMMIT: `cf4cd844` -- `feat(spec-explore-change): A proposal with no matching surface reports no signal`

### instruction-integrity

- [x] Scenario: `The propose guidance carries the no-signal routing rule`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/finding-severity-routing-signal/specs cargo test --locked --test features_runner -- --name "The propose guidance carries the no-signal routing rule"
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        ```
  - [x] GREEN: added the rule to the instruction, command, and template; implemented the steps. Files: `assets/instructions/proposal.md`, `assets/commands/givn-propose.md`, `assets/templates/proposal.md`, `tests/steps/instruction_integrity_steps.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
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
  - [x] COMMIT: `8baba4fc` -- `feat(instruction-integrity): The propose guidance carries the no-signal routing rule`

## Final checks

- [x] Run the change delta suite (`GIVN_FEATURES=givn/changes/finding-severity-routing-signal/specs ./run-tests.sh`) → zero exit. Evidence:
      ```
      3 features
      8 scenarios (8 passed)
      45 steps (45 passed)
      ```
      The permanent corpus still asserts the pre-change route behavior for the
      replaced scenario until the archive merge; the archive verification runs
      the merged corpus.
- [x] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      Summary [5.693s] 88 tests run: 88 passed, 445 skipped
      ```
- [x] Run the change delta suite → zero exit (recorded above).
- [x] Grep every touched step-definition file for empty/no-op bodies → 0. Evidence:
      ```
      grep -c "unimplemented!\|todo!()" tests/steps/spec_discovery_deterministic_steps.rs tests/steps/instruction_integrity_steps.rs
      0
      0
      ```
- [x] Confirm commit count matches scenario task count and every task has a
      recorded hash: setup `e29e9d3e`; scenarios `55c0987c`, `0ef3d685`,
      `39016c10`, `6f62406b`, `a99b1a6a`, `cf4cd844`, `8baba4fc`.
- [x] Confirm `givn lint --change finding-severity-routing-signal` has no
      parse errors (exit 0). Evidence:
      ```
      givn lint: 3 file(s) checked — clean
      exit 0
      ```
