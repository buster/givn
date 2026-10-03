# Tasks: domain-modeling-glossary-enforcement

## Setup

- [x] Configure and prove the existing Gherkin runner for this change.
  - Runner: `GIVN_FEATURES=givn/changes/domain-modeling-glossary-enforcement/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled cargo test --no-default-features --features spec-retrieval --test features_runner`
  - E2E runner: `./run-tests.sh --e2e`
  - Strict mode: `GivnWorld::cucumber().fail_on_skipped()` (`tests/features_runner.rs`).
  - Stub: Rust `unimplemented!()`.
  - Step files: extend `tests/steps/fix_domain_modeling_skill_trigger_steps.rs`
    and `tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs` (one per
    capability, already registered in `tests/steps/mod.rs`).
  - Add the shared removed-scenario placeholder step
    `Given a placeholder removed scenario` (a real assertion body, never empty)
    to the regular capability step file.
  - Proof command and non-zero output (strict mode already proven in prior
    changes; an undefined step in the delta fails):
    ```text
    GIVN_FEATURES=givn/changes/domain-modeling-glossary-enforcement/specs \
    GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled \
    cargo test --no-default-features --features spec-retrieval --test features_runner
    ...
    Step doesn't match any function
    ```
  - Full/e2e scenario-count proof:
    ```text
    Full scope: 9 scenarios (1 removed placeholder out of parse scope)
    E2E: 1 scenario
    Non-E2E: 8 scenarios
    E2E count is strictly smaller than full count. Verify commands:
    verify.command     : ./run-tests.sh
    verify.e2e_command : ./run-tests.sh --e2e
    ```

## Non-E2E Scenarios

### The skill trigger names conversation states

- [x] RED: target `The skill trigger names conversation states`; paste
  non-zero runner output.
  ```text
  Then the trigger loads while naming or discussing domain concepts  (passed)
  And the trigger loads during every workflow step of a change  (failed)
  Step panicked. Captured output: not implemented: trigger-loads-during-every-workflow-step
  1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed)
  ```
- [x] GREEN: make the trigger description cover every workflow step of a
  change (keep the domain-concept and term-disagreement triggers); paste
  zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 5 steps (5 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 5 steps (5 passed)  (unchanged after edit)
  ```
- [x] COMMIT: `a4dde5c` - `fix(domain-modeling-glossary): Trigger the domain skill for every workflow step`

### The ubiquitous language is recorded in the project's arc42 glossary

- [x] RED: target `The ubiquitous language is recorded in the project's arc42
  glossary`; paste non-zero runner output.
  ```text
  Then the body instructs maintaining the project's arc42 glossary  (failed)
  Step panicked. Captured output: not implemented: body-instructs-maintaining-glossary
  1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed)
  ```
- [x] GREEN: retarget the skill body from `domain.md` to
  `docs/arc42/12-glossary.md` (UL table, aggregates/invariants); paste
  zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 4 steps (4 passed)
  Permanent 'The skill body is an active discipline' still 5/5 (template keeps
  'Challenge generic or overloaded terms', 'Name aggregates with domain nouns',
  inline-update discipline, 'the *domain's own*' + 'trip reset', no givn lifecycle).
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 4 steps (4 passed)  (unchanged)
  ```
- [x] COMMIT: `0abf991` - `feat(domain-modeling-glossary): Record the vocabulary in the project's arc42 glossary`

### The skill requires the arc42 glossary for domain modeling

- [x] RED: target `The skill requires the arc42 glossary for domain modeling`;
  paste non-zero runner output.
  ```text
  Then the body instructs enabling the arc42 addon first...  (failed)
  Step panicked. Captured output: not implemented: body-instructs-enabling-arc42
  1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed)
  ```
- [x] GREEN: add the "requires the arc42 addon; enable it first" instruction
  to the skill body; paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 3 steps (3 passed)  (unchanged)
  ```
- [x] COMMIT: `2f4e77f` - `feat(domain-modeling-glossary): Require the arc42 addon for domain modeling`

### The next-step guidance names the domain skill during later change phases

- [x] RED: target `The next-step guidance names the domain skill during later
  change phases`; paste non-zero runner output (fixture stages the change to
  awaiting-review; generic next-step currently stays silent at that stage).
  ```text
  Given a Givn project with the domain model addon enabled and a change
  awaiting review  (failed)
  Step doesn't match any function
  1 feature / 1 scenario (1 failed)
  ```
- [x] GREEN: change `domain_skill_to_load` to the
  `(domain_modeling: bool, arc42: bool) -> Option<&'static str>` predicate
  (names the skill for any workflow step when both addons are on), widen
  `report_change` (new `arc42` parameter, 3 call sites in
  `run_generic`), update `run_artifact`, and update the `domain_skill_to_load`
  unit tests; paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 3 steps (3 passed)
  cargo test --lib instructions: 9 passed (old 2 unit tests replaced)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 3 steps (3 passed)  (unchanged)
  ```
- [x] COMMIT: `cecdb3f` - `feat(domain-modeling-glossary): Route the domain skill through every workflow step when the arc42 addon is enabled`

### A project without the arc42 glossary is not pointed at the domain skill

- [x] RED: target `A project without the arc42 glossary is not pointed at the
  domain skill`; paste non-zero runner output (guard scenario; passes before
  and after the router change).
  ```text
  Given a Givn project with the domain model addon enabled but no arc42
  glossary and a change awaiting specs  (failed)
  Step doesn't match any function
  ```
- [x] GREEN: assert the coupling contract (domain_modeling on, arc42 off →
  no skill pointer) with the already-wired predicate and the disable-arc42
  fixture; paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 3 steps (3 passed)  (unchanged)
  ```
- [x] COMMIT: `87eb54b` - `feat(domain-modeling-glossary): Guard the domain skill behind the arc42 glossary`

### The design-review skill checks that the ubiquitous language is followed

- [x] RED: target `The design-review skill checks that the ubiquitous language
  is followed`; paste non-zero runner output.
  ```text
  Then a check requires confirming the ubiquitous language is used
  consistently and recorded in the arc42 glossary  (failed)
  Step panicked. Captured output: not implemented: design-review-checks-ubiquitous-language
  1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed)
  ```
- [x] GREEN: add the mandatory ubiquitous-language check to the
  `givn-design-review` skill template; paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 3 steps (3 passed)  (unchanged)
  ```
- [x] COMMIT: `6b65c44` - `feat(domain-modeling-glossary): Design-review checks the ubiquitous language against the arc42 glossary`

### The review skill checks that the ubiquitous language is followed

- [x] RED: target `The review skill checks that the ubiquitous language is
  followed`; paste non-zero runner output.
  ```text
  Then a check requires confirming the ubiquitous language is used
  consistently everywhere and recorded in the arc42 glossary  (failed)
  Step panicked. Captured output: not implemented: review-checks-ubiquitous-language
  1 feature / 2 scenarios within filter (1 failed) / 6 steps (5 passed, 1 failed)
  ```
- [x] GREEN: add the mandatory ubiquitous-language check to the `givn-review`
  skill template; paste zero-exit targeted output.
  ```text
  2 scenarios (design-review + review) / 6 steps (6 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  unchanged
  ```
- [x] COMMIT: `db65a88` - `feat(domain-modeling-glossary): Review checks the ubiquitous language against the arc42 glossary`

## E2E Scenarios

### An agent synchronizes skills and the domain skill is routed

- [x] RED: target `An agent synchronizes skills and the domain skill is
  routed`; paste non-zero runner output.
  ```text
  And the generated skill loads during every workflow step of a change  (failed)
  Step panicked. Captured output: not implemented: generated-skill-loads-during-every-workflow-step
  1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed)
  ```
- [x] GREEN: assert the generated skill loads during every workflow step and
  instructs maintaining the project's arc42 glossary (e2e assertions on the
  file generated by `givn skills sync`); paste zero-exit targeted output.
  ```text
  1 feature / 1 scenario (1 passed) / 6 steps (6 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  unchanged
  ```
- [x] COMMIT: `a0c2905` - `feat(domain-modeling-glossary): E2E asserts the generated skill covers every step and the glossary`

## Removed Scenarios

### The skill documents a per-change ubiquitous-language table

- [x] Superseded by `The ubiquitous language is recorded in the project's
  arc42 glossary` (same topic, durable-artifact upgrade). Merge removes the
  scenario by title; the delta placeholder step is covered by the shared
  `Given a placeholder removed scenario` step.

## Done

- [x] Full suite: `./run-tests.sh` exits 0 (change-scoped full
  `GIVN_FEATURES=givn/changes/domain-modeling-glossary-enforcement/specs ./run-tests.sh`).
  ```text
  9 scenarios (9 passed) / 31 steps (31 passed)
  ```
- [x] E2E suite: `./run-tests.sh --e2e` exits 0 (change-scoped e2e).
  ```text
  1 scenario (1 passed) / 6 steps (6 passed)
  ```
- [x] `givn lint --change domain-modeling-glossary-enforcement` exits 0 or 2.
  ```text
  givn lint: 1 file(s) checked — clean (exit 0)
  ```
- [x] Self-dogfood: `cargo install --path . --locked --force`, `givn skills sync`,
      and the generated `.opencode/skills/givn-domain-modeling/SKILL.md` has the
      whole-lifecycle trigger, the arc42-glossary instruction, and no per-change
      `domain.md` requirement.
  ```text
  generated SKILL.md line 8: "during every workflow step of a change"
  line 47: "Requires the arc42 addon - enable it first when the glossary is missing"
  line 55: "maintaining the project's arc42 glossary inline ..."
  no "domain.md" anywhere
  live router: arc42 off -> no skill; arc42 on -> skill at specs and at review
  ```
  Also: `cargo test --lib` 166 passed; `cargo clippy --workspace --all-targets -D warnings` clean; `cargo fmt --all` clean.
