# Tasks: domain-modeling-glossary-enforcement

## Setup

- [ ] Configure and prove the existing Gherkin runner for this change.
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

- [ ] RED: target `The skill trigger names conversation states`; paste
  non-zero runner output.
  ```text
  (to fill)
  ```
- [ ] GREEN: make the trigger description cover every workflow step of a
  change (keep the domain-concept and term-disagreement triggers); paste
  zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] REFACTOR: paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] COMMIT: `<hash>` - `feat(domain-modeling-glossary): <title>`

### The ubiquitous language is recorded in the project's arc42 glossary

- [ ] RED: target `The ubiquitous language is recorded in the project's arc42
  glossary`; paste non-zero runner output.
  ```text
  (to fill)
  ```
- [ ] GREEN: retarget the skill body from `domain.md` to
  `docs/arc42/12-glossary.md` (UL table, aggregates/invariants); paste
  zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] REFACTOR: paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] COMMIT: `<hash>` - `feat(domain-modeling-glossary): <title>`

### The skill requires the arc42 glossary for domain modeling

- [ ] RED: target `The skill requires the arc42 glossary for domain modeling`;
  paste non-zero runner output.
  ```text
  (to fill)
  ```
- [ ] GREEN: add the "requires the arc42 addon; enable it first" instruction
  to the skill body; paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] REFACTOR: paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] COMMIT: `<hash>` - `feat(domain-modeling-glossary): <title>`

### The next-step guidance names the domain skill during later change phases

- [ ] RED: target `The next-step guidance names the domain skill during later
  change phases`; paste non-zero runner output (fixture stages the change to
  awaiting-review; generic next-step currently stays silent at that stage).
  ```text
  (to fill)
  ```
- [ ] GREEN: change `domain_skill_to_load` to the
  `(domain_modeling: bool, arc42: bool) -> Option<&'static str>` predicate
  (names the skill for any workflow step when both addons are on), widen
  `report_change` (new `arc42` parameter, 3 call sites in
  `run_generic`), update `run_artifact`, and update the `domain_skill_to_load`
  unit tests; paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] REFACTOR: paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] COMMIT: `<hash>` - `feat(domain-modeling-glossary): <title>`

### A project without the arc42 glossary is not pointed at the domain skill

- [ ] RED: target `A project without the arc42 glossary is not pointed at the
  domain skill`; paste non-zero runner output (guard scenario; passes before
  and after the router change).
  ```text
  (to fill)
  ```
- [ ] GREEN: assert the coupling contract (domain_modeling on, arc42 off →
  no skill pointer) with the already-wired predicate and the disable-arc42
  fixture; paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] REFACTOR: paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] COMMIT: `<hash>` - `feat(domain-modeling-glossary): <title>`

### The design-review skill checks that the ubiquitous language is followed

- [ ] RED: target `The design-review skill checks that the ubiquitous language
  is followed`; paste non-zero runner output.
  ```text
  (to fill)
  ```
- [ ] GREEN: add the mandatory ubiquitous-language check to the
  `givn-design-review` skill template; paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] REFACTOR: paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] COMMIT: `<hash>` - `feat(domain-modeling-glossary): <title>`

### The review skill checks that the ubiquitous language is followed

- [ ] RED: target `The review skill checks that the ubiquitous language is
  followed`; paste non-zero runner output.
  ```text
  (to fill)
  ```
- [ ] GREEN: add the mandatory ubiquitous-language check to the `givn-review`
  skill template; paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] REFACTOR: paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] COMMIT: `<hash>` - `feat(domain-modeling-glossary): <title>`

## E2E Scenarios

### An agent synchronizes skills and the domain skill is routed

- [ ] RED: target `An agent synchronizes skills and the domain skill is
  routed`; paste non-zero runner output.
  ```text
  (to fill)
  ```
- [ ] GREEN: assert the generated skill loads during every workflow step and
  instructs maintaining the project's arc42 glossary (e2e assertions on the
  file generated by `givn skills sync`); paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] REFACTOR: paste zero-exit targeted output.
  ```text
  (to fill)
  ```
- [ ] COMMIT: `<hash>` - `feat(domain-modeling-glossary): <title>`

## Removed Scenarios

### The skill documents a per-change ubiquitous-language table

- [ ] Superseded by `The ubiquitous language is recorded in the project's
  arc42 glossary` (same topic, durable-artifact upgrade). Merge removes the
  scenario by title; the delta placeholder step is covered by the shared
  `Given a placeholder removed scenario` step.

## Done

- [ ] Full suite: `./run-tests.sh` exits 0 (change-scoped full
  `GIVN_FEATURES=givn/changes/domain-modeling-glossary-enforcement/specs ./run-tests.sh`).
- [ ] E2E suite: `./run-tests.sh --e2e` exits 0 (change-scoped e2e).
- [ ] `givn lint --change domain-modeling-glossary-enforcement` exits 0 or 2.
- [ ] Self-dogfood: `cargo install --path . --locked --force`, `givn skills sync`,
      and the generated `.opencode/skills/givn-domain-modeling/SKILL.md` has the
      whole-lifecycle trigger, the arc42-glossary instruction, and no per-change
      `domain.md` requirement.
