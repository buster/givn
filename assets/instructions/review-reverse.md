# Instruction: review-reverse

Perform a reverse-engineering coverage review. The goal is to confirm that the
characterization is complete, that no behaviour was changed during refactoring,
and that all discovered bugs and code smells are documented.

**First: record the coverage number.**
Run the coverage tool and note the overall percentage (line and/or branch).
This is a fact recorded in the artifact — not a gate.

**Coverage classification (three buckets — reverse-mode rules):**

1. **Dead code** → flag as follow-up `@givn.removed` candidate. Do NOT delete.
2. **Missing test coverage** → flag as follow-up `@givn.added` candidate. Do NOT add during review.
3. **Legitimately hard to test** → write concrete technical justification.

**Behaviour-preservation invariant:**
- Re-run `verify.command` after all refactoring. All characterization tests
  must still be GREEN.
- If any characterization test went RED during the change (a refactor changed
  behaviour), the invariant was violated. Flag it in review.md with: which
  scenario, which refactor, what happened. The refactor should have been
  reverted during characterization — if it wasn't, flag as a blocker.

**Mandatory reporting sections in review.md:**

1. **Coverage** — percentage and raw tool output.
2. **Coverage gap table** — bucket, region, action for each gap.
3. **Spec corrections** — scenarios that were updated during characterization
   because the initial inference (from reading code/docs) was wrong. For each:
   - Scenario title
   - What was originally inferred
   - What the actual behaviour is
   - Root cause (documentation wrong / code ambiguous)
4. **Documentation corrections** — documentation (README, arc42, design.md,
   tasks.md, code comments) that was updated because it was wrong. For each:
   - Document and section
   - What it said before
   - What it says now
5. **Code smell markers** — all `REVERSE-ENGINEERING NOTE:` comments added
   during characterization. For each:
   - File and line
   - What was misleading
   - Actual behaviour
   - Suggested follow-up refactoring
 6. **Suspected bugs** — all bugs discovered during characterization. For each:
    - Description of observed vs correct behaviour
    - Scenario that captures the buggy AS-IS behaviour
    - Follow-up candidate: `@givn.modified` for that scenario
 7. **Boundary failures** — scenarios that were commented out because the
    black-box boundary proved untestable. For each:
    - Scenario title
    - What was attempted
    - Why the boundary failed
    - Suggested seam or boundary change for a follow-up change
 8. **Test suite output and result** — final GREEN confirmation.
 9. **Sign-off checklist** — all items checked.

**Sign-off checklist:**
- All scenarios pass (test runner exits 0).
- Every coverage gap classified and resolved (or justified).
- No `@wip` tags remain in the delta spec.
- The delta spec contains no implementation-layer detail.
- All spec corrections documented.
- All documentation corrections documented.
- All code smell markers listed.
- All suspected bugs flagged as follow-up candidates (NOT fixed).
- Behaviour-preservation invariant confirmed (all characterization tests
  GREEN after refactoring).

**Review result:** End the file with `REVIEW: PASS` or `REVIEW: FAIL`.
The archive gate reads this marker.
