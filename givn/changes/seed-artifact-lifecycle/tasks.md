# Tasks: seed-artifact-lifecycle

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/seed-artifact-lifecycle/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- `author-change`: generated guidance is synchronized from canonical source
  assets; generated guidance may name only commands that work.
- `verify-change`: lint is a static check with a stable exit-code contract;
  an advisory must never change it.
- Support-fragment capabilities (`ideation-skill`) keep their fragment
  ownership and carry no independent user goal or inventory.
- Confirmed Personas: `agent--technical-role` (generated guidance, lint
  output) and `givn-maintainer--owner` (seed convention, rule corpus).
  Actors in Gherkin stay Developer and Authoring agent.
- A mandatory check is never silently deferred.

Capabilities and step files:

- `ideation-skill` — reused generic steps in `tests/steps/prose_consolidation_steps.rs`
- `prose-consolidation` — reused generic steps in `tests/steps/prose_consolidation_steps.rs`
- `lint-command` — `tests/steps/mod.rs`: one new fixture Given (`a change seed at "<path>" with:`) and one new assertion (`stderr does not contain "..."`)

No scenario in this change is tagged `@e2e`: running the static lint check is
already driven end-to-end by the two `spec-overlap-gates` `@e2e` scenarios,
and the new scenarios are regular variants (advisory findings, guidance
phrases).

## Setup

- [x] Confirm strict mode and the runner: `tests/features_runner.rs` uses
      `.fail_on_skipped()`; `verify.command` is `./run-tests.sh` and
      `verify.e2e_command` is `./run-tests.sh --e2e`. No new runner is needed.
- [x] Proof-of-strictness: wrote a temporary `tests/strict-mode-probe.feature`
      with one undefined step, ran it, confirmed non-zero exit, then deleted
      the probe. Evidence:
      ```
      GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --locked --test features_runner
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      exit status: 1
      ```
- [x] Created the step skeletons in `tests/steps/mod.rs` (`a change seed at
      "<path>" with:` and `stderr does not contain "..."`) with
      `unimplemented!()` bodies. Evidence:
      ```
      cargo check --locked --tests
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
      ```
- [x] COMMIT: `<hash>` -- `test(seed-artifact-lifecycle): strict-mode proof and step skeletons`

## Scenarios

### ideation-skill

- [ ] Scenario: `The ideation guidance carries the seed schema and its status values`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the seed schema and status values to
        `assets/skills/givn-ideation/SKILL.md.tmpl`; added the guardrail to
        `assets/commands/givn-ideation.md`. Files:
        `assets/skills/givn-ideation/SKILL.md.tmpl`,
        `assets/commands/givn-ideation.md`. `cargo clean -p givn`, then the
        targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(ideation-skill): The ideation guidance carries the seed schema and its status values`

### prose-consolidation

- [ ] Scenario: `The propose and archive commands carry the seed stamping instructions`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the stamping instructions to
        `assets/commands/givn-propose.md`, `assets/instructions/proposal.md`,
        `assets/templates/proposal.md`, and `assets/commands/givn-archive.md`.
        Files: those four assets. `cargo clean -p givn`, then the targeted run
        → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(prose-consolidation): The propose and archive commands carry the seed stamping instructions`

### lint-command

- [ ] Scenario: `A change seed without a status line is reported as an advisory`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: implemented `check_seed_status` in `src/cli/lint.rs` (scan,
        tolerant status parse, `seed_status` findings, `[SEED ]` rendering,
        advisory counting) and the two new steps in `tests/steps/mod.rs`.
        Files: `src/cli/lint.rs`, `tests/steps/mod.rs`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): A change seed without a status line is reported as an advisory`

- [ ] Scenario: `A change seed proposed by a missing change is reported as an advisory`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the seed check's dangling branch and the early-return
        rendering. Files: `src/cli/lint.rs`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): A change seed proposed by a missing change is reported as an advisory`

- [ ] Scenario: `A change seed proposed by an archived change is reported as stale`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the archived branch. Files: `src/cli/lint.rs`. Run →
        zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): A change seed proposed by an archived change is reported as stale`

- [ ] Scenario: `A change seed proposed by an active change is not reported`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the active-change branch. Files: `src/cli/lint.rs`.
        Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): A change seed proposed by an active change is not reported`

- [ ] Scenario: `An unrecognised seed status is reported and a status after prose is honoured`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the tolerant first-match parse and the unrecognised
        classification. Files: `src/cli/lint.rs`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): An unrecognised seed status is reported and a status after prose is honoured`

- [ ] Scenario: `An advisory seed finding is counted in the summary and still exits 0`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: covered by the advisory count and exit-code neutrality. Files:
        `src/cli/lint.rs`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(lint-command): An advisory seed finding is counted in the summary and still exits 0`

## Final checks

- [ ] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      ```
- [ ] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      ```
- [ ] Run the change delta suite
      (`GIVN_FEATURES=givn/changes/seed-artifact-lifecycle/specs cargo test --locked --test features_runner`)
      → zero exit. Evidence:
      ```
      ```
- [ ] Grep every touched step-definition file for empty/no-op bodies → 0.
      Evidence:
      ```
      ```
- [ ] Confirm commit count matches scenario task count and every task has a
      recorded hash.
- [ ] Confirm `givn lint --change seed-artifact-lifecycle` has no parse errors
      (exit 0 or 2).
