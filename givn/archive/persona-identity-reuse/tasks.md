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

- [x] Confirm strict mode and the runner: `.fail_on_skipped()`; `verify.command` is `./run-tests.sh`.
- [x] Proof-of-strictness: temporary `tests/strict-mode-probe.feature` with one
      undefined step → non-zero exit, then deleted. Evidence:
      ```
      GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --locked --test features_runner
      1 scenario (1 failed)
      1 step (1 failed)
      exit status: 1
      ```
- [x] Created the step skeletons (three fixture Givens in `tests/steps/mod.rs`,
      six template steps in `tests/steps/instruction_integrity_steps.rs`) with
      `unimplemented!()` bodies. Evidence:
      ```
      cargo check --locked --tests
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.81s
      ```
- [x] COMMIT: `<hash>` -- `test(persona-identity-reuse): strict-mode proof and step skeletons`

## Scenarios

### ideation-skill

- [x] Scenario: `The ideation guidance carries the persona identity criteria and the reuse protocol`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/persona-identity-reuse/specs cargo test --locked --test features_runner -- --name "The ideation guidance carries the persona identity criteria and the reuse protocol"
Step panicked. Captured output: givn-ideation guidance missing "authority boundary"
1 scenario (1 failed)
        ```
  - [x] GREEN: added the identity criteria, the reuse protocol, the header line, and the schema reference to `assets/skills/givn-ideation/SKILL.md.tmpl`; added the guardrails to `assets/commands/givn-ideation.md`. Files: those two assets. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
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
  - [x] COMMIT: `b119a89f` -- `feat(ideation-skill): The ideation guidance carries the persona identity criteria and the reuse protocol`

### ideation-skill

- [x] Scenario: `The ideation guidance keeps the confirmed persona behaviors`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GUARD (immediate GREEN): all steps reuse the existing generic steps; the assertions pin the preserved behaviors. No RED run.
        ```
  - [x] GREEN: no files changed; guard only. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        13 steps (13 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        13 steps (13 passed)
        ```
  - [x] COMMIT: `8e570f1d` -- `feat(ideation-skill): The ideation guidance keeps the confirmed persona behaviors`

### ideation-skill

- [x] Scenario: `The persona template carries the stable core and the running record`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/persona-identity-reuse/specs cargo test --locked --test features_runner -- --name "The persona template carries the stable core and the running record"
Step panicked. Captured output: not implemented
1 scenario (1 failed)
        ```
  - [x] GREEN: wrote `assets/templates/persona.md` (stable core, running record, topic stances) and implemented the six steps. Files: `assets/templates/persona.md`, `tests/steps/instruction_integrity_steps.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
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
  - [x] COMMIT: `95601fe6` -- `feat(ideation-skill): The persona template carries the stable core and the running record`

### lint-command

- [x] Scenario: `A dangling persona reference is reported as an advisory`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/persona-identity-reuse/specs cargo test --locked --test features_runner -- --name "A dangling persona reference is reported as an advisory"
Step panicked. Captured output: not implemented
1 scenario (1 failed)

The folded scenario also failed RED against the pre-check revision:
Step panicked. Captured output: stderr missing "[PERS ]"
1 scenario (1 failed)
9 steps (8 passed, 1 failed)
        ```
  - [x] GREEN: implemented `check_persona_references` in `src/cli/lint.rs`, the `[PERS ]` render arm, and the fixture Givens in `tests/steps/mod.rs`; the project- and topic-resolution guards were folded into this scenario (commit 883bee06). Files: `src/cli/lint.rs`, `tests/steps/mod.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
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
  - [x] COMMIT: `90aec644` -- `feat(lint-command): A dangling persona reference is reported as an advisory`

### lint-command

- [x] Scenario: `The change scope reports a dangling persona reference`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        restored `src/cli/lint.rs` to the pre-check revision (a992f90f):
Step panicked. Captured output: stderr missing "[PERS ]"
1 scenario (1 failed)
5 steps (4 passed, 1 failed)
        ```
  - [x] GREEN: covered by the `--change` scan scope in `src/cli/lint.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
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
  - [x] COMMIT: `ea2a6e4e` -- `feat(lint-command): The change scope reports a dangling persona reference`

### visual-review-gate

- [x] Scenario: `A visual change with an unknown persona disposition fails review`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        stashed `src/integrity_hook.rs` and the fixture change back to the pre-resolution state:
Step panicked. Captured output: expected failure, got exit Some(0)
1 scenario (1 failed)
9 steps (8 passed, 1 failed)
        ```
  - [x] GREEN: implemented the ID resolution in `src/integrity_hook.rs::check_persona_disposition` and created the fixture persona file in the shared design-review Given (`tests/steps/workflow_enforcement_steps.rs`). Files: `src/integrity_hook.rs`, `tests/steps/workflow_enforcement_steps.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        ```
  - [x] COMMIT: `18185e92` -- `feat(visual-review-gate): A visual change with an unknown persona disposition fails review`

### visual-review-gate

- [x] Scenario: `A visual change with a reasoned none fails review`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        restored `src/integrity_hook.rs` to a992f90f:
Step panicked. Captured output: expected failure, got exit Some(0)
1 scenario (1 failed)
9 steps (8 passed, 1 failed)
        ```
  - [x] GREEN: covered by the resolution: no `none — ...` persona file exists. Files: `src/integrity_hook.rs`. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        ```
  - [x] COMMIT: `38edbb52` -- `feat(visual-review-gate): A visual change with a reasoned none fails review`

### visual-review-gate

- [x] Scenario: `A change without a visual interface passes review with a reasoned none`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario command → non-zero. Evidence:
        ```
        GUARD (immediate GREEN): all steps reuse the existing generic steps; the exempt path never reaches the persona check. No RED run.
        ```
  - [x] GREEN: no files changed; guard only. `cargo clean -p givn` where assets changed, then the targeted run → zero. Evidence:
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
  - [x] COMMIT: `5468db24` -- `feat(visual-review-gate): A change without a visual interface passes review with a reasoned none`

## Final checks

- [x] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      Summary [29.957s] 520 tests run: 520 passed, 0 skipped
      ```
- [x] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      Summary [6.630s] 88 tests run: 88 passed, 432 skipped
      ```
- [x] Run the change delta suite → zero exit. Evidence:
      ```
      3 features
      8 scenarios (8 passed)
      75 steps (75 passed)
      ```
- [x] Grep every touched step-definition file for empty/no-op bodies → 0. Evidence:
      ```
      grep -c "unimplemented!\|todo!()" tests/steps/mod.rs tests/steps/instruction_integrity_steps.rs
      0
      0
      ```
- [x] Confirm commit count matches scenario task count and every task has a
      recorded hash: setup `a992f90f`, scenarios `b119a89f`, `8e570f1d`,
      `95601fe6`, `90aec644` (+ fold `883bee06`), `ea2a6e4e`, `18185e92`,
      `38edbb52`, `5468db24`.
- [x] Confirm `givn lint --change persona-identity-reuse` has no parse errors
      (exit 0). Evidence:
      ```
      givn lint: 3 file(s) checked — clean
      exit 0
      ```
