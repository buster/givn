# Instruction: design-review

Stress-test the plan before a single line of production code is written.

This review runs after the tasks list is complete — the full plan exists
(proposal, spec, design, tasks) but implementation has not started. Its purpose
is to catch problems now, when changing a decision costs a markdown edit, not a
rewrite.

**Two modes, both required:**

### 1. Grilling — relentless interrogation of the plan

Work through the plan as a sceptical colleague who has to build and maintain it.
Walk the design tree branch by branch, resolving dependencies between decisions
one at a time.

For each question or concern:
- Ask one question at a time, provide your recommended answer, and wait for
  confirmation before continuing.
- If a question can be answered by exploring the existing codebase, explore it
  instead of asking.
- Do not proceed to the next branch until the current one is resolved.

Cover at minimum:
- **Scope creep** — does the spec contain more than the proposal asked for? Less?
- **Tech choices** — is the selected stack the right fit? Are there simpler
  alternatives that still satisfy all scenarios?
- **Missing scenarios** — are there observable behaviours implied by the proposal
  but absent from the spec? Edge cases? Error paths?
- **Task order and dependencies** — are the tasks ordered by dependency? Will any
  task block another if done in the listed sequence?
- **Design constraints in tasks** — does every scenario task have a populated
  "Design constraints" block? Are the constraints specific enough that a coding
  agent cannot take a shortcut?
- **Testability** — can every scenario actually fail in RED? Are the Then-steps
  asserting concrete values, not just "it ran"?
- **Risks** — what is the most likely way this plan fails during implementation?

Record every decision reached and every open question that requires user input.

### 2. Hardening — fix what grilling found

After grilling is complete and all questions are resolved:

- Update `design.md` with any decisions changed during grilling.
- Update `tasks.md` with any missing scenarios, reordered tasks, or strengthened
  design-constraint blocks.
- Update `specs/**/*.feature` if any missing scenarios were identified (add with
  `@givn.added @wip`; run `givn lint --change <id>` to confirm exit 0 or 2).
- Do not change the proposal unless the grilling revealed a fundamental scope
  mismatch — flag that as a blocker instead.

**Rules:**
- Do not skip grilling in favour of a quick rubber-stamp. The whole point is to
  surface problems before implementation.
- Grilling is complete only when every branch of the design tree has been walked
  and every open question resolved or flagged.
- Hardening changes are ordinary edits to already-existing artifacts — they do
  not reset state.
- Sign off by writing `DESIGN-REVIEW: PASS` at the end of `design-review.md`
  only when grilling is done and all hardening edits are applied.
