# Design Review: reshape-archive-change

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Operator decision | Approve the rename and the include removal? | agreed — `preserve-scenario-docstrings`; `usecase: verify-change` removed |
| 2 | Model | Rewrite the Goal or split the use case? | agreed — archiving is one goal; rewrite as one phrase |
| 3 | Discipline | Do the eight extensions carry step, condition, handling, and guarantee? | agreed — mapped one by one |
| 4 | Discipline | Is the R7 wording domain language? | agreed — "version-control system"; glossary revisions stay |
| 5 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Dispositions

- Status: closed 2026-09-18 (operator)
- Blocks: the rename and the relationship repair.
- Context: the migration defers the final names and relationship dispositions
  to this follow-up.
- Question: approve the rename and the include removal?
- Options:
  1. **Approve both.** `preserve-scenario-docstrings`; Includes keep only
     `fragment: support` because the archive runs its own gates.
     Meaning: R8 and R10 are repaired.
     In practice: one rename and one include removed.
     Gain: model-conformant.
     Cost: none.
  2. **Rename only; keep the include.** Record R10 as an exception.
     Meaning: the leaf-to-leaf include stays.
     In practice: R10 remains falsified.
     Gain: no relationship edit.
     Cost: a known violation.
  3. **Model the reuse as a fragment.** Move reused behavior to a fragment.
     Meaning: a bigger restructure for no identified reuse.
     In practice: new fragment capability.
     Gain: none.
     Cost: unnecessary work.
- Resolution: **Option 1** (operator-approved 2026-09-18). Rejected:
  Options 2 and 3.

### Q2 — Goal repair

- Status: closed 2026-09-18 (model rule)
- Blocks: whether the use case splits.
- Context: R1 falsifies the two-verb Goal; the closure test asks whether the
  actor can stop after the first part.
- Question: rewrite or split?
- Options:
  1. **Rewrite as one phrase.** "Archive a completed change with proven
     publication." The actor cannot stop after publishing; the proof is part
     of the goal.
     Meaning: one goal, one outcome.
     In practice: two sentences rewritten.
     Gain: repairs R1 without a split.
     Cost: none.
  2. **Split into publish and prove.** Two use cases.
     Meaning: the actor's single goal becomes two.
     In practice: a new use case, moved capabilities.
     Gain: mechanical R1.
     Cost: an artificial split.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Extension mapping

- Status: closed 2026-09-18 (review criteria)
- Blocks: whether the extensions pass R5.
- Context: eight extensions lacked step references.
- Question: is every rewrite step-anchored?
- Options:
  1. **Yes.** Each names its main-flow step, its condition, its handling,
     and the guarantee it protects.
     Meaning: R5 holds.
     In practice: the mapping table in `design.md`.
     Gain: conformance.
     Cost: none.
  2. **Spot-check.** Trust the rewrite.
     Meaning: possible gaps.
     In practice: none.
     Gain: faster.
     Cost: risk.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Mechanism vocabulary

- Status: closed 2026-09-18 (review criteria)
- Blocks: whether the R7 rewrite is domain language.
- Context: "active backend" and "backend revision" appeared in the Main flow
  and the Success guarantee.
- Question: which words stay?
- Options:
  1. **"Version-control system" replaces "backend"; glossary revisions
     stay.** The confirmed terms (`revision`, `candidate revision`, `tested
     revision`) are domain language.
     Meaning: mechanism-free sections.
     In practice: the wording edit.
     Gain: R7 holds.
     Cost: none.
  2. **Replace revisions too.** Avoid every SCM-shaped word.
     Meaning: the domain's confirmed vocabulary disappears.
     In practice: vaguer text.
     Gain: none.
     Cost: glossary drift.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Architecture documentation

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
  2. **Chapter 12 Yes.** Add "version-control system".
     Meaning: it expands an existing term, not a new one.
     In practice: none.
     Gain: none.
     Cost: noise.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Model | Only `archive-change` includes `usecase: verify-change`; no other leaf-to-leaf include exists. |
| Discipline | The archive's post-merge gates run the project's runner, not the verify-change contract, so there is no unconditional reuse. |
| Consistency | The capability id resolves from the feature filename; the prose title was updated for coherence. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | One capability renamed; every capability keeps one owner | operator-approved |
| Typed Includes / Extends relationships | The leaf-to-leaf include removed; `fragment: support` stays; no Extends | operator-approved |
| Confirmed Persona coverage | The three Personas stay listed | accepted |
| Actor and Persona distinction | Personas remain review lenses | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Archives changes | narrative |
| agent--technical-role | Reads receipts and status | narrative |
| givn-maintainer--owner | Owns the archive contract | narrative |

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
- [x] `design.md` reflects the approved dispositions.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change reshape-archive-change` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
