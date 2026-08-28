# Tasks: fix-domain-modeling-skill-trigger

## Setup

- [x] Configure and prove the existing Gherkin runner for this change.
  - Runner: `GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner`
  - E2E runner: `./run-tests.sh --e2e`
  - Strict mode: `GivnWorld::cucumber().fail_on_skipped()` (`tests/features_runner.rs`).
  - Stub: Rust `unimplemented!()`.
  - Step files: `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` and
    `tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs` (one per
    capability, registered in `tests/steps/mod.rs`).
  - Proof command and non-zero output (strict mode already proven in prior
    changes; a fresh undefined-step run fails):
    ```text
    GIVN_FEATURES=givn/changes/fix-domain-modeling-skill-trigger/specs \
    GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub \
    cargo test --features spec-retrieval --test features_runner
    ...
    1 feature, 10 scenarios (10 failed), ... step(s) failed
    Step doesn't match any function
    ```
  - Full/e2e scenario-count proof (e2e count strictly below full count):
    ```text
    Full scope: 10 scenarios
    E2E: 4 scenarios
    Non-E2E: 6 scenarios
    E2E count is strictly smaller than full count. Verify commands:
    verify.command     : ./run-tests.sh
    verify.e2e_command : ./run-tests.sh --e2e
    ```

## Non-E2E Scenarios

### The skill trigger names conversation states

- [x] RED: target `The skill trigger names conversation states`; paste
  non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 3 steps (2 passed, 1 failed)
  Step panicked. Captured output: not implemented: trigger-loads-while-naming
  ```
- [x] GREEN: replace `assets/skills/givn-domain-modeling/SKILL.md.tmpl` frontmatter
  description and step assertions; list files touched; paste zero-exit
  targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  Files: assets/skills/givn-domain-modeling/SKILL.md.tmpl,
  tests/steps/fix_domain_modeling_skill_trigger_steps.rs,
  tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs,
  tests/steps/mod.rs, givn/changes/fix-domain-modeling-skill-trigger/*
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  ```
- [x] COMMIT: `5f286e1` - `feat(domain-skill-trigger): The skill trigger names conversation states`

### The skill body is an active discipline

- [x] RED: target `The skill body is an active discipline`; paste non-zero
  runner output.
  ```text
  1 feature, 1 scenario (1 failed), 3 steps (2 passed, 1 failed)
  Step panicked. Captured output: not implemented: body-challenges-generic-terms
  ```
- [x] GREEN: complete the skill body rework and step assertions; paste
  zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  ```
- [x] COMMIT: `409728c` - `feat(domain-skill-trigger): The skill body is an active discipline`

### Lifecycle guidance describes the domain, not the givn change

- [x] RED: target `Lifecycle guidance describes the domain, not the givn change`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 3 steps (2 passed, 1 failed)
  Step panicked. Captured output: not implemented: lifecycle-describes-domain-states
  ```
- [x] GREEN: fix the lifecycle section in the skill template and step
  assertions; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] COMMIT: `18fce48` - `feat(domain-skill-trigger): Lifecycle guidance describes the domain, not the givn change`

### The skill documents a per-change ubiquitous-language table

- [x] RED: target `The skill documents a per-change ubiquitous-language table`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 3 steps (2 passed, 1 failed)
  Step panicked. Captured output: not implemented: body-requires-ul-table
  ```
- [x] GREEN: complete the domain.md / anti-term guidance in the skill template
  and step assertions; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] COMMIT: `046e382` - `feat(domain-skill-trigger): The skill documents a per-change ubiquitous-language table`

### An agent requests next-step guidance as JSON and the skill route is machine-readable

- [x] RED: target `An agent requests next-step guidance as JSON and the skill route is machine-readable`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 3 steps (2 passed, 1 failed)
  Step panicked. Captured output: not implemented: json-response-names-domain-skill
  ```
- [x] GREEN: add `skill_to_load` to the generic JSON report in
  `src/cli/instructions.rs`; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  Files: src/cli/instructions.rs,
  tests/steps/fix_domain_modeling_skill_trigger_steps.rs
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  ```
- [x] COMMIT: `20f22d5` - `feat(domain-skill-trigger): An agent requests next-step guidance as JSON and the skill route is machine-readable`

### A project without the addon is not routed to the domain skill

- [x] RED: target `A project without the addon is not routed to the domain skill`;
  the guard first captured only `not implemented: guidance-does-not-mention-domain-skill`.
  Once `run_artifact` shipped (scenario 8), the full suite caught a false green:
  `make_init_root()` enables all addons, so the fixture still had
  `domain_modeling: true` and the specs guidance legitimately named the
  domain skill.
  ```text
  1 feature, 1 scenario (1 failed), 3 steps (2 passed, 1 failed)
  Step panicked. Captured output: guidance mentions the domain-modeling skill with the addon disabled
  ```
- [x] GREEN: disable the addon in the fixture (`givn addons disable
  domain_modeling`) so the guard exercises a genuinely addon-free project;
  paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  Files: tests/steps/fix_domain_modeling_skill_trigger_steps.rs (fixture),
  src/cli/instructions.rs (run_artifact, see scenario 8)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  ```
- [x] COMMIT: `c83042b` - `fix(domain-skill-trigger): Disable the addon in the addon-off fixture`
  (constraint-commit; the original `8758532` pass was issued before `run_artifact`
  routed on the addon flag)

## E2E Scenarios

### An agent synchronizes skills and the domain skill is routed

- [x] RED: target `An agent synchronizes skills and the domain skill is routed`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 2 steps (1 passed, 1 failed)
  Step panicked. Captured output: not implemented: agent-synchronizes-skills
  ```
- [x] GREEN: e2e steps enabling the addon, syncing skills, and asserting the
  generated skill; paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 5 steps (5 passed)
  ```
- [x] COMMIT: `1f5b082` - `feat(domain-skill-trigger): An agent synchronizes skills and the domain skill is routed`

### An agent requests next-step guidance and is routed to the domain skill

- [x] RED: target `An agent requests next-step guidance and is routed to the domain skill`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 2 steps (1 passed, 1 failed)
  Step panicked. Captured output: not implemented: agent-requests-next-step-guidance
  ```
- [x] GREEN: emit the router line in `src/cli/instructions.rs` generic path;
  paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  Files: src/cli/instructions.rs,
  tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs,
  tests/steps/fix_domain_modeling_skill_trigger_steps.rs
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  ```
- [x] COMMIT: `a3d3fed` - `feat(domain-skill-trigger): An agent requests next-step guidance and is routed to the domain skill`

### An agent requests the specs guidance and is routed to the domain skill

- [x] RED: target `An agent requests the specs guidance and is routed to the domain skill`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 3 steps (2 passed, 1 failed)
  Step panicked. Captured output: guidance does not say to load the domain-modeling skill now
  ```
- [x] GREEN: emit the router line in the artifact path for `specs`; paste
  zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  Files: src/cli/instructions.rs (run_artifact)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  ```
- [x] COMMIT: `b00c741` - `feat(domain-skill-trigger): An agent requests the specs guidance and is routed to the domain skill`

### An agent requests the design guidance and is routed to the domain skill

- [x] RED: target `An agent requests the design guidance and is routed to the domain skill`;
  paste non-zero runner output.
  ```text
  1 feature, 1 scenario (1 failed), 2 steps (1 passed, 1 failed)
  Step panicked. Captured output: not implemented: agent-requests-design-instructions
  ```
- [x] GREEN: emit the router line in the artifact path for `design`; paste
  zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  Files: tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs
  (artifact path for design covered by the run_artifact change)
  ```
- [x] REFACTOR: paste zero-exit targeted output.
  ```text
  1 feature, 1 scenario (1 passed), 3 steps (3 passed)
  ```
- [x] COMMIT: `e2e511a` - `feat(domain-skill-trigger): An agent requests the design guidance and is routed to the domain skill`

## Done

- [ ] Full suite: `./run-tests.sh` exits 0.
- [ ] E2E suite: `./run-tests.sh --e2e` exits 0.
- [ ] `givn lint --change fix-domain-modeling-skill-trigger` exits 0 or 2.
- [ ] Self-dogfood: `cargo install --path . --locked --force`, `givn skills sync`,
      and the generated `.opencode/skills/givn-domain-modeling/SKILL.md` has the
      new trigger text and lifecycle section.
