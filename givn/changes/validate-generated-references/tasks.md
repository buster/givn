# Tasks: validate-generated-references

## Context

- Use case: fragment `support` — capabilities `instruction-integrity` and
  `generate-ideation-guidance`. The shared guarantee: generated guidance
  stays complete and trustworthy for its reader; a dead reference is an
  incomplete instruction.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner` (review context only; Gherkin actors stay
  distinct).
- Verify command: `./run-tests.sh`; change scope:
  `GIVN_FEATURES=givn/changes/validate-generated-references/specs`.
- E2E: no delta `@e2e` scenario exists; `verify.e2e_command`
  (`./run-tests.sh --e2e`) is unchanged; the e2e setup task is not
  applicable.
- Design: `givn/changes/validate-generated-references/design.md`.
- Interaction coverage cross-reference: no `.feature` file in this
  repository carries a `# User Interaction Inventory:` comment block; the
  canonical policy keeps the inventory in the fragment document. The
  fragment's two `instruction-integrity` rows and the annotated
  `generate-ideation-guidance` sync row all map to the design's matrix with
  real `givn` subprocesses, and every referenced `@e2e` title exists in the
  permanent specs.

## Setup task — strict mode and skeletons

- [x] **S1. Prove strict mode and create the step skeletons.**
  - Runner config: none required — `./run-tests.sh` already drives
    `tests/features_runner.rs` with `.fail_on_skipped()`, and
    `givn/commands.yaml` already sets `verify.command: ./run-tests.sh`.
  - Skeletons: add nine new steps in
    `tests/steps/instruction_integrity_steps.rs` (one When, two regex
    Givens, six Thens) with `unimplemented!()` bodies. The shared
    classifier module `tests/common/reference_check.rs` is created in
    Task 3, its first consumer.
  - Proof-of-strictness (stub panic must fail the run):
    `GIVN_FEATURES=givn/changes/validate-generated-references/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::The characterization guidance requests the reverse review from Givn"`
  - Expected: non-zero exit.
  - Captured output:
    ```
    exit=101
    Feature: Instruction and skill guidance remains complete and machine-readable
      Scenario: The characterization guidance requests the reverse review from Givn
       ✔  Given a givn project with all agent skill targets enabled
       ✔  When the user runs `givn skills sync`
       ✘  Then the generated givn-characterize guidance requests `givn instructions review-reverse`
          Step panicked. Captured output: not implemented
    [Summary]
    1 feature
    1 scenario (1 failed)
    3 steps (2 passed, 1 failed)
    error: test failed, to rerun pass `--test features_runner`
    ```

## Task 1 — The ideation guidance carries the persona schema (`generate-ideation-guidance`)

- [x] RED — remove `@wip` from this scenario only; run the single-scenario
  command; expect non-zero (the capitalization assertions fail today).
  - Command: `GIVN_FEATURES=givn/changes/validate-generated-references/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the persona schema"`
  - Output: `exit=101` — `✘ Then the generated givn-ideation guidance contains "Stable core"` → `Step panicked. Captured output: givn-ideation guidance missing "Stable core"`; `1 scenario (1 failed)`.
- [x] GREEN — replace the pointer paragraph in
  `assets/skills/givn-ideation/SKILL.md.tmpl` with the inline schema block
  from `design.md`; re-run; expect zero.
  - Files: `assets/skills/givn-ideation/SKILL.md.tmpl`.
  - Output: `exit=0` — all 8 assertions pass ("Stable core" through "Topic stances"); `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] REFACTOR — re-run; expect zero.
  - Output: `exit=0` — `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] COMMIT — one commit for RED+GREEN+REFACTOR;
  `feat(generate-ideation-guidance): <release note>` with the scenario title
  in the body.
  - Hash: `6224ef7f`

## Task 2 — The characterization guidance requests the reverse review from Givn (`instruction-integrity`)

- [x] RED — remove `@wip` from this scenario only; run; expect non-zero
  (stub panic).
  - Command: `GIVN_FEATURES=givn/changes/validate-generated-references/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::The characterization guidance requests the reverse review from Givn"`
  - Output: `exit=101` — `✘ Then the generated givn-characterize guidance requests \`givn instructions review-reverse\`` → `Step panicked. Captured output: not implemented`; `1 scenario (1 failed)`.
- [x] GREEN — implement the two Then steps; edit
  `assets/skills/givn-characterize/SKILL.md.tmpl` (one occurrence) and
  `assets/commands/givn-characterize.md` (two occurrences) to
  `` `givn instructions review-reverse` ``; re-run; expect zero.
  - Files: `tests/steps/instruction_integrity_steps.rs`,
    `assets/skills/givn-characterize/SKILL.md.tmpl`,
    `assets/commands/givn-characterize.md`.
  - Output: `exit=0` — both assertions pass; `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR — re-run; expect zero.
  - Output: `exit=0` — `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] COMMIT — `feat(instruction-integrity): <release note>` with the
  scenario title in the body.
  - Hash: `e9d34960`

## Task 3 — The generated guidance references resolve with the default configuration (`instruction-integrity`)

- [x] RED — remove `@wip` from this scenario only; run; expect non-zero
  (stub panic).
  - Command: `GIVN_FEATURES=givn/changes/validate-generated-references/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::The generated guidance references resolve with the default configuration"`
  - Output: `exit=101` — `✘ Then every guidance reference resolves …` → `Step panicked. Captured output: not implemented`; `1 scenario (1 failed)`.
- [x] GREEN — implement the classifier (`tests/common/reference_check.rs`)
  and the installed-corpus Then step: extraction, trailing-slash
  normalization, exists class, workflow-created table, rule-owned table
  (`docs/design` only in this task), findings. Land the remaining
  installed-corpus content fixes: drop the `docs/use-case-model.md` clause
  in `assets/skills/givn-ideation/SKILL.md.tmpl` and
  `assets/skills/givn-spec/SKILL.md.tmpl`; reword the ADR example in
  `assets/commands/grill-with-docs.md`. Re-run; expect zero.
  - Files: `tests/common/reference_check.rs`, `tests/common/mod.rs`,
    `tests/steps/instruction_integrity_steps.rs`,
    `assets/skills/givn-ideation/SKILL.md.tmpl`,
    `assets/skills/givn-spec/SKILL.md.tmpl`,
    `assets/commands/grill-with-docs.md`.
  - Output: first GREEN run failed on four findings — `givn/personas/` in
    the ideation guidance of `.agents`, `.claude`, and `.opencode`, plus the
    `.opencode` command — because the bare directory candidate had no table
    leaf; `givn/personas` gained its empty leaf. Re-run: `exit=0`,
    `1 scenario (1 passed)`, `3 steps (3 passed)`.
- [x] REFACTOR — re-run; expect zero.
  - Output: `exit=0` — the directory walk was split into
    `collect_skill_files`/`collect_command_files`/`push_source`;
    `1 scenario (1 passed)`, `3 steps (3 passed)`.
- [x] COMMIT — `feat(instruction-integrity): <release note>` with the
  scenario title in the body.
  - Hash: `054825cf`

## Task 4 — The generated guidance references resolve without addons (`instruction-integrity`)

- [x] RED — remove `@wip` from this scenario only; run; expect non-zero: in
  an opencode-only project `.agents/skills/givn-steps/SKILL.md` does not
  exist and the by-name resolution rule is not implemented yet.
  - Command: `GIVN_FEATURES=givn/changes/validate-generated-references/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::The generated guidance references resolve without addons"`
  - Output: `exit=101` — two findings: `.opencode/commands/givn-characterize.md:98` and `.opencode/commands/givn-implement.md:103`, both `.agents/skills/givn-steps/SKILL.md`; `1 scenario (1 failed)`.
- [x] GREEN — add the agent-target skill-file resolution rule (resolve the
  skill by name across the configured targets); re-run; expect zero.
  - Files: `tests/common/reference_check.rs`.
  - Output: `exit=0` — `1 scenario (1 passed)`, `3 steps (3 passed)`.
- [x] REFACTOR — re-run; expect zero.
  - Output: `exit=0` — `1 scenario (1 passed)`, `3 steps (3 passed)`.
- [x] COMMIT — `feat(instruction-integrity): <release note>` with the
  scenario title in the body.
  - Hash: `debeb675`

## Task 5 — The printed instructions reference only resolvable files (`instruction-integrity`)

- [x] RED — remove `@wip` from this scenario only; run; expect non-zero
  (stub panic on the printed-corpus When step).
  - Command: `GIVN_FEATURES=givn/changes/validate-generated-references/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::The printed instructions reference only resolvable files"`
  - Output: `exit=101` — `✘ When the author requests every instruction Givn prints` → `Step panicked. Captured output: not implemented`; `1 scenario (1 failed)`.
- [x] GREEN — implement the When step (manifest artifacts from
  `givn graph --json` plus `review-reverse`, `specs-reverse`,
  `tasks-reverse`, `dialog`) and the printed Then step; add the
  `docs/arc42/adr` rule-owned rows; drop the `docs/use-case-model.md`
  clause in `assets/instructions/specs.md` and the durable-reference
  sentence in `assets/instructions/review.md`. Re-run; expect zero.
  - Files: `tests/steps/instruction_integrity_steps.rs`,
    `tests/common/reference_check.rs`, `assets/instructions/specs.md`,
    `assets/instructions/review.md`.
  - Output: `exit=0` — `1 scenario (1 passed)`, `3 steps (3 passed)`.
- [x] REFACTOR — re-run; expect zero.
  - Output: `exit=0` — `1 scenario (1 passed)`, `3 steps (3 passed)`.
- [x] COMMIT — `feat(instruction-integrity): <release note>` with the
  scenario title in the body.
  - Hash: `d0b83eb7`

## Task 6 — An undeclared reference in an installed skill fails the check (`instruction-integrity`)

- [x] RED — remove `@wip` from this scenario only; run; expect non-zero
  (stub panic on the skill-override Given).
  - Command: `GIVN_FEATURES=givn/changes/validate-generated-references/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::An undeclared reference in an installed skill fails the check"`
  - Output: `exit=101` — `✘ And a givn-ideation skill override that names the missing file "givn/reference-that-resolves-nowhere.md"` → `Step panicked. Captured output: not implemented`; `1 scenario (1 failed)`.
- [x] GREEN — implement the skill-override Given (valid frontmatter plus
  one backticked dead reference) and the installed-failure Then; re-run;
  expect zero.
  - Files: `tests/steps/instruction_integrity_steps.rs`.
  - Output: `exit=0` — `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR — re-run; expect zero.
  - Output: `exit=0` — `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] COMMIT — `feat(instruction-integrity): <release note>` with the
  scenario title in the body.
  - Hash: `394ada3e`

## Task 7 — A printed reference that resolves nowhere fails the check (`instruction-integrity`)

- [x] RED — remove `@wip` from this scenario only; run; expect non-zero
  (stub panic on the specs-override Given).
  - Command: `GIVN_FEATURES=givn/changes/validate-generated-references/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::A printed reference that resolves nowhere fails the check"`
  - Output: `exit=101` — `✘ And a specs instruction override that names the missing file "givn/reference-that-resolves-nowhere.md"` → `Step panicked. Captured output: not implemented`; `1 scenario (1 failed)`.
- [x] GREEN — implement the specs-override Given and the printed-failure
  Then; re-run; expect zero.
  - Files: `tests/steps/instruction_integrity_steps.rs`.
  - Output: `exit=0` — `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR — re-run; expect zero.
  - Output: `exit=0` — `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] COMMIT — `feat(instruction-integrity): <release note>` with the
  scenario title in the body.
  - Hash: `511ea8f1`

## Final verification

- [ ] **F1. Change scope green.** Run
  `GIVN_FEATURES=givn/changes/validate-generated-references/specs ./run-tests.sh`
  and paste the summary (zero exit).
  - Output: _(paste)_
- [ ] **F2. Full suites green.** Run `./run-tests.sh` and
  `./run-tests.sh --e2e`; paste both summaries (zero exit each).
  - Output: _(paste)_
- [ ] **F3. No empty step bodies.** Grep every touched step file for `{}`,
  bare `return`, and `unimplemented!()`; zero hits.
  - Output: _(paste)_
- [ ] **F4. Commit count.** One commit per scenario (7) plus the planning
  commit; list the hashes.
  - Output: _(paste)_
