# Design Review: reshape-inspect-change-state

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Operator decision | Approve the four final names? | agreed — pre-approved in the inventory's before/after list (2026-09-18) |
| 2 | Model | Does the Goal need repair? | no — one understanding outcome |
| 3 | Discipline | Does the rewrite pass R5 and R7? | agreed — anchored extension; outcome-language trigger |
| 4 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Names

- Status: closed 2026-09-18 (operator, inventory before/after list)
- Blocks: the final names for the four capabilities.
- Context: the migration's full-list approval named them.
- Question: confirm `report-change-status`, `validate-change-ids`,
  `show-skipped-artifacts`, and `read-task-progress`?
- Options:
  1. **Confirm.** Behavior names replace the command and fix names.
     Meaning: R8 holds.
     In practice: four feature files move.
     Gain: model-conformant.
     Cost: none.
  2. **Adjust.** Give corrections.
     Meaning: same repair, different labels.
     In practice: one more round.
     Gain: preferred wording.
     Cost: none.
- Resolution: **Option 1** (pre-approved 2026-09-18). Rejected: Option 2.

### Q2 — Goal

- Status: closed 2026-09-18 (model rule)
- Blocks: whether the Goal needs repair.
- Context: "Understand the current change and artifact state without reading
  workflow files by hand" is one understanding outcome.
- Question: rewrite or keep?
- Options:
  1. **Keep.** One goal; the minimal guarantee declares observation.
     Meaning: no R1 finding.
     In practice: unchanged.
     Gain: minimal churn.
     Cost: none.
  2. **Rewrite.** Shorten the goal.
     Meaning: cosmetic.
     In practice: none.
     Gain: none.
     Cost: churn.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Discipline

- Status: closed 2026-09-18 (review criteria)
- Blocks: whether the rewrite passes.
- Context: R5 applies; the trigger reads as outcome language.
- Question: what was verified?
- Options:
  1. **Criterion by criterion.** The extension names step 1, the invalid
     identity condition, the rejection handling, and the minimal guarantee;
     the Trigger says "requests the current change state".
     Meaning: conformance.
     In practice: verified here.
     Gain: model-conformant.
     Cost: none.
  2. **Spot-check.** Trust the rewrite.
     Meaning: possible gaps.
     In practice: none.
     Gain: faster.
     Cost: risk.
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
  2. **Chapter 8 Yes.** Restate status output.
     Meaning: unchanged.
     In practice: none.
     Gain: none.
     Cost: noise.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Consistency | No durable artifact references the four old capability names outside inspect-change-state. |
| Consistency | The `givn status` command and its output behavior are unchanged. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Four capabilities renamed; every capability keeps one owner | pre-approved |
| Typed Includes / Extends relationships | No relationship change; `fragment: support` stays | accepted |
| Confirmed Persona coverage | `developer--primary-user` reviews the surface | accepted |
| Actor and Persona distinction | No Persona change | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Inspects the change state | narrative |

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
- [x] `givn lint --change reshape-inspect-change-state` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
