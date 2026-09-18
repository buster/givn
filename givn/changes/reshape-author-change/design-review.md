# Design Review: reshape-author-change

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Operator decision | Approve the split and the final names? | agreed — `initialize-project` + `create-change`, three capability renames |
| 2 | Model | Is the summary's conjunction allowed? | agreed — a `+` summary's coverage is the union of its children |
| 3 | Model | Where do the guidance capabilities live? | agreed — under `create-change` |
| 4 | Model | Do the children include the support fragment? | agreed — each leaf includes `fragment: support` |
| 5 | Discipline | Do the rewrites pass R1, R2, R5, R6, R7, R9? | agreed — verified criterion by criterion |
| 6 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Names

- Status: closed 2026-09-18 (operator)
- Blocks: the final names for the split and the renames.
- Context: the migration decision defers the final names to this follow-up's
  design-review.
- Question: approve the proposed names?
- Options:
  1. **Approve.** `author-change` stays the summary; children
     `initialize-project` and `create-change`; `new-command` →
     `create-a-change`, `init-agent-target-selection` →
     `initialize-a-project`, `install-extras` → `enable-optional-addons`.
     Meaning: goal-shaped names per the model.
     In practice: `givn spec tree` shows the summary and two children.
     Gain: conformance.
     Cost: none.
  2. **Adjust.** Give corrected names.
     Meaning: the same split with different labels.
     In practice: reference churn in the implementation.
     Gain: preferred wording.
     Cost: one more round.
- Resolution: **Option 1** (operator-approved 2026-09-18). Rejected:
  Option 2.

### Q2 — Summary conjunction

- Status: closed 2026-09-18 (model rule)
- Blocks: whether the summary may have a union goal and guarantee.
- Context: R1 fails a leaf whose success guarantee is a conjunction.
- Question: is the conjunction allowed on the `+`?
- Options:
  1. **Yes.** Rule 3: the `+` coverage is the union of its children; the
     promotion rule moves the conjunction up.
     Meaning: the summary states the union.
     In practice: no direct capabilities.
     Gain: model-conformant.
     Cost: none.
  2. **No.** Keep the summary single-goal.
     Meaning: the split is incomplete.
     In practice: the summary's guarantee still conjures two outcomes.
     Gain: none.
     Cost: an unrepairable R1.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Guidance capabilities

- Status: closed 2026-09-18 (operator)
- Blocks: where `prose-consolidation` and `visual-design-contract` live.
- Context: they are read while authoring a change.
- Question: `create-change` or a third child?
- Options:
  1. **`create-change`.** The goal "ready to carry from intent to executable
     specification" covers the guidance.
     Meaning: three capabilities under the leaf.
     In practice: the guidance scenarios keep their home.
     Gain: no extra use case.
     Cost: none.
  2. **A third child.** "Read the guidance".
     Meaning: a use case for reading.
     In practice: an actor goal that is really a step.
     Gain: none.
     Cost: over-splitting.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Fragment includes

- Status: closed 2026-09-18 (model rule)
- Blocks: whether the children include `support`.
- Context: the parent included `fragment: support`; a `+` includes only
  child use cases.
- Question: who includes the fragment?
- Options:
  1. **Each leaf includes it.** Rule 8: leaf-to-fragment is allowed.
     Meaning: both children carry the include.
     In practice: the fragment's capabilities stay reachable.
     Gain: model-conformant.
     Cost: none.
  2. **The summary includes it.** Rule 8 forbids summary-to-fragment.
     Meaning: an invalid include.
     In practice: a model violation.
     Gain: fewer lines.
     Cost: R10.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Rewrite discipline

- Status: closed 2026-09-18 (review criteria)
- Blocks: whether the rewritten narratives pass the criteria.
- Context: R1, R2, R5, R6, R7, and R9 apply.
- Question: what was verified?
- Options:
  1. **Criterion by criterion.** One goal and one primary actor per leaf;
     extensions carry step, condition, handling, and the protected
     guarantee; preconditions hold before the trigger; Goal, Trigger, Main
     flow, and Guarantees are mechanism-free; each leaf's flow names fewer
     than three actor goals.
     Meaning: the new documents pass.
     In practice: verified in this review.
     Gain: conformance.
     Cost: none.
  2. **Spot-check only.** Trust the rewrite.
     Meaning: possible regressions.
     In practice: none.
     Gain: faster.
     Cost: risk.
- Resolution: **Option 1**. Rejected: Option 2.

### Q6 — Architecture documentation

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
  2. **Chapter 3 Yes.** The use-case set changed.
     Meaning: chapter 3 describes actors, not the use-case tree.
     In practice: no edit needed.
     Gain: none.
     Cost: none.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Model | No `+` summary exists yet; the tree parser accepts a zero-capability use case and shows it in `givn spec tree`. |
| Discipline | The use-case parser requires one bullet per line in list sections; the narratives use single-line bullets. |
| Consistency | No reference to the old capability names remains outside the moved features. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | One leaf promoted to a summary with two children; every capability re-homed once | operator-approved |
| Typed Includes / Extends relationships | The summary includes the children; each leaf includes `fragment: support`; no Extends | accepted |
| Confirmed Persona coverage | All three Personas listed on the summary and both children | accepted |
| Actor and Persona distinction | Personas remain review lenses; the actors are the Developer and the Authoring agent | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Sets up projects and creates changes | narratives |
| agent--technical-role | Consumes init and guidance output | narratives |
| givn-maintainer--owner | Owns the model the split follows | narratives |

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
- [x] `givn lint --change reshape-author-change` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
