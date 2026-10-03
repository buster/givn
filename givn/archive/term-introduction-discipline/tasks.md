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
      `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` (one When, four
      Then) with `unimplemented!()` bodies. Evidence:
      ```
      cargo check --locked --tests
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.54s
      ```
- [x] COMMIT: `dd0c4f0d` -- `test(term-introduction-discipline): strict-mode proof and step skeletons`

## Scenarios

### ideation-skill

- [x] Scenario: `The ideation guidance carries the term schema and the collision procedure`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/term-introduction-discipline/specs cargo test --locked --test features_runner -- --name "The ideation guidance carries the term schema and the collision procedure"
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: added the term schema, status rules, escaping rule, the
        three-step collision procedure, the quality-review branch, and the
        Handoff rule to `assets/skills/givn-ideation/SKILL.md.tmpl`; added the
        pointer to `docs/workflow.md`. Files:
        `assets/skills/givn-ideation/SKILL.md.tmpl`, `docs/workflow.md`.
        `cargo clean -p givn`, then the targeted run → zero. Evidence:
        ```
        ✔ Then the generated givn-ideation guidance contains "domain-terms.md"
        ✔ And ... "`candidate`, `confirmed`, or `rejected`"
        ✔ And ... "Contrast"
        ✔ And ... "First use"
        ✔ And ... "definition and first use"
        ✔ And ... "recorded collision check"
        ✔ And ... "only `confirmed` terms"
        ✔ And ... "never deleted"
        ✔ And ... "givn-domain-modeling"
        ✔ And ... "Collision check"
        ✔ And ... "Grep the durable glossary"
        ✔ And ... "without recorded commands is not a record"
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
  - [x] COMMIT: `79c3fa87` -- `feat(ideation-skill): The ideation guidance carries the term schema and the collision procedure`

### domain-skill-trigger

- [x] Scenario: `The domain-modeling guidance carries the term introduction test`
  - [x] RED: removed `@wip` from this scenario only; run → non-zero. Evidence:
        ```
        GIVN_FEATURES=givn/changes/term-introduction-discipline/specs cargo test --locked --test features_runner -- --name "The domain-modeling guidance carries the term introduction test"
        Step panicked. Captured output: not implemented
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        ```
  - [x] GREEN: added the term introduction test (genus + differentia, contrast
        pair, one concrete domain sentence, the Substrate failure, and the
        folded generic-word set) to
        `assets/skills/givn-domain-modeling/SKILL.md.tmpl`; implemented the
        five new steps. Files:
        `assets/skills/givn-domain-modeling/SKILL.md.tmpl`,
        `tests/steps/fix_domain_modeling_skill_trigger_steps.rs`. `cargo clean
        -p givn`, then the targeted run → zero. Evidence:
        ```
        ✔ Given the domain-modeling skill template is inspected
        ✔ When an agent reads the term introduction test
        ✔ Then the body requires a genus and differentia definition
        ✔ And the body requires a contrast pair
        ✔ And the body requires one concrete domain sentence
        ✔ And the body demonstrates a failure when the genus slot cannot be filled
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
  - [x] COMMIT: `0ecf09ad` -- `feat(domain-skill-trigger): The domain-modeling guidance carries the term introduction test`

## Final checks

- [x] Run the full suite (`./run-tests.sh`) → zero exit. The first run caught
      the permanent template-conditional-rendering `@e2e` contract: the
      ideation skill must not name a disabled addon. Fixed by wrapping the
      glossary step and the term-test reference in their addon conditionals
      (commit `7ec71223`). Evidence:
      ```
      Summary [27.563s] 505 tests run: 505 passed, 0 skipped
      ```
- [x] Run the e2e suite (`./run-tests.sh --e2e`) → zero exit. Evidence:
      ```
      Summary [5.149s] 88 tests run: 88 passed, 417 skipped
      ```
- [x] Run the change delta suite
      (`GIVN_FEATURES=givn/changes/term-introduction-discipline/specs cargo test --locked --test features_runner`)
      → zero exit. Evidence:
      ```
      2 features
      2 scenarios (2 passed)
      20 steps (20 passed)
      ```
- [x] Grep every touched step-definition file for empty/no-op bodies → 0.
      Evidence:
      ```
      grep -n "unimplemented!\|todo!()" tests/steps/fix_domain_modeling_skill_trigger_steps.rs
      (no matches)
      ```
- [x] Confirm commit count matches scenario task count and every task has a
      recorded hash: one setup commit (`dd0c4f0d`), two scenario commits
      (`79c3fa87`, `0ecf09ad`), and one full-suite fix commit (`7ec71223`).
- [x] Confirm `givn lint --change term-introduction-discipline` has no parse
      errors (exit 0). Evidence:
      ```
      givn lint: 2 file(s) checked — clean
      exit 0
      ```
