# Instruction: design-review

Stress-test the plan before the task list is written.

This review runs after design is complete — the full planning layer exists
(proposal, spec, design) but tasks have not been written and implementation has
not started. Its purpose is to catch scope, architecture, and testability
problems now, when changing a decision costs a markdown edit, not a rewrite.

**Two phases, both required:**

### 1. Grilling — relentless interrogation of the plan

A grilling subagent reads the full plan and produces a ranked list of questions
with recommended answers. The orchestrator then presents the questions to the
user one at a time, waits for each answer, and records the outcome.

Required branches — cover every one (or explicitly record "no issue found"):

- **Scope** — does the spec contain exactly what the proposal asked for? Nothing
  extra, nothing missing? Is "done" unambiguous?
- **Tech choices** — is the stack in design.md the right fit for the scenarios?
  Are there simpler alternatives that still satisfy all observable behaviours?
- **Missing scenarios** — are there observable behaviours implied by the proposal
  but absent from the spec? Error paths? Boundary conditions?
- **Testability** — can every scenario actually fail in RED with no production
  code? Do the Then-steps assert concrete observable values, not just "it ran"?
- **Risk** — what is the single most likely way this plan fails during
  implementation? What is the mitigation?

If a question can be answered by exploring the existing codebase, explore it
and record the finding — do not burden the user with it.

### 2. Hardening — fix what grilling found

After all questions are answered, a hardening subagent applies decisions:

- Update `design.md` if any tech decisions changed.
- Add missing scenarios to `specs/**/*.feature` with `@givn.added @wip` if any
  were identified; run `givn lint --change <id>` to confirm exit 0 or 2.
- Do not write `tasks.md` — that is the next artifact.
- Do not change the proposal unless grilling revealed a fundamental scope
  mismatch and the user decided to fix it — flag that as a blocker instead.

**Rules:**
- Do not rubber-stamp. If the plan looks clean on first read, look harder.
- Grilling is complete only when every branch has been walked and every question
  answered or flagged as a blocker.
- Sign off by writing `DESIGN-REVIEW: PASS` at the end of `design-review.md`
  only when no blocking questions remain and all hardening edits are applied.
  Write `DESIGN-REVIEW: FAIL` if any blocker is unresolved.
