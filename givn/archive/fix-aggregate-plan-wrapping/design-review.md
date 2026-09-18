# Design Review: fix-aggregate-plan-wrapping

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Approach | Fix the source or hand-patch generated plans? | agreed — fix the source; generated plans are snapshots |
| 2 | Testability | What pins the spacing? | agreed — a unit test on the generated proposal |
| 3 | Blast radius | Does the fix touch behavior? | no — string spacing only |
| 4 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Approach

- Status: closed 2026-09-18 (operator, dogfooding finding)
- Blocks: whether future plans stay correct.
- Context: the defect is in the Rust wrapper; generated plans already exist.
- Question: source fix or patch generated files?
- Meaning: a source fix corrects every future plan; patching fixes one
  snapshot.
- Options:
  1. **Fix the source.** Four spaces plus a test.
     Meaning: all future plans read correctly.
     In practice: one source edit.
     Gain: durable.
     Cost: none.
  2. **Patch the generated plan only.** Edit the snapshot text.
     Meaning: the next plan has the defect again.
     In practice: cosmetic only.
     Gain: no code change.
     Cost: the defect persists.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — Testability

- Status: closed 2026-09-18 (operator)
- Blocks: how the spacing is pinned.
- Context: no existing test asserts the wrapper text.
- Question: unit test or no test?
- Options:
  1. **Unit test on the generated proposal.** Assert the spaced phrases and
     the absence of the four joined forms.
     Meaning: the regression cannot return silently.
     In practice: one test beside the route test.
     Gain: pinned.
     Cost: none.
  2. **No test.** The fix is obvious.
     Meaning: a future edit can regress it.
     In practice: none.
     Gain: smaller diff.
     Cost: unguarded.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Blast radius

- Status: closed 2026-09-18 (codebase inspection)
- Blocks: whether behavior changes.
- Context: the wrapper is a format string.
- Question: any behavioral risk?
- Options:
  1. **None.** Only spaces change; placeholders and structure stay.
     Meaning: a behavior-preserving fix.
     In practice: the route test and the full suite stay green.
     Gain: safe.
     Cost: none.
  2. **Regenerate the catalog.** Touch the assets.
     Meaning: unnecessary churn.
     In practice: none.
     Gain: none.
     Cost: churn.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: the arc42 assessment.
- Context: no architecture change.
- Question: any Yes row?
- Options:
  1. **All No.** Truthful.
     Meaning: no chapter edits.
     In practice: none.
     Gain: none.
     Cost: none.
  2. **Chapter 6 Yes.** Describe the wrapper.
     Meaning: duplication.
     In practice: drift.
     Gain: none.
     Cost: maintenance.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Testability | The existing route test in `migration.rs` shows how to construct bundles in tests. |
| Blast radius | The design wrapper already carries the trailing spaces; only the proposal wrapper is defective. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | No use-case delta; the fix corrects generated prose | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | No user-visible surface; `none` with the reason | accepted |
| Actor and Persona distinction | No Gherkin actors introduced | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| none | The fix corrects generated prose; no user-visible surface. | accepted |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | — |
| 2 | 02 architecture-constraints | No | No | Yes | — |
| 3 | 03 context-and-scope | No | No | Yes | — |
| 4 | 04 solution-strategy | No | No | Yes | — |
| 5 | 05 building-block-view | No | No | Yes | — |
| 6 | 06 runtime-view | No | No | Yes | — |
| 7 | 07 deployment-view | No | No | Yes | — |
| 8 | 08 crosscutting-concepts | No | No | Yes | — |
| 9 | 09 architecture-decisions | No | No | Yes | — |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | No | No | Yes | — |
| 12 | 12 glossary | No | No | Yes | — |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; the fix is a string correction.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the fix and the test.
- [x] `specs` is intentionally skipped; the unit test is the regression
      proof.
- [x] `givn lint --change fix-aggregate-plan-wrapping` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
