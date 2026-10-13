---
description: Socratically grill the current plan or design — one hard question at a time — before implementation begins.
---

Grill the current plan or design before implementation.

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Rules**: the grilling discipline — the stress-test stance, the question
list, push-until-concrete, and the outcome recording rules — is served by
`givn instructions grill-me`. Load it and follow it; this file owns the
invocation orchestration only.

## Steps

1. Read the change in order:

```sh
givn status --change <id>
```

Then read `proposal.md`, the `.feature` specs, `design.md`, `tasks.md`.

2. Run `givn instructions grill-me` and conduct the grill as it defines: one
   sharp question at a time, waiting for each answer before the next.

3. Record the hardened decisions and the unresolved blocking questions.
