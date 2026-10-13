# Instruction: review-reverse

Confirm characterization is complete, no behaviour changed during
refactoring, and all discovered bugs/code smells are documented.

## Step 0 (mandatory, first): fabrication audit

Same mechanical check as the prescriptive `review` instruction. Before
trusting tasks.md:

1. Grep every step definition file for empty/no-op bodies (`{}`, bare
   `pass`/`return`, no assertion reference). Any match = fabricated
   characterization.
2. Cross-check every checked `[x]` task has a commit whose diff contains
   real assertion code, not just the `.feature` file.
3. Confirm the runner's strict-mode proof is present and passing.
4. Any finding is unfinished characterization — reopen the task, redo it
   for real.

Record the result even when clean.

## Coverage mechanics (overlays the review instruction)

Read the review instruction served by `givn instructions review` first — it
defines the coverage gate (completeness, not percentage), gap classification,
and sign-off. The following overrides its rules for reverse-engineering.

Review does not run an instrumented pass. Coverage comes from the most recent
measured evidence — the archive verification receipt or the merged report —
and gaps are classified from that evidence plus static inspection; do not
write "not measured" as a substitute for classification. The evidence must
include the Gherkin runner binary as well as unit tests: instrumenting unit
tests alone undercounts and misleads classification. If no measured evidence
exists yet for this change, say so explicitly and classify from inspection —
the archive receipt will carry the measured result.

Redundant unit tests (behaviour already covered by a `.feature` scenario)
are removed as part of characterization — flag each removal under "Redundant
unit tests removed" below.

## Classification (reverse-mode — overrides the review instruction)

1. Dead code → flag as `@givn.removed` follow-up candidate. Do NOT delete.
2. Missing test coverage → flag as `@givn.added` follow-up candidate. Do NOT
   add during review.
3. Legitimately hard to test → concrete technical justification.

## Behaviour-preservation invariant

Re-run `verify.command` after all refactoring — all characterization tests
must still be GREEN; the archive verification receipt's regular scope is the
recorded evidence that they are. If any went RED during the change, the
invariant was
violated: flag scenario, refactor, and what happened. It should have been
reverted during characterization; if not, flag as a blocker.

## Mandatory reporting sections in review.md

1. Coverage — the most recent measured evidence used (source and totals) plus
   static-inspection notes.
2. Coverage gap table — bucket, region, action.
3. Spec corrections — scenario title, original inference, actual behaviour,
   root cause (documentation wrong / code ambiguous).
4. Documentation corrections — document/section, before, after.
5. Code smell markers — every `TODO: REVERSE-ENGINEERING NOTE:` added: file,
   line, what was misleading, actual behaviour, suggested refactor.
6. Suspected bugs — observed vs correct behaviour, scenario capturing the
   AS-IS behaviour, `@givn.modified` follow-up candidate.
7. Boundary failures — scenario title, what was attempted, why the boundary
   failed, suggested seam/boundary change.
8. Redundant unit tests removed — test (file + function), superseding
   scenario, confirmation coverage is unchanged.
9. Scenario execution evidence and result — final GREEN confirmation recorded
   per scope (review does not re-run the suite).
10. Sign-off checklist — all items checked.

## Sign-off checklist

- Fabrication audit: clean.
- Scenario execution evidence recorded per scope; review does not re-run the
  suite.
- Coverage evidence (most recent measured report or archive verification
  receipt) includes the Gherkin runner binary; gaps classified from that
  evidence plus static inspection.
- Every coverage gap classified and resolved.
- No `@wip` tags remain. No implementation-layer detail in the spec.
- All spec corrections, documentation corrections, code smell markers, and
  suspected bugs documented (bugs flagged, never fixed).
- All redundant unit tests removed and listed with their superseding scenario.
- Behaviour-preservation invariant confirmed.

## Result

End the file with `REVIEW: PASS` or `REVIEW: FAIL`. The archive gate reads
this marker.
