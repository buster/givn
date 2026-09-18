# Design Review: stamp-change-seeds

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Status value | `implemented in` or `withdrawn`? | agreed — all seven were implemented by their archived changes |
| 2 | Completeness | How much option treatment is reconstructed? | agreed — options with meaning, in practice, gain, and cost from the recorded material; the full dialog stays in the topic session |
| 3 | Consistency | Are the field names the schema's? | agreed — `Use-case ID`, `Topic`, `Personas`, `Suggested start`, `Handoff` |
| 4 | Completeness | Where does `Suggested start` come from? | agreed — the recorded dependencies (`scm-backend` first) |
| 5 | Consistency | Do open questions carry their resolutions? | agreed — Q10 and Q12 are marked resolved/moved |
| 6 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Status value

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: the provenance record.
- Context: all seven seeds were implemented by archived changes of the same
  ids.
- Question: `implemented in` or `withdrawn`?
- Options:
  1. **`implemented in <change-id>`.** Each seed names the archived change
     that realized it.
     Meaning: the provenance chain closes.
     In practice: `givn lint` stops reporting `[SEED ]`.
     Gain: machine-visible history.
     Cost: none.
  2. **`withdrawn`.** Treat the seeds as superseded by the shipped changes.
     Meaning: the seeds claim nothing shipped.
     In practice: a false record.
     Gain: none.
     Cost: wrong provenance.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — Option treatment depth

- Status: closed 2026-09-18 (operator)
- Blocks: how much is reconstructed retrospectively.
- Context: the schema's option treatment is four-part; the recorded material
  is the questions log and the seed bullets.
- Question: full four-part treatment or the recorded substance?
- Options:
  1. **Recorded substance.** Context, options with meaning and practical
     effect, gain, cost where recorded, and Chosen/Rejected with reasons.
     Meaning: no invention; the session record remains the dialog source.
     In practice: each decision is readable and auditable.
     Gain: truthful.
     Cost: some options carry no recorded gain/cost line.
  2. **Full four-part treatment for all 28 decisions.** Reconstruct every
     gain and cost.
     Meaning: partly invented detail.
     In practice: fabricated dialog.
     Gain: schema-identical shape.
     Cost: invention.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Field names

- Status: closed 2026-09-18 (operator)
- Blocks: schema conformance.
- Context: the seeds use bold labels (`**Stable use-case ID:**`).
- Question: normalize to the schema names?
- Options:
  1. **Normalize.** `Use-case ID`, `Topic`, `Personas`, `Suggested start`,
     `Handoff`.
     Meaning: the schema's names, no bold.
     In practice: a reader can compare seeds directly.
     Gain: consistency.
     Cost: none.
  2. **Keep the old labels.** Minimal diff.
     Meaning: mixed dialects.
     In practice: schema conformance stays partial.
     Gain: smaller diff.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Suggested start

- Status: closed 2026-09-18 (operator)
- Blocks: the missing dependency field.
- Context: the scope notes record the dependencies.
- Question: how is `Suggested start` derived?
- Options:
  1. **From the recorded dependencies.** `scm-backend` first; the others
     after it, with the "may be folded" notes preserved.
     Meaning: the historical order is stated.
     In practice: no new ordering decision.
     Gain: grounded.
     Cost: none.
  2. **Invent an order.** Pick a sequence.
     Meaning: a new decision in a provenance repair.
     In practice: unsupported.
     Gain: none.
     Cost: fabrication.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Open questions

- Status: closed 2026-09-18 (operator)
- Blocks: whether the open-questions section stays useful.
- Context: the seeds' open questions (Q10, Q12) were resolved during the
  topic's work.
- Question: carry them how?
- Options:
  1. **Carry them with their resolutions.** The question text plus the
     recorded outcome.
     Meaning: a reader sees both.
     In practice: no dangling "open" items on consumed seeds.
     Gain: honest.
     Cost: none.
  2. **Drop them.** The seeds are implemented.
     Meaning: the record loses the question.
     In practice: incomplete provenance.
     Gain: shorter.
     Cost: lost context.
- Resolution: **Option 1**. Rejected: Option 2.

### Q6 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: the arc42 assessment.
- Context: no durable chapter changes.
- Question: any Yes row?
- Options:
  1. **All No.** The conversion is provenance.
     Meaning: no chapter edits.
     In practice: truthful.
     Gain: none.
     Cost: none.
  2. **Chapter 12.** Add seed terms.
     Meaning: no new terms.
     In practice: none.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Completeness | All seven seeds map to archived changes of the same ids. |
| Completeness | The questions log holds each decision's resolution; the seeds hold the decision bullets. |
| Testability | `givn lint` reports seven `[SEED ]` advisories before the conversion and none after. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | No use-case delta; the seeds record realized contracts | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | No user-visible surface; `none` with the reason | accepted |
| Actor and Persona distinction | No Gherkin actors introduced | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| none | The repair edits ideation provenance; no user-visible surface. | accepted |

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
- [x] No qualified ADR candidate; the change records realized decisions.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the conversion scope.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change stamp-change-seeds` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
