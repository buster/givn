---
name: givn-characterize
description: Execute the characterization loop for a reverse-engineering change — RED/GREEN/REFACTOR with step reuse, testability seams, reverse review.
---

# givn-characterize

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Delegated contexts:** a fresh delegated context does not inherit the parent's loaded policy. Pass the obligation explicitly — for every user-facing question, follow the effective policy from `givn instructions dialog`; load it in the fresh context before the first question. The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.

Execute the characterization loop for change `<change-id>`.

**Rules**: the characterization discipline — the RED/GREEN/REFACTOR technique
for observed behaviour, the what-not-to-do rules, the spec/implementation
mismatch diagnostic, and the reverse review epilogue — is served by
`givn instructions characterize`. Load it and follow it; this file owns the
invocation orchestration only.

## Context

- Tasks: `givn/changes/<change-id>/tasks.md`
- Spec: `givn/changes/<change-id>/specs/`
- Test runner: `./run-tests.sh`
- Features path: `GIVN_FEATURES=givn/specs`
- Shared step mechanics: `givn instructions steps`

## Invocation

1. Run `givn instructions characterize` and follow it.
2. Take the tasks in order: one RED/GREEN/REFACTOR cycle per scenario, pasting
   the runner output into the task before checking it off.
3. After all tasks are checked, apply `givn instructions review-reverse` in
   the reverse review epilogue.

## Transition out

When characterization + review are complete, suggest:
`/givn-archive` — archive the change and create the baseline spec.

## Verify command

```
./run-tests.sh
```
