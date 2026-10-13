---
name: givn-reverse-engineer
description: Reverse-engineer an existing capability into descriptive Gherkin specs — discovery, feature writing, characterization tasks.
---

# givn-reverse-engineer

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Delegated contexts:** a fresh delegated context does not inherit the parent's loaded policy. Pass the obligation explicitly — for every user-facing question, follow the effective policy from `givn instructions dialog`; load it in the fresh context before the first question. The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.

Reverse-engineer an existing capability into **descriptive Gherkin specs** for
change `<change-id>`. No proposals, no designs — just `.feature` files +
characterization tasks that make them executable.

**Rules**: the reverse-engineering discipline — the descriptive stance, the
interactive discovery phase, the spec-writing rules, and the black-box and
suspected-bug rules — is served by `givn instructions reverse-engineer`. Load
it and follow it; this file owns the invocation orchestration only.

## Context

- Change: `givn/changes/<change-id>/`
- Test runner: `./run-tests.sh`
- Features path: `GIVN_FEATURES=givn/specs`

## Invocation

1. Run `givn instructions reverse-engineer` and follow it.
2. Discovery is always interactive: propose ONE capability with a clear
   black-box boundary and wait for the user's confirmation before writing any
   artifact.

## Transition out

When planning is complete, suggest:
`/givn-characterize` — execute the RED/GREEN/REFACTOR characterization loop.

## Verify command

```
./run-tests.sh
```
