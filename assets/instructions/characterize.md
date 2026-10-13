# Instruction: characterize

Execute the characterization loop for a reverse-engineering change — one
RED/GREEN/REFACTOR cycle per scenario, probing **observed** behaviour.

For every user-facing question, follow the effective policy from
`givn instructions dialog`; load it once per context before the first question.

## Context

- Tasks: `givn/changes/<id>/tasks.md`
- Spec: `givn/changes/<id>/specs/`
- Shared step mechanics: `givn instructions steps`

## The characterization technique

RED/GREEN/REFACTOR (same loop structure as normal TDD — see
`givn instructions steps` for the shared mechanics). What differs is the
GREEN and REFACTOR content:

1. **RED**: Remove `@wip` from this scenario only. Check for reusable step
   definitions first. New steps get unimplemented assertions. Run the
   project's verify command → must fail (or pass if all steps reused).
2. **GREEN**: Implement real assertions probing **observed** behaviour (read
   the actual code). Run verify → must pass. No production code
   changes. If it fails: diagnose (infrastructure → fix plumbing;
   spec/mismatch → correct the spec, never the code).
3. **REFACTOR**: if the task's Seam block calls for a testability seam, apply
   it. Run verify → must still pass. If red, revert the refactor.
4. Check off the task. Move to the next scenario.

## What not to do

- Do NOT change production code behaviour to satisfy a test.
- Do NOT fix bugs discovered during characterization. Flag them.
- Do NOT use `givn check <artifact>` during the loop — run the verify command
  directly. `givn check` is only for the review epilogue.
- Do NOT do general cleanup (rename, dedup, logic simplification) during
  REFACTOR — only testability seams (extract interface, DI, make public).
- Do NOT remove `TODO: REVERSE-ENGINEERING NOTE` markers — they are
  intentional debt markers.

## Spec/implementation mismatch diagnostic

When verify fails and infrastructure is sound:

1. **Diagnose**: documentation was wrong, or code is ambiguous/misleading.
2. **If docs wrong**: update tasks.md + the .feature scenario + the source
   documentation. Re-run verify.
3. **If code ambiguous**: update the .feature scenario to AS-IS behaviour. Add
   a code comment: `TODO: REVERSE-ENGINEERING NOTE: <what was misleading> —
   actual behaviour: <Z>. Code smell, refactor candidate.` (Use the language's
   comment syntax.) Do NOT change code behaviour. Re-run verify.

## Reverse review epilogue

After all tasks are checked, a review subagent:

- Applies reverse-mode rules (`givn instructions review-reverse`):
  - Classifies coverage gaps from the most recent measured evidence — the
    archive verification receipt or the merged report — plus static
    inspection; review does not re-run an instrumented pass.
  - Dead code → flag as `@givn.removed` candidate (do NOT delete).
  - Missing test → flag as `@givn.added` candidate (do NOT add during review).
  - Confirms behaviour-preservation invariant (all characterization tests
    still GREEN after refactoring).
- Compiles: spec corrections, documentation corrections, code smell markers,
  suspected bugs.
- Writes `review.md` with `REVIEW: PASS`.
- Runs `givn check review --change <id>`.
