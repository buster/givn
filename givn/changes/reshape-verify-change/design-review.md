# Design Review: reshape-verify-change

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Operator decision | Approve the three final names? | agreed — pre-approved in the inventory's before/after list (2026-09-18) |
| 2 | Model | Does the Goal need a split? | no — one check outcome; the compound object is not two verb phrases |
| 3 | Discipline | Do the rewrites pass R5 and R7? | agreed — step-anchored extensions; outcome language in Trigger and Guarantee |
| 4 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Names

- Status: closed 2026-09-18 (operator, inventory before/after list)
- Blocks: the final names for the three capabilities.
- Context: the migration's full-list approval named them with use-case
  context available here.
- Question: confirm `check-specification-health`,
  `resolve-change-directories`, and `validate-artifact-ids`?
- Options:
  1. **Confirm.** The names describe the behavior, not the command or the
     mechanism.
     Meaning: R8 holds.
     In practice: three feature files move.
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
- Blocks: whether the use case splits.
- Context: the Goal's object is compound ("valid, complete, and free of
  unresolved overlap") but the verb is one ("Check").
- Question: split or keep?
- Options:
  1. **Keep.** One check outcome; the closure test holds (the developer
     stops after receiving the result).
     Meaning: one goal.
     In practice: no new use case.
     Gain: no artificial split.
     Cost: none.
  2. **Split.** Three checks become three goals.
     Meaning: the verification surface fragments.
     In practice: three use cases.
     Gain: mechanical.
     Cost: artificial.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Discipline

- Status: closed 2026-09-18 (review criteria)
- Blocks: whether the rewrites pass.
- Context: R5 and R7 apply.
- Question: what was verified?
- Options:
  1. **Criterion by criterion.** Both extensions name their step,
     condition, handling, and guarantee; the Trigger, Main flow, and
     Success guarantee use outcome language ("requests verification",
     "reads", "reports", "receives a complete verification result").
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
  2. **Chapter 8 Yes.** Restate the exit codes.
     Meaning: unchanged.
     In practice: none.
     Gain: none.
     Cost: noise.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Consistency | No durable artifact references the three old capability names outside verify-change. |
| Consistency | The `givn lint` command name and its exit codes are asserted by the capability's scenarios and stay unchanged. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Three capabilities renamed; every capability keeps one owner | pre-approved |
| Typed Includes / Extends relationships | No relationship change; `fragment: support` stays | accepted |
| Confirmed Persona coverage | `developer--primary-user` reviews the surface | accepted |
| Actor and Persona distinction | The Reviewer is a supporting actor, not a Persona | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Requests verification and reads the result | narrative |

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
- [x] `givn lint --change reshape-verify-change` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
