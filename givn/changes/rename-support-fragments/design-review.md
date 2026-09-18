# Design Review: rename-support-fragments

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Operator decision | Approve the fourteen final names? | agreed — pre-approved in the inventory's before/after list (2026-09-18) |
| 2 | Completeness | Is the orphaned `config-overlay` handled? | agreed — renamed and owned in `fragment.md` |
| 3 | Risk | Do any references break? | one test path updated; no other reference exists |
| 4 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Names

- Status: closed 2026-09-18 (operator, inventory before/after list)
- Blocks: the final names for the fourteen capabilities.
- Context: the migration's full-list approval named them.
- Question: confirm the fourteen names?
- Options:
  1. **Confirm.** Behavior names replace the mechanism, channel, and
     change-shaped names.
     Meaning: R8 holds across the fragment corpus.
     In practice: fourteen files move.
     Gain: model-conformant.
     Cost: none.
  2. **Adjust.** Give corrections.
     Meaning: same repair, different labels.
     In practice: one more round.
     Gain: preferred wording.
     Cost: none.
- Resolution: **Option 1** (pre-approved 2026-09-18). Rejected: Option 2.

### Q2 — Orphan

- Status: closed 2026-09-18 (operator)
- Blocks: whether `config-overlay` gains an owner.
- Context: the file exists but `fragment.md` omits it.
- Question: own it or retire it?
- Options:
  1. **Own it as `customize-the-lifecycle-graph`.** The behavior is real
     (manifest overlay) and specified.
     Meaning: the capability is listed and named.
     In practice: one list entry.
     Gain: no orphan.
     Cost: none.
  2. **Retire it.** The behavior is covered elsewhere.
     Meaning: the scenarios disappear.
     In practice: a retirement delta.
     Gain: fewer capabilities.
     Cost: lost coverage.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — References

- Status: closed 2026-09-18 (codebase exploration)
- Blocks: whether the renames break references.
- Question: which references exist?
- Options:
  1. **One test path.** The register-history step names
     `spec-explore-permanent.feature`; it is updated.
     Meaning: no other reference exists.
     In practice: one edit.
     Gain: complete.
     Cost: none.
  2. **Assume none.** Risk a broken suite.
     Meaning: the suite fails.
     In practice: repair after.
     Gain: none.
     Cost: rework.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: the arc42 assessment.
- Context: no runtime change.
- Question: any Yes row?
- Options:
  1. **All No.** Truthful.
     Meaning: no chapter edits.
     In practice: none.
     Gain: none.
     Cost: none.
  2. **Chapter 5 Yes.** The fragment list changed.
     Meaning: no component changed.
     In practice: none.
     Gain: none.
     Cost: noise.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| References | Only `fragment.md` and one test step name the old capability files; test comments and fixture change ids are historical and stay. |
| Risk | The fragment narrative's capability list is the ownership record; the rename updates it. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Fourteen capabilities renamed; the orphan gains an owner | pre-approved |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | No user-visible surface; `none` with the reason | accepted |
| Actor and Persona distinction | No Gherkin actors introduced | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| none | The rename repairs capability names; no user-visible surface. | accepted |

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
- [x] No qualified ADR candidate; the change follows the model.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the approved names.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change rename-support-fragments` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
