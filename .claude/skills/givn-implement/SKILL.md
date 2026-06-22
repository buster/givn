---
name: givn-implement
description: Execute the RED/GREEN/REFACTOR TDD loop for a givn change, one scenario at a time.
---

# givn-implement

Implement change `<change-id>` using scenario-by-scenario TDD.

## Context

- Tasks: `givn/changes/<change-id>/tasks.md`
- Spec: `givn/changes/<change-id>/specs/`
- Design: `givn/changes/<change-id>/design.md`
- Test runner: `givn missing-testrunner`
- Features path: `GIVN_FEATURES=givn/specs`

## Before starting

1. Check your progress: `givn apply --change <change-id>`
2. `givn lint` to confirm the spec is well-formed (does NOT run tests).

## The loop

Work through tasks.md, one scenario at a time:

### RED phase
1. Remove `@wip` from the current scenario only (keep it on others).
2. Write step definitions with **real assertions** (`Then` checks concrete values).
3. Run: `givn missing-testrunner`
   - This scenario **must fail** — you have no production code yet.
   - If it passes, the assertions are fake — fix the steps.

### GREEN phase
4. Write the **minimum** production code to make this scenario pass.
5. Run: `givn missing-testrunner` — this scenario passes; others still fail (expected).

### REFACTOR phase
6. Clean up code, names, duplication — without changing behaviour.
7. Run: `givn missing-testrunner` — still passes.

### Done with scenario
8. Check off the task in `tasks.md`: `- [ ]` → `- [x]`
9. Move to the next scenario.

## When all tasks are checked

Run `givn apply --change <change-id>` to confirm progress.
Then proceed to the review step.

## Verify command

```
givn missing-testrunner
```
