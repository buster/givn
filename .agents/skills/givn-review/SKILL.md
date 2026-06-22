---
name: givn-review
description: Complete the coverage review for a givn change — classify every gap, sign off, and confirm GREEN.
---

# givn-review

Complete the coverage review for change `<change-id>`.

## Context

- Review file: `givn/changes/<change-id>/review.md`
- Instructions: run `givn instructions review --change <change-id>`
- Test runner: `givn missing-testrunner`

## Coverage gate (completeness, not percentage)

Classify every uncovered line/branch into exactly one bucket:

1. **Dead code** (YAGNI/KISS) → delete it now.
2. **Missing test coverage** → add a scenario + run RED/GREEN/REFACTOR, or fix steps.
3. **Legitimately hard to test** — rare; needs concrete technical justification.

Coverage is complete when every gap is classified and resolved (or justified).
**There is no minimum percentage.** A high number is a side effect, not the goal.

## Steps

1. Run `givn missing-testrunner` — confirm all tests pass.
2. Run your coverage tool and list uncovered regions in the review table.
3. Classify each gap (1, 2, or 3 above).
4. Resolve each gap:
   - Dead code → delete.
   - Missing test → add scenario + RED/GREEN/REFACTOR.
   - Hard to test → write justification.
5. Run `givn missing-testrunner` again — confirm still GREEN.
6. Complete the sign-off checklist in review.md.
7. Write `REVIEW: PASS` at the end of review.md when all items are checked.

## After sign-off

```
givn check review --change <change-id>
```

## Verify command

```
givn missing-testrunner
```
