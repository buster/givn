# Instruction: review-reverse

Perform a reverse-engineering coverage review. The goal is to confirm that the
characterization is complete, that no behaviour was changed during refactoring,
and that all discovered bugs and code smells are documented.

**Before any work: read the shared coverage mechanics.**
Read the `givn-review` skill (at `.agents/skills/givn-review/SKILL.md` or the
equivalent path in your agent's skill directory) in full. It describes the
coverage gate (completeness, not percentage), gap classification, and sign-off
process. The rules below OVERLAY the givn-review skill for reverse-engineering.

**First: record the coverage number.**
Run the coverage tool and note the overall percentage (line and/or branch).
This is a fact recorded in the artifact — not a gate.

**Coverage MUST include the Gherkin runner tests.** The coverage tool must be
configured to instrument every test binary that `verify.command` executes —
crucially including the Gherkin/Cucumber runner binary that executes the
`.feature` files under `givn/specs/`. A coverage run that instruments only
unit tests (and omits the `.feature` execution) is invalid: it undercounts
the real coverage and misleads the gap classification below. If the project's
coverage tool does not automatically pick up the Gherkin runner binary,
configure it to do so (most tools instrument all test binaries by default
when invoked over the whole test suite; verify this in the raw output).

**Redundant unit tests.** When a unit test exercises behaviour that is fully
covered by a `.feature` scenario (the scenario drives the same code path
through the black-box boundary), the unit test is redundant. The `.feature`
file is the source of truth. In reverse-engineering, removing redundant unit
tests is part of characterization — flag each removal in review.md under
"Spec corrections" or a dedicated "Redundant unit tests removed" list. (Keep
unit tests only for code paths that no `.feature` scenario reaches.)

**Coverage classification (three buckets — reverse-mode rules):**

These rules OVERRIDE the givn-review skill's gap resolution:

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
4. **Documentation corrections** — documentation (README, arc42, tasks.md,
   code comments) that was updated because it was wrong. For each:
   - Document and section
   - What it said before
   - What it says now
5. **Code smell markers** — all `TODO: REVERSE-ENGINEERING NOTE:` comments
   added during characterization. For each:
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
8. **Redundant unit tests removed** — unit tests deleted during
   characterization because a `.feature` scenario covers the same code path
   through the black-box boundary. For each:
   - The unit test (file + function name)
   - The `.feature` scenario that supersedes it
   - Confirm coverage for the region is unchanged after removal
9. **Test suite output and result** — final GREEN confirmation.
10. **Sign-off checklist** — all items checked.

**Sign-off checklist:**
- All scenarios pass (test runner exits 0).
- Coverage was measured across all test binaries, including the Gherkin
  runner that executes the `.feature` files (not unit tests alone).
- Every coverage gap classified and resolved (or justified).
- No `@wip` tags remain in the delta spec.
- The delta spec contains no implementation-layer detail.
- All spec corrections documented.
- All documentation corrections documented.
- All code smell markers listed.
- All suspected bugs flagged as follow-up candidates (NOT fixed).
- All redundant unit tests removed and listed (with the superseding
  `.feature` scenario).
- Behaviour-preservation invariant confirmed (all characterization tests
  GREEN after refactoring).

**Review result:** End the file with `REVIEW: PASS` or `REVIEW: FAIL`.
The archive gate reads this marker.
