# Design Review: reshape-operate-product

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Operator decision | Approve the two final names? | agreed — pre-approved in the inventory's before/after list (2026-09-18) |
| 2 | Model | Split start from observe, or rewrite the Goal? | agreed — one goal; rewrite as one phrase |
| 3 | Discipline | Do the rewrites pass R5 and R7? | agreed — anchored extension; outcome language throughout |
| 4 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Names

- Status: closed 2026-09-18 (operator, inventory before/after list)
- Blocks: the final names for the two capabilities.
- Context: the migration's full-list approval named them.
- Question: confirm `declare-the-product-start` and
  `start-the-declared-product`?
- Options:
  1. **Confirm.** The names describe the behavior, not the command.
     Meaning: R8 holds.
     In practice: two feature files move.
     Gain: model-conformant.
     Cost: none.
  2. **Adjust.** Give corrections.
     Meaning: same repair, different labels.
     In practice: one more round.
     Gain: preferred wording.
     Cost: none.
- Resolution: **Option 1** (pre-approved 2026-09-18). Rejected: Option 2.

### Q2 — Goal repair

- Status: closed 2026-09-18 (model rule)
- Blocks: whether the use case splits.
- Context: R1 falsifies "Start … and observe …".
- Question: rewrite or split?
- Options:
  1. **Rewrite.** "Run the declared product to a reported outcome." The
     observation is the run's outcome, not a second goal.
     Meaning: one goal.
     In practice: one sentence.
     Gain: no artificial split.
     Cost: none.
  2. **Split.** A start use case and an observe use case.
     Meaning: two halves of one operator action.
     In practice: new use case and moved capabilities.
     Gain: mechanical R1.
     Cost: artificial.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Discipline

- Status: closed 2026-09-18 (review criteria)
- Blocks: whether the rewrites pass.
- Context: R5 and R7 apply.
- Question: what was verified?
- Options:
  1. **Criterion by criterion.** The extension names step 1, its condition,
     its handling, and the minimal guarantee; the Trigger ("starts the
     declared product"), Main flow ("waits for the declared readiness",
     "reports the product outcome"), and Success guarantee ("observes the
     product's lifecycle outcome") are mechanism-free.
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
  2. **Chapter 6 Yes.** Restate the run runtime.
     Meaning: unchanged.
     In practice: none.
     Gain: none.
     Cost: noise.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Consistency | No durable artifact references the two old capability names outside operate-product. |
| Consistency | The `givn run` command and its contract scenarios are unchanged. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Two capabilities renamed; every capability keeps one owner | pre-approved |
| Typed Includes / Extends relationships | No relationship change; `fragment: support` stays | accepted |
| Confirmed Persona coverage | `developer--primary-user` reviews the surface | accepted |
| Actor and Persona distinction | The operator is the actor; no Persona change | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Operates the declared product | narrative |

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
- [x] `givn lint --change reshape-operate-product` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
