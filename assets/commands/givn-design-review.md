---
description: Stress-test the plan before implementation — grill the design, harden the artifacts, sign off
---

Stress-test the complete plan (proposal, spec, design, tasks) before any
production code is written.

**Input**: Optionally specify a change ID after `/givn-design-review`. If
omitted, infer from conversation context, or auto-select if only one active
change exists. If ambiguous, list changes and ask.

---

## Role

This command runs **in the current context** — it does not spawn subagents.
It is an interactive, two-phase process:

1. **Grilling** — relentless one-question-at-a-time interrogation of the plan.
2. **Hardening** — apply the decisions reached to the existing artifacts.

Both phases must complete before `design-review.md` is written and signed off.

---

## Steps

### 1. Resolve the change

```sh
givn status --change <id> --json
```

Identify the active change. Announce: "Using change: <id>".

Check the `artifacts` array:
- `tasks` must be `"status": "done"` — all tasks listed and checked.
- If `tasks` is not done, warn: "Tasks are not complete. The design-review
  should run after the full task list is written." Confirm to proceed anyway.

### 2. Read everything

Read all planning artifacts in full before asking a single question:

- `givn/changes/<id>/proposal.md`
- `givn/changes/<id>/specs/**/*.feature`
- `givn/changes/<id>/design.md`
- `givn/changes/<id>/tasks.md`
- Relevant existing codebase files named in design.md (integration points,
  patterns, modules that will be affected).

Also run:

```sh
givn instructions design-review --change <id>
```

Capture `output` path, `template`, and `instruction`.

Do not ask a single question until all reading is done. Grilling from
incomplete knowledge produces shallow questions.

### 3. Grilling — one question at a time

Work through the plan as a sceptical colleague who has to build and maintain it.
Walk the design tree branch by branch. For each question:

1. State which **branch** you are exploring (e.g. "Scope", "Tech choices",
   "Task order", "Testability").
2. Ask **one question** only.
3. Provide your **recommended answer** — do not ask without a recommendation.
4. **Wait for the user's response** before moving to the next question.
5. Record the outcome (agreed / overridden / deferred).

If a question can be answered by exploring the codebase, explore it and report
the finding instead of asking the user.

**Required branches to walk (in any order):**

- **Scope** — does the spec contain everything the proposal asked for? Anything
  extra? Is anything ambiguous about "done"?
- **Tech choices** — is the stack in design.md the right fit for the scenarios?
  Are there simpler alternatives that still satisfy all observable behaviours?
- **Missing scenarios** — are there observable behaviours implied by the
  proposal but absent from the spec? Error paths? Boundary conditions?
- **Task order and dependencies** — are tasks ordered by dependency? Will any
  task require something that a preceding task hasn't built yet?
- **Design constraints** — does every scenario task have a "Design constraints"
  block? Are the constraints specific enough that there is no ambiguous shortcut?
- **Testability** — can every scenario actually fail in RED with no production
  code? Do Then-steps assert concrete observable values, or do they just assert
  "it ran"?
- **Risk** — what is the single most likely way this plan fails during
  implementation? What is the mitigation?

Do not move on from a branch until the current question is answered. Do not ask
two questions at once.

Grilling is complete when every branch has been walked and every question
resolved or explicitly flagged as a blocker.

### 4. Hardening — apply the decisions

After grilling is complete, apply all decisions reached:

- Edit `givn/changes/<id>/design.md` if any tech decisions changed.
- Edit `givn/changes/<id>/tasks.md` if any task order, constraint blocks, or
  scenario tasks need updating.
- Edit `givn/changes/<id>/specs/**/*.feature` if any missing scenarios were
  identified — add them with `@givn.added @wip`.
- Do not edit the proposal unless grilling revealed a fundamental scope mismatch
  — flag that as a blocker instead.

After hardening edits:

```sh
givn lint --change <id>
```

Exit 0 or 2 = correct. Exit 1 = parse error in a .feature file — fix before
continuing.

### 5. Write design-review.md

Write `givn/changes/<id>/design-review.md` following the template from
`givn instructions design-review --change <id>`:

- Grilling log: every question, recommended answer, and outcome.
- Open questions: any that could not be resolved (with "Blocking? yes/no").
- Changes made during hardening: a table of artifact edits.
- Sign-off checklist: all items checked.
- Final line: `DESIGN-REVIEW: PASS` — only if no blockers remain.

### 6. Show final status

```sh
givn status --change <id>
```

Display next step: "Run `/givn-implement` to start the TDD loop."
(Or flag blockers if any open questions are marked as blocking.)

---

## Output

### During grilling

```
## Design review: <id>

### Grilling — Scope

**Q1 (Scope):** <question>
**Recommended:** <recommended answer>

[waiting for user response]
```

After each answer, acknowledge briefly and move to the next question.

### On completion

```
## Design review complete: <id>

**Grilling:** <N> questions across <M> branches
**Hardening:** <summary of edits, or "no changes needed">
**Open blockers:** none / <list>

**Review written:** givn/changes/<id>/design-review.md  (DESIGN-REVIEW: PASS)

**Next:** Run `/givn-implement` to start the TDD loop.
```

---

## Guardrails

- Do not ask multiple questions at once — it defeats the purpose of grilling.
- Do not rubber-stamp: if the plan looks fine after reading, look harder. A
  plan with no issues is a plan that was not read carefully enough.
- Do not write `DESIGN-REVIEW: PASS` if any open question is marked as blocking.
- Hardening edits are ordinary artifact edits, not new artifacts — they do not
  reset the `design` or `tasks` status.
- If grilling reveals that the proposal scope is fundamentally wrong, do not
  silently fix it in design — surface it to the user as a blocker.
- `givn lint` must exit 0 or 2 after any .feature edits before signing off.
