---
description: Execute the characterization loop for a reverse-engineering change — RED/GREEN/REFACTOR with step reuse, testability seams, reverse review
---

Execute the characterization loop for a givn reverse-engineering change.
RED/GREEN/REFACTOR per scenario: step definitions assert **observed**
behaviour, production code is refactored only for testability seams.

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: Optionally specify a change ID after `/givn-characterize`. If
omitted, infer from conversation context, or auto-select if only one active
change exists. If ambiguous, list changes and ask.

**Rules**: the characterization discipline — the RED/GREEN/REFACTOR technique
for observed behaviour, the what-not-to-do rules, the spec/implementation
mismatch diagnostic, and the reverse review epilogue — is served by
`givn instructions characterize`. Load it and follow it; this file owns the
invocation orchestration only.

---

## Steps

### 1. Resolve the change

```sh
givn status --json
```

Identify from `active[0].id` (or ask if multiple). Announce: "Using change: <id>".

### 2. Check current progress

```sh
givn status --change <id>
```

If all tasks are already checked, suggest restarting.

### 3. Validate the spec

```sh
givn lint --change <id>
```

Exit 0 or 2 = proceed. Exit 1 = parse error — fix first.

### 4. Collect context for subagents

`tasks.md` (each task's "Seam" block if needed), `specs/<capability>/
<capability>.feature` (Feature: description has the black-box boundary and
suspected bugs), `verify.command`/`verify.e2e_command`.

### 5. Loop: one characterization subagent per task

For each unchecked task, in order: spawn a characterization subagent with a
self-contained prompt that passes the dialog obligation explicitly, names the
task and scenario, and tells the subagent to run
`givn instructions characterize` and follow it (including the shared step
mechanics from `givn instructions steps`). Wait for it to finish, then move to
the next task.

### 6. Reverse review epilogue

After all tasks are checked, spawn a review subagent. It runs
`givn instructions review-reverse` and follows it: reverse-mode coverage
review, behaviour-preservation invariant, spec/documentation corrections, code
smell markers, and `review.md` ending with `REVIEW: PASS`. Then it runs:

```sh
givn check review --change <id>
```

### 7. Show status

```sh
givn status --change <id>
```

---

## Guardrails

- Follow the rules served by `givn instructions characterize`; where this file
  and the served instruction disagree, the served instruction wins.
- Never change production code behaviour to satisfy a test; characterization
  probes observed behaviour.
- Testability seams only during REFACTOR — no general cleanup.
