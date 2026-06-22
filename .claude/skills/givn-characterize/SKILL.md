---
name: givn-characterize
description: Execute the characterization loop for a reverse-engineering change — GREEN-first tests, testability refactor, reverse review.
---

# givn-characterize

Execute the characterization loop for change `<change-id>`.

## Context

- Tasks: `givn/changes/<change-id>/tasks.md`
- Spec: `givn/changes/<change-id>/specs/`
- Design: `givn/changes/<change-id>/design.md`
- Test runner: `givn missing-testrunner`
- Features path: `GIVN_FEATURES=givn/specs`

## The characterization technique

GREEN-first, not RED-first:

1. **GREEN**: write tests asserting **observed** behaviour (probe the real
   code). Remove `@wip` from this scenario only. Run `givn missing-testrunner` →
   must pass immediately.
2. If it fails: diagnose (infrastructure problem → fix plumbing; spec/mismatch
   → correct the spec, never the code).
3. **REFACTOR**: if design.md calls for a testability seam, apply it. Run
   `givn missing-testrunner` → must still pass. If red, revert the refactor.
4. Check off the task. Move to the next scenario.

## What not to do

- Do NOT change production code behaviour to satisfy a test.
- Do NOT fix bugs discovered during characterization. Flag them.
- Do NOT use `givn check <artifact>` during the loop — run `givn missing-testrunner`
  directly. `givn check` is only for the review epilogue.
- Do NOT do general cleanup (rename, dedup, logic simplification) during
  REFACTOR — only testability seams (extract interface, DI, make public).
- Do NOT remove `REVERSE-ENGINEERING NOTE` markers — they are intentional debt
  markers.

## Spec/implementation mismatch diagnostic

When verify fails and infrastructure is sound:

1. **Diagnose**: documentation was wrong, or code is ambiguous/misleading.
2. **If docs wrong**: update design.md + tasks.md + the .feature scenario +
   the source documentation. Re-run verify.
3. **If code ambiguous**: update the .feature scenario to AS-IS behaviour. Add
   a code comment: `REVERSE-ENGINEERING NOTE: <what was misleading> — actual
   behaviour: <Z>. Code smell, refactor candidate.` (Use the language's comment
   syntax.) Do NOT change code behaviour. Re-run verify.

## Reverse review epilogue

After all tasks are checked, a review subagent:
- Runs coverage (record the number, classify gaps).
- Dead code → flag as `@givn.removed` candidate (do NOT delete).
- Missing test → flag as `@givn.added` candidate (do NOT add during review).
- Confirms behaviour-preservation invariant (all characterization tests still
  GREEN after refactoring).
- Compiles: spec corrections, documentation corrections, code smell markers,
  suspected bugs.
- Writes `review.md` with `REVIEW: PASS`.
- Runs `givn check review --change <change-id>`.

## Transition out

When characterization + review are complete, suggest:
`/givn-archive` — archive the change and create the baseline spec.

## Verify command

```
givn missing-testrunner
```
