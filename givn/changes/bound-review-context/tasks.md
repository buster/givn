# Tasks: bound-review-context

Owning use case: `verify-change` (`givn/specs/verify-change/usecase.md`).
Capability: `givn-workflow-enforcement`. Personas: `agent--technical-role`,
`developer--primary-user`.

The change adds a review-context record and gate, and removes prescriptive
subagent orchestration from guidance assets. Review kind defaults to
`self-review`; a recorded `independent` proposal accepts only an independent
review record whose reviewer context differs from the author.

## Setup

- [ ] Confirm the existing runner and strict mode; no new runner is installed.
      Evidence:
      ```text
      # existing configuration, unchanged:
      tests/features_runner.rs: GivnWorld::cucumber().fail_on_skipped()
      givn/commands.yaml: verify.command = ./run-tests.sh --non-e2e
      givn/commands.yaml: verify.e2e_command = ./run-tests.sh --e2e
      ```
      Strict-mode probe (undefined step must fail):
      ```text
      <probe command and non-zero output>
      ```
      Remove the probe and re-run the runner to return to the passing baseline.
- [ ] Design `## Step Definitions` declares `tests/steps/workflow_enforcement_steps.rs`
      for all scenarios; the file already exists.
- [ ] No new E2E scenario is added; the existing `check review` E2E action and
      `tests/steps/workflow_enforcement_e2e_steps.rs` cover the real CLI
      boundary.

## Batch A — parser and accepted review kinds

Design constraints for every scenario in this batch:

- Contract: `review_context::check(&Path) -> anyhow::Result<()>`.
- `src/review_context.rs` owns parsing and validation; `src/lib.rs` exports it.
- `src/integrity_hook.rs::run_integrity_hook` calls it before the visual and
  ARc42 checks; `src/cli/archive.rs::run_static_review_gates` calls it before
  mutation.
- Record format, outside fenced blocks:
  `Review kind: self-review|independent`, `Author: <id>`,
  `Reviewer context: <id>`.
- A change with no `Review kind` line keeps the previous result.
- A self-review is accepted when no independent kind is recorded.
- An independent record requires a reviewer context different from the author.
- Failure messages name the artifact, the kind, and the offending value.

### Scenario: A recorded self-review is accepted

- [ ] RED: remove `@wip` from this scenario; add the three record given steps
      and the `check review` step. Run:
      `GIVN_FEATURES=givn/changes/bound-review-context/specs cargo test --locked --test features_runner -- --name '^A recorded self-review is accepted$'`
      Paste the non-zero output.
- [ ] GREEN: implement `ReviewKind`, `ReviewRecord`, `parse`, and the happy-path
      branch of `check`; wire the call into the integrity hook. Run the same
      command; paste the zero output.
- [ ] REFACTOR: extract the fenced-block scan and the record parser; re-run;
      paste the zero output.
- [ ] COMMIT: `<hash>` -- `feat(givn-workflow-enforcement): ...` (Batch: review-context-parser)

### Scenario: An independent review by a different context is accepted

- [ ] RED: remove `@wip`; run the scenario only; paste the non-zero output.
- [ ] GREEN: accept `independent` when the reviewer context differs from the
      author; run the scenario only; paste the zero output.
- [ ] REFACTOR: share the record validation between both kinds; re-run; paste
      the zero output.
- [ ] COMMIT: `<hash>` -- `feat(givn-workflow-enforcement): ...` (Batch: review-context-parser)

### Scenario: A legacy review record without a kind keeps its existing gate result

- [ ] RED: remove `@wip`; run the scenario only; paste the non-zero output
      (or record that the legacy branch already passes because no record is
      present, and keep the scenario as regression protection).
- [ ] GREEN: return `Ok(())` when neither the proposal nor the review records
      a kind; run the scenario only; paste the zero output.
- [ ] REFACTOR: name the legacy branch; re-run; paste the zero output.
- [ ] COMMIT: `<hash>` -- `feat(givn-workflow-enforcement): ...` (Batch: review-context-parser)

## Batch B — independent-request failures and one context

Design constraints for every scenario in this batch:

- The proposal kind is authoritative. A proposal recording `independent` with a
  review recording `self-review` fails with
  `independent review is required` and names the recorded `Review kind: self-review`.
- An independent reviewer context equal to the author fails naming
  `Reviewer context` and `must differ from the author`.
- `design-review.md` and `review.md` must name the same reviewer context for an
  independent review; the failure names both contexts and `one review context`.
- The gate reads the record on every run; no session memory is trusted.

### Scenario: An independent review whose reviewer is the author is refused

- [ ] RED: remove `@wip`; add the record given steps; run the scenario only;
      paste the non-zero output.
- [ ] GREEN: reject `author == reviewer` for independent; run the scenario only;
      paste the zero output.
- [ ] REFACTOR: use one helper for the equality check; re-run; paste the zero
      output.
- [ ] COMMIT: `<hash>` -- `feat(givn-workflow-enforcement): ...` (Batch: review-kind-enforcement)

### Scenario: A self-review cannot satisfy a recorded independent request

- [ ] RED: remove `@wip`; run the scenario only; paste the non-zero output.
- [ ] GREEN: compare the proposal kind with the review kind; run the scenario
      only; paste the zero output.
- [ ] REFACTOR: report both recorded kinds; re-run; paste the zero output.
- [ ] COMMIT: `<hash>` -- `feat(givn-workflow-enforcement): ...` (Batch: review-kind-enforcement)

### Scenario: The recorded independent choice is authoritative from the proposal

- [ ] RED: remove `@wip`; add the `twice` step; run the scenario only; paste
      the non-zero output.
- [ ] GREEN: ensure each run re-reads the proposal and review records from disk;
      run the scenario only; paste the zero output.
- [ ] REFACTOR: no state is cached between runs; re-run; paste the zero output.
- [ ] COMMIT: `<hash>` -- `feat(givn-workflow-enforcement): ...` (Batch: review-kind-enforcement)

### Scenario: Design review and final review share one independent context

- [ ] RED: remove `@wip`; run the scenario only; paste the non-zero output.
- [ ] GREEN: compare the design-review and review reviewer contexts for an
      independent record; run the scenario only; paste the zero output.
- [ ] REFACTOR: share the record comparison helper; re-run; paste the zero output.
- [ ] COMMIT: `<hash>` -- `feat(givn-workflow-enforcement): ...` (Batch: review-kind-enforcement)

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

- [ ] RED: remove `@wip`; add the instruction-read step and the policy Then
      steps; run the scenario only; paste the non-zero output.
- [ ] GREEN: write the exact policy block into `assets/instructions/review.md`;
      run the scenario only; paste the zero output.
- [ ] REFACTOR: keep the policy in one section; re-run; paste the zero output.
- [ ] COMMIT: `<hash>` -- `docs(givn-workflow-enforcement): ...` (Batch: review-context-guidance)

### Scenario: Guidance assets stop prescribing subagent orchestration

- [ ] RED: remove `@wip`; add the asset-scan step and the forbidden-prescription
      check; run the scenario only; paste the non-zero output.
- [ ] GREEN: rewrite the listed guidance assets, the command/skill templates,
      and `agents-project-hint.md`; run the scenario only; paste the zero output.
- [ ] REFACTOR: remove duplicate dialog boilerplate by pointing to the canonical
      dialog instruction; re-run; paste the zero output.
- [ ] COMMIT: `<hash>` -- `docs(givn-workflow-enforcement): ...` (Batch: review-context-guidance)

## Closeout

- [ ] `GIVN_FEATURES=givn/changes/bound-review-context/specs ./run-tests.sh --non-e2e`
      exits 0 and the scenario count is recorded.
- [ ] `GIVN_FEATURES=givn/changes/bound-review-context/specs ./run-tests.sh --e2e`
      exits 0 (or records that no new `@e2e` scenario was added and the existing
      E2E scope is unchanged).
- [ ] `rg -i 'subagent|sub-agent' assets/instructions assets/commands assets/skills assets/templates assets/agents-project-hint.md`
      matches only `assets/instructions/review.md`.
- [ ] Every delta step phrase is bound in the permanent test tree.
- [ ] `givn check review --change bound-review-context` passes after the review
      record is written.
- [ ] `givn archive --change bound-review-context` succeeds.
