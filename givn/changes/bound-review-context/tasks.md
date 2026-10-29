# Tasks: bound-review-context

Owning use case: `verify-change` (`givn/specs/verify-change/usecase.md`).
Capability: `givn-workflow-enforcement`. Personas: `agent--technical-role`,
`developer--primary-user`.

The change adds a review-context record and gate, and removes prescriptive
subagent orchestration from guidance assets. Review kind defaults to
`self-review`; a recorded `independent` proposal accepts only an independent
review record whose reviewer context differs from the author.

## Setup

- [x] Confirm the existing runner and strict mode; no new runner is installed.
      Evidence:
      ```text
      tests/features_runner.rs: GivnWorld::cucumber().fail_on_skipped()
      givn/commands.yaml: verify.command = ./run-tests.sh --non-e2e
      givn/commands.yaml: verify.e2e_command = ./run-tests.sh --e2e
      ```
      Strict-mode probe (undefined step):
      ```text
      $ GIVN_FEATURES=<probe> cargo test --locked --test features_runner -- --name '^undefined step$'
      Scenario: undefined step
        ✘  Given an undefined strict probe step
      Step doesn't match any function
      1 scenario (1 failed)
      exit status: 1
      ```
      Probe removed; the suite returned to its passing baseline.
- [x] Design `## Step Definitions` declares `tests/steps/workflow_enforcement_steps.rs`
      for all scenarios; the file already exists.
- [x] No new E2E scenario is added; the existing `check review` E2E action and
      `tests/steps/workflow_enforcement_e2e_steps.rs` cover the real CLI
      boundary.

## Batch A — parser and accepted review kinds

Design constraints for every scenario in this batch:

- Contract: `review_context::check(&Path) -> anyhow::Result<()>`.
- `src/review_context.rs` owns parsing and validation; `src/lib.rs` exports it.
- `src/integrity_hook.rs::run_integrity_hook` calls it; `src/cli/archive.rs::run_static_review_gates` calls it before mutation.
- Record format, outside fenced blocks:
  `Review kind: self-review|independent`, `Author: <id>`,
  `Reviewer context: <id>`.
- A change with no `Review kind` line keeps the previous result.
- A self-review is accepted when no independent kind is recorded.
- An independent record requires a reviewer context different from the author.
- Failure messages name the artifact, the kind, and the offending value.

### Scenario: A recorded self-review is accepted

- [x] RED: `GIVN_FEATURES=givn/changes/bound-review-context/specs cargo test --locked --test features_runner -- --name '^A recorded self-review is accepted$'`
      passed before the gate existed; kept as regression protection for the
      default path. No production diff was forced for this scenario.
- [x] GREEN: `1 scenario (1 passed)` after wiring `review_context::check` into
      the integrity hook.
- [x] REFACTOR: `1 scenario (1 passed)` after the shared parser extraction.
- [x] COMMIT: `53ab78b6` -- `feat(givn-workflow-enforcement): review-context gate` (Batch: review-context-gate)

### Scenario: An independent review by a different context is accepted

- [x] RED: passed before the gate existed; kept as regression protection for
      the independent path.
- [x] GREEN: `1 scenario (1 passed)` after accepting an independent record with
      a distinct reviewer context.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `53ab78b6` -- `feat(givn-workflow-enforcement): review-context gate` (Batch: review-context-gate)

### Scenario: A legacy review record without a kind keeps its existing gate result

- [x] RED: passed before the gate existed; kept as backward-compatibility
      protection.
- [x] GREEN: `1 scenario (1 passed)` with the legacy branch returning `Ok`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `53ab78b6` -- `feat(givn-workflow-enforcement): review-context gate` (Batch: review-context-gate)

## Batch B — independent-request failures and one context

Design constraints for every scenario in this batch:

- The proposal kind is authoritative. A proposal recording `independent` with a
  review recording `self-review` fails with `independent review is required`
  and names the recorded `Review kind: self-review`.
- An independent reviewer context equal to the author fails naming
  `Reviewer context` and `must differ from the author`.
- `design-review.md` and `review.md` must name the same reviewer context for an
  independent review; the failure names both contexts and `one review context`.
- The gate reads the record on every run; no session memory is trusted.

### Scenario: An independent review whose reviewer is the author is refused

- [x] RED: `expected failure, got exit Some(0)` before the equality check.
- [x] GREEN: `1 scenario (1 passed)` after rejecting `author == reviewer`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `53ab78b6` -- `feat(givn-workflow-enforcement): review-context gate` (Batch: review-context-gate)

### Scenario: A self-review cannot satisfy a recorded independent request

- [x] RED: `expected failure, got exit Some(0)` before the proposal-kind check.
- [x] GREEN: `1 scenario (1 passed)` after comparing the proposal and review
      kinds.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `53ab78b6` -- `feat(givn-workflow-enforcement): review-context gate` (Batch: review-context-gate)

### Scenario: The recorded independent choice is authoritative from the proposal

- [x] RED: `first run unexpectedly succeeded` before the proposal-kind check.
- [x] GREEN: `1 scenario (1 passed)`; both runs re-read the proposal from disk
      and fail.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `53ab78b6` -- `feat(givn-workflow-enforcement): review-context gate` (Batch: review-context-gate)

### Scenario: Design review and final review share one independent context

- [x] RED: `expected failure, got exit Some(0)` before the design-review
      comparison.
- [x] GREEN: `1 scenario (1 passed)` after requiring one reviewer context.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `53ab78b6` -- `feat(givn-workflow-enforcement): review-context gate` (Batch: review-context-gate)

## Batch C — guidance assets and review instruction

Design constraints for every scenario in this batch:

- `assets/instructions/review.md` is the only guidance asset allowed to name a
  review subagent. It owns the default self-review, the one independent review
  context, the major-change criteria, the one-context maximum, and the
  reviewer-not-author rule.
- The exact policy text from the proposal is added to that instruction.
- Remove architect-subagent orchestration from `assets/instructions/proposal.md`.
- Remove mandatory fresh-context grilling and hardening subagents from
  `assets/instructions/design-review.md`; refer to `givn instructions review`
  when independence is selected.
- Remove the review-subagent step from `characterize.md`; remove fresh-context
  quality-review instructions from `ideation.md`; remove fresh-delegated-context
  instructions from `reverse-engineer.md`; remove coding-subagent delegation and
  self-report language from `tasks.md`.
- Remove `start one fresh review subagent` from `assets/commands/givn-review.md`
  and `assets/skills/givn-review/SKILL.md.tmpl`; load the review instruction.
- Rewrite the command/skill dialog boilerplate so it is conditional on handing
  work to another context, not a prescription to create one.
- Keep business-language "delegating" in `dialog.md`.
- Asset scan scope: `assets/instructions`, `assets/commands`, `assets/skills`,
  `assets/templates`, plus `assets/agents-project-hint.md`. The development
  report and harness reporter integrations are excluded.

### Scenario: The review instruction carries the one-context policy

- [x] RED: `givn instructions review failed: error: No active changes` and the
      missing-policy assertion before the instruction was rewritten.
- [x] GREEN: `1 scenario (1 passed)` after adding the exact policy block.
- [x] REFACTOR: `1 scenario (1 passed)` with the policy in one section.
- [x] COMMIT: `f1e0dbf5` -- `docs(givn-workflow-enforcement): review-context guidance` (Batch: review-context-guidance)

### Scenario: Guidance assets stop prescribing subagent orchestration

- [x] RED: `guidance assets other than the review instruction name a subagent:
      assets/commands/givn-review.md, assets/instructions/characterize.md,
      assets/instructions/design-review.md, assets/instructions/ideation.md,
      assets/instructions/proposal.md, assets/instructions/review.md,
      assets/instructions/tasks.md, assets/skills/givn-review/SKILL.md.tmpl`.
- [x] GREEN: `1 scenario (1 passed)`; only `assets/instructions/review.md`
      matches, and no other guidance asset prescribes a separate agent,
      session, architect, or reviewer.
- [x] REFACTOR: `1 scenario (1 passed)` after removing the duplicated dialog
      boilerplate and pointing to the canonical instruction.
- [x] COMMIT: `f1e0dbf5` -- `docs(givn-workflow-enforcement): review-context guidance` (Batch: review-context-guidance)

## Batch D — outline identity follows the recorded review kind

Design constraints for every scenario in this batch:

- The outline gate reads `Review kind`, `Author`, and `Reviewer context` from
  `review.md` through `review_context::parse`.
- Self-review: `OUTLINE: AUTHOR` and `OUTLINE: REVIEWER` match the recorded
  author and self context; equality is accepted.
- Independent: `OUTLINE: AUTHOR` and `OUTLINE: REVIEWER` match the recorded
  author and reviewer; the reviewer differs from the author.
- A fake or conflicting outline id fails with the recorded value in the
  message.
- A legacy record with no `Review kind` keeps the reviewer-differs rule.
- The outline addon remains an evidence generator; no second context is
  started for a self-review.

### Scenario: A self-review outline uses the self context

- [x] RED against the pre-fix gate: `Then the review passes` failed with
      `outline reviewer 'agent' must differ from the author 'agent'`.
- [x] GREEN: `1 scenario (1 passed)` after the outline gate read the recorded
      self context.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `dc33067f` -- `fix(givn-workflow-enforcement): outline review context` (Batch: outline-review-context)

### Scenario: An independent outline uses the recorded reviewer context

- [x] RED: passed against the pre-fix gate; kept as regression protection for
      the independent path.
- [x] GREEN: `1 scenario (1 passed)` with the recorded reviewer context.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `dc33067f` -- `fix(givn-workflow-enforcement): outline review context` (Batch: outline-review-context)

### Scenario: An outline reviewer that differs from the recorded context is refused

- [x] RED against the pre-fix gate: `Then the review fails` failed because the
      old gate accepted any differing label.
- [x] GREEN: `1 scenario (1 passed)` with
      `does not match the recorded Reviewer context`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `dc33067f` -- `fix(givn-workflow-enforcement): outline review context` (Batch: outline-review-context)

### Scenario: An outline author that differs from the recorded author is refused

- [x] RED against the pre-fix gate: `Then the review fails` failed because the
      old gate did not compare the outline author with the review record.
- [x] GREEN: `1 scenario (1 passed)` with
      `does not match the recorded Author`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `dc33067f` -- `fix(givn-workflow-enforcement): outline review context` (Batch: outline-review-context)

### Scenario: A legacy outline without a review kind keeps the reviewer-differs rule

- [x] RED: passed against the pre-fix gate; kept as backward-compatibility
      protection.
- [x] GREEN: `1 scenario (1 passed)`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `dc33067f` -- `fix(givn-workflow-enforcement): outline review context` (Batch: outline-review-context)

### Scenario: A legacy outline with the author as reviewer is refused

- [x] RED: passed against the pre-fix gate; kept as regression protection for
      the legacy rule.
- [x] GREEN: `1 scenario (1 passed)` with `must differ from the author`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `dc33067f` -- `fix(givn-workflow-enforcement): outline review context` (Batch: outline-review-context)

## Closeout

- [x] `GIVN_FEATURES=givn/changes/bound-review-context/specs ./run-tests.sh --non-e2e`
      exited 0; `9 scenarios (9 passed)` for the review-context feature and
      `6 scenarios (6 passed)` for the outline feature.
- [x] `./run-tests.sh --e2e` exited 0; `112 tests run: 112 passed`.
- [x] `rg -i 'subagent|sub-agent' assets/instructions assets/commands assets/skills assets/templates assets/agents-project-hint.md`
      matches only `assets/instructions/review.md`.
- [x] Every delta step phrase is bound in the permanent test tree; the new
      outline scenarios reuse `tests/steps/outline_review_gate_steps.rs`.
- [ ] `givn check review --change bound-review-context` passes after the review
      record is written.
- [ ] `givn archive --change bound-review-context` succeeds.

