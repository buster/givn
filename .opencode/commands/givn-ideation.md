---
description: Work a vague idea into change-ready use cases — structured, multi-session domain exploration
---

Work a vague idea into change-ready use cases.

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: The argument after `/givn-ideation` is an optional topic id. A
topic id names one ideation effort; without one, derive it from the user's
description and confirm it with the user.

**Rules**: the ideation discipline — the phase ladder, the one-question rule,
the persona rules, the topic state, the term and seed schemas, the handoff
rules, and the event-storming and use-case rules — is served by
`givn instructions ideation`. Load it and follow it; this file owns the
invocation orchestration only.

---

## Steps

### 1. Check for context

```sh
givn status
```

Resume an existing topic by reading its state first (the topic state layout
is in the served instruction); otherwise start a new topic and confirm its
id with the user.

### 2. Run the served instruction

```sh
givn instructions ideation
```

Follow it for every phase. The phases run in order:

Frame → Event Storming → Use Case Definition → Use Case Elaboration → Handoff

---

## Guardrails

- Follow the rules served by `givn instructions ideation`; where this file
  and the served instruction disagree, the served instruction wins.
