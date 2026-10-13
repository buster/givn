---
name: givn-dev-principles
description: Apply the team's engineering principles whenever code is read or written during a givn change — correctness first, boring when called for, no unnecessary abstractions.
---

# givn-dev-principles

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Apply the team's engineering principles while implementing change `<change-id>`.

**Rules**: the principles and anti-patterns — correctness first, the TDD-loop
match, boring-when-called-for, no avoidable allocation, fix at the source, and
peer coordination — are served by `givn instructions dev-principles`. Load it
and follow it; this file owns the invocation orchestration only.

## Context

- Change: `givn/changes/<change-id>/`
- Tasks: `givn/changes/<change-id>/tasks.md`
- Design: `givn/changes/<change-id>/design.md`
- Verify command: `./run-tests.sh`

## Invocation

1. Run `givn instructions dev-principles` and follow it.
2. Apply the principles whenever code is read or written during this change.
