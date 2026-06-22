---
description: Stress-test the plan before tasks are written — grill the design, harden the artifacts, sign off
---

Stress-test the plan (proposal, spec, design) before the task list is written.
Catches scope, architecture, and testability problems when they cost a markdown
edit rather than a rewrite.

**Input**: Optionally specify a change ID after `/givn-design-review`. If
omitted, infer from conversation context, or auto-select if only one active
change exists. If ambiguous, list changes and ask.

---

## Role: orchestrator

This command is an **orchestrator** with two subagents and an interactive loop
in between:

1. **Grilling subagent** — reads all planning artifacts and the codebase; returns
   a ranked list of questions with recommended answers.
2. **Interactive loop** — the orchestrator presents questions one at a time to
   the user, records answers.
3. **Hardening subagent** — applies all decisions reached to the artifacts and
   writes `design-review.md`.

The interactive loop runs in the orchestrator's own context so the
one-question-at-a-time dialogue works naturally.

---

## Steps

### 1. Resolve the change

```sh
givn status --change <id> --json
```

Identify the active change. Announce: "Using change: <id>".

Check the `artifacts` array:
- `design` must be `"status": "done"` before proceeding.
- If `design` is not done, stop: "Design must be complete before design-review."

### 2. Collect instructions

```sh
givn instructions design-review --change <id>
```

Capture `output` path, `template`, `instruction`, `dependencies`.

### 3. Spawn the grilling subagent

Use the **Task tool** to spawn a grilling subagent in a clean context.

Pass a self-contained prompt:

```
You are a grilling subagent for givn change '<id>'.

Your job: read the full plan and return a prioritised list of questions that
must be answered before implementation starts. You do NOT ask the user anything
directly — you return the questions so the orchestrator can present them one at
a time.

## Read everything first

Read all of these in full before forming any questions:
- givn/changes/<id>/proposal.md
- givn/changes/<id>/specs/**/*.feature
- givn/changes/<id>/design.md
- Existing codebase files named in design.md (integration points, affected
  modules, patterns). If a question can be answered by exploring the codebase,
  explore it and record the finding as a resolved item — do not include it as
  a question for the user.

## Required branches to cover

Walk the design tree and produce at least one question per branch (or record it
as "no issue found" if genuinely clean):

- **Scope** — does the spec match what the proposal asked for? Anything extra?
  Anything missing? Is "done" unambiguous?
- **Tech choices** — is the stack in design.md the right fit? Are there simpler
  alternatives that satisfy all the observable behaviours in the spec?
- **Missing scenarios** — are there observable behaviours implied by the proposal
  but absent from the spec? Error paths? Boundary conditions?
- **Testability** — can every scenario actually fail in RED with no production
  code? Do the Then-steps assert concrete values, not just "it ran"?
- **Risk** — what is the single most likely way this plan fails during
  implementation? What is the mitigation?

## Return format

Return a JSON array of objects, ordered by priority (most critical first):

[
  {
    "branch": "<branch name>",
    "question": "<the question>",
    "recommended": "<your recommended answer>",
    "codebase_finding": null   // or a string if you resolved it by exploring
  },
  ...
]

Also return a separate array of items you resolved by codebase exploration
(so the orchestrator can report them):

{
  "questions": [ ... ],
  "resolved_by_codebase": [
    { "branch": "...", "finding": "..." }
  ]
}

Return only the JSON — no prose before or after it.
```

### 4. Interactive grilling loop

Wait for the grilling subagent to return.

If any `resolved_by_codebase` items were found, report them first:

```
## Design review: <id>

**Resolved by codebase exploration:**
- [Branch]: <finding>
```

Then work through `questions` one at a time:

For each question:
1. Present it to the user:
   ```
   **Q<N> (<branch>):** <question>
   **Recommended:** <recommended answer>
   ```
2. **Wait for the user's response.** Do not present the next question until
   the current one is answered.
3. Record the outcome: agreed / overridden (note what the user decided) /
   deferred (flag as open question).
4. Ask: "Blocking or non-blocking?" Record the answer. Blocking means this
   question must be resolved before implementation can proceed. Non-blocking
   means it can be deferred.

When all questions are answered, summarise:

```
**Grilling complete.** <N> questions answered.
Open questions (blocking): <list or "none">
Open questions (non-blocking): <list or "none">

Proceeding to hardening...
```

If any open question is marked as blocking by the user, stop and report the
blocker. Do not proceed to hardening until the user resolves it.

### 5. Spawn the hardening subagent

Use the **Task tool** to spawn a hardening subagent in a clean context.

Pass a self-contained prompt that includes the full grilling outcome:

```
You are a hardening subagent for givn change '<id>'.

The plan has been grilled. Your job: apply the decisions reached to the
existing artifacts and write design-review.md.

## Grilling outcome

### Resolved by codebase exploration
<paste resolved_by_codebase items>

### Questions and answers
<paste each question, recommended answer, and the user's decision>

### Open questions (non-blocking)
<list, or "none">

## Your tasks

### 1. Apply hardening edits

Based on the grilling outcome, edit the existing artifacts as needed:

- `givn/changes/<id>/design.md` — update if any tech decisions changed.
- `givn/changes/<id>/specs/**/*.feature` — add missing scenarios with
  `@givn.added @wip` if any were identified. Then run:
  ```sh
  givn lint --change <id>
  ```
  Exit 0 or 2 = correct. Exit 1 = parse error — fix before continuing.
- Do NOT create tasks.md — tasks has not been written yet and is not your
  responsibility.
- Do NOT change proposal.md unless the grilling revealed a fundamental scope
  mismatch that the user explicitly decided to fix — flag that as a blocker.

### 2. Write design-review.md

Write `givn/changes/<id>/design-review.md` following this template structure:

## Grilling log
| # | Branch | Question | Recommended | Outcome |

## Resolved by codebase exploration
| Branch | Finding |

## Open questions
| # | Question | Blocking? |

## Changes made during hardening
| Artifact | Change summary |

## Sign-off
- [ ] All branches walked.
- [ ] All open questions resolved or flagged as non-blocking.
- [ ] design.md reflects decisions reached.
- [ ] specs/*.feature updated for any missing scenarios.
- [ ] givn lint exits 0 or 2.

DESIGN-REVIEW: PASS

Write `DESIGN-REVIEW: PASS` as the final line ONLY if no questions are marked
as blocking. If there are blocking open questions, write `DESIGN-REVIEW: FAIL`
instead and list the blockers.

### 3. Report back with
- Artifacts edited (list, or "none")
- Lint result (exit code)
- Whether DESIGN-REVIEW: PASS or FAIL was written, and why
- Path written: givn/changes/<id>/design-review.md
```

### 6. Wait for the hardening subagent to complete

If the subagent reports `DESIGN-REVIEW: FAIL`, surface the blockers to the user
and stop. Do not suggest next steps until blockers are resolved.

### 7. Show final status

```sh
givn status --change <id>
```

Display next step: "Run `/givn-tasks` to write the task list."

---

## Output on completion

```
## Design review complete: <id>

**Grilling:** <N> questions across <M> branches
**Resolved by codebase:** <N> items
**Hardening:** <summary of edits, or "no changes needed">
**Open questions (non-blocking):** <list or "none">

**Review written:** givn/changes/<id>/design-review.md  (DESIGN-REVIEW: PASS)

**Next:** Run `/givn-tasks` to write the task list.
```

## Output on blocker

```
## Design review blocked: <id>

**Blocking question:** <question>
**Status:** DESIGN-REVIEW: FAIL

Resolve the blocker before proceeding.
```

---

## Guardrails

- The orchestrator presents exactly one question per turn. Never batch questions.
- Do not proceed to hardening while any question is unanswered.
- Do not proceed to hardening while any blocking open question is unresolved.
- The hardening subagent must not write tasks.md — that is the next artifact.
- Hardening edits do not reset design or specs status — they are ordinary edits.
- `DESIGN-REVIEW: PASS` requires no blocking open questions. If blockers remain,
  write `DESIGN-REVIEW: FAIL` and surface them to the user.
- `givn lint` must exit 0 or 2 after any .feature edits before signing off.
