---
description: Think through a problem before proposing a change — investigate, clarify, never implement
---

Think through a problem before proposing a change.

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**NEVER write code or implement features during explore.** If the user asks to
implement, direct them to `/givn-propose`.

**Input**: The argument after `/givn-explore` is whatever the user wants to
think about. Could be a vague idea, a specific problem, a comparison, or
nothing (just enter explore mode).

**Rules**: the explore discipline — the stance, what you might do, what you
must NOT do, and the transition rules — is served by
`givn instructions explore`. Load it and follow it; this file owns the
invocation orchestration only.

---

## Steps

1. Check for context:

```sh
givn status
```

2. Run `givn instructions explore` and think with the user as it defines.
3. When insights crystallise, offer the matching transition (`/givn-propose`,
   `/givn-spec`, `/givn-design`, or `/givn-ideation` for structured domain
   work). Never write givn artifacts during explore — artifacts are produced
   by their matching commands.
