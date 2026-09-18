# Design Review: migrate-term-records

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Status assignment | Which terms are confirmed? | agreed — the nine merged into the glossary |
| 2 | Status assignment | What happens to the per-SCM terms? | agreed — rejected or candidate with reasons, never deleted |
| 3 | Evidence | Re-run the searches or carry the recorded evidence? | agreed — carry the migration's 2026-09-18 evidence |
| 4 | Consistency | One term per row? | agreed — the combined upgrade/plan row splits |
| 5 | Architecture documentation | Any chapter affected? | no — the glossary is unchanged |

### Q1 — Confirmed set

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: which rows claim confirmation.
- Context: Q10 merged the neutral terms plus SCM backend into the glossary.
- Question: which terms are confirmed?
- Options:
  1. **The nine in the glossary.** They carry definitions, contrasts, first
     uses, and the recorded evidence.
     Meaning: status matches the durable list.
     In practice: a reader can trace each to `12-glossary.md`.
     Gain: truthful.
     Cost: none.
  2. **All twelve.** Confirm the per-SCM terms too.
     Meaning: the seam terms claim durable status.
     In practice: they stay topic-local.
     Gain: none.
     Cost: false confirmation.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — Per-SCM terms

- Status: closed 2026-09-18 (operator)
- Blocks: the disposition of Hook, Check-in, Repository root, and Index.
- Context: Q10 keeps per-SCM terms in the backend seam; two terms are
  superseded by domain choices.
- Question: rejected or candidate?
- Options:
  1. **Reject the superseded two; keep the seam two as candidates.**
     Repository root is superseded by Checkout; Index/staging area is
     Git-only and replaced by "publish a subset"; Hook and Check-in stay
     qualified candidates.
     Meaning: each row states its disposition.
     In practice: no term is deleted.
     Gain: honest.
     Cost: none.
  2. **Reject all four.** Seam terms are not domain terms.
     Meaning: the seam loses its vocabulary record.
     In practice: the qualifier rules are lost.
     Gain: none.
     Cost: lost context.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Evidence

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: whether the searches are re-run.
- Context: the migration change recorded the commands and counts on
  2026-09-18 as its one direct edit.
- Question: re-run or carry?
- Options:
  1. **Carry the recorded evidence.** The cells keep the dated commands and
     counts.
     Meaning: the evidence is preserved verbatim.
     In practice: the follow-up adds statuses and columns.
     Gain: no duplicate work.
     Cost: none.
  2. **Re-run the searches.** Fresh evidence.
     Meaning: the cells carry a new date.
     In practice: the same result.
     Gain: freshness.
     Cost: redundant.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Row shape

- Status: closed 2026-09-18 (operator)
- Blocks: the combined upgrade/plan row.
- Context: the old table combined two terms in one row.
- Question: split or keep combined?
- Options:
  1. **Split.** One term per row, per the schema.
     Meaning: 13 rows.
     In practice: each term has its own status and evidence.
     Gain: schema conformance.
     Cost: none.
  2. **Keep combined.** Fewer rows.
     Meaning: two terms share a status cell.
     In practice: the schema breaks.
     Gain: smaller diff.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: the arc42 assessment.
- Context: the durable glossary does not change.
- Question: any Yes row?
- Options:
  1. **All No.** The glossary is unchanged.
     Meaning: no chapter edits.
     In practice: truthful.
     Gain: none.
     Cost: none.
  2. **Chapter 12 Yes.** Restate the term list.
     Meaning: duplication.
     In practice: drift risk.
     Gain: none.
     Cost: maintenance.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Consistency | The glossary holds the nine confirmed terms at lines 94-95 and 146-152. |
| Consistency | The migration recorded the collision evidence in the old table's cells on 2026-09-18. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | No use-case delta; the change repairs the topic's term records | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | No user-visible surface; `none` with the reason | accepted |
| Actor and Persona distinction | No Gherkin actors introduced | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| none | The repair edits the topic's term records; no user-visible surface. | accepted |

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
- [x] No qualified ADR candidate; the change records terms.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the status assignment.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change migrate-term-records` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
