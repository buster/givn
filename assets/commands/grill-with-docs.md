---
description: Grill the plan or design against the project's own docs and specs — cite the source for every challenge.
---

Grill the current plan or design, grounding every challenge in the project's
existing documentation and specs.

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Rules**: the sourcing discipline — the cite-a-source rule, the source
inventory, the question-by-question cross-check, the drift-flagging rule, and
the output contract — is served by `givn instructions grill-with-docs`. Load
it and follow it; this file owns the invocation orchestration only.

## Steps

1. Gather the authoritative sources (the inventory is in the served
   instruction).
2. Run `givn instructions grill-with-docs` and conduct the grill as it
   defines: one sourced question at a time, waiting for each answer.
3. Report the sourced list of contradictions and drift findings; unresolved
   drift blocks implementation.
