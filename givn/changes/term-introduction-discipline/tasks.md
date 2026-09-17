# Tasks: term-introduction-discipline

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/term-introduction-discipline/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- `author-change`: generated guidance is synchronized from canonical source
  assets; generated guidance may name only commands that work.
- Support-fragment capabilities (`ideation-skill`, `domain-skill-trigger`) keep
  their fragment ownership and carry no independent user goal or inventory.
- Confirmed Personas: `agent--technical-role` (generated guidance) and
  `givn-maintainer--owner` (rule corpus, term convention). Actors in Gherkin
  stay the Developer and the Authoring agent.
- A mandatory check is never silently deferred.

Capabilities and step files:

- `ideation-skill` — reused generic steps in `tests/steps/prose_consolidation_steps.rs`
- `domain-skill-trigger` — `tests/steps/fix_domain_modeling_skill_trigger_steps.rs`: one new When and four new Then steps

No scenario in this change is tagged `@e2e`: the actions (synchronize skills,
route the domain skill) are already proven end-to-end, and the new scenarios
are guidance-content variants.

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
      `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` with
      `unimplemented!()` bodies; `cargo check --locked --tests` succeeds.
      Paste the output:
      ```
      ```
- [ ] COMMIT: `<hash>` -- `test(term-introduction-discipline): strict-mode proof and step skeletons`

## Scenarios

### ideation-skill

- [ ] Scenario: `The ideation guidance carries the term schema and the collision procedure`
  - [ ] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the term schema, status rules, escaping rule, the
        three-step collision procedure, the quality-review branch, and the
        Handoff rule to `assets/skills/givn-ideation/SKILL.md.tmpl`; added the
        pointer to `docs/workflow.md`. Files:
        `assets/skills/givn-ideation/SKILL.md.tmpl`, `docs/workflow.md`.
        `cargo clean -p givn`, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(ideation-skill): The ideation guidance carries the term schema and the collision procedure`

### domain-skill-trigger

- [ ] Scenario: `The domain-modeling guidance carries the term introduction test`
  - [ ] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: added the term introduction test (genus + differentia, contrast
        pair, one concrete domain sentence, the Substrate failure, and the
        folded generic-word set) to
        `assets/skills/givn-domain-modeling/SKILL.md.tmpl`. Files:
        `assets/skills/givn-domain-modeling/SKILL.md.tmpl`. `cargo clean -p
        givn`, then the targeted run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `feat(domain-skill-trigger): The domain-modeling guidance carries the term introduction test`

## Final checks

- [ ] Run the full suite (`./run-tests.sh`) → zero exit. Evidence:
      ```
      ```
- [ ] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      ```
- [ ] Run the change delta suite
      (`GIVN_FEATURES=givn/changes/term-introduction-discipline/specs cargo test --locked --test features_runner`)
      → zero exit. Evidence:
      ```
      ```
- [ ] Grep every touched step-definition file for empty/no-op bodies → 0.
      Evidence:
      ```
      ```
- [ ] Confirm commit count matches scenario task count and every task has a
      recorded hash.
- [ ] Confirm `givn lint --change term-introduction-discipline` has no parse
      errors (exit 0 or 2).
