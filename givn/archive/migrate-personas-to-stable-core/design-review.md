# Design Review: migrate-personas-to-stable-core

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Schema | Where do interests, goals, and pain points go? | agreed — folded into the stable-core fields; the schema has no such sections |
| 2 | Identity | Is each merge justified by a shared criterion? | agreed — the Role line states the shared criterion |
| 3 | Completeness | Is the log preserved? | agreed — every dated entry moves to the running record |
| 4 | Completeness | Are topic demands separated? | agreed — topic stances carry the multi-scm-support and kiekste demands |
| 5 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Old sections

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: whether persona content is dropped.
- Context: the 0.8.0 schema has stable core, running record, and topic
  stances; the old files have interests, goals, and pain points.
- Question: fold or drop?
- Options:
  1. **Fold into the stable-core fields.** Interests inform the authority
     boundary and evaluation capability; pain points inform vocabulary
     refusals; goals inform the success criterion.
     Meaning: no valuable content is lost.
     In practice: the five fields carry the substance.
     Gain: schema conformance without loss.
     Cost: none.
  2. **Drop the old sections.** Keep only the fields that map directly.
     Meaning: persona context shrinks.
     In practice: reviewers lose nuance.
     Gain: smaller files.
     Cost: lost context.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — Identity criteria

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: whether the merges survive the disjunctive test.
- Context: each persona merges two segments by user decision; the stable core
  must state the shared criterion.
- Question: where is the criterion stated?
- Options:
  1. **In the Role line.** The Role names the segments and the shared
     criterion (authority boundary and evaluation capability, or vocabulary).
     Meaning: the merge is justified where the persona is introduced.
     In practice: a reviewer can falsify the merge.
     Gain: auditable.
     Cost: none.
  2. **In a separate note.** Add a merge note below the core.
     Meaning: the criterion is not part of the schema.
     In practice: drift risk.
     Gain: none.
     Cost: schema noise.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Running record

- Status: closed 2026-09-18 (operator)
- Blocks: whether the dated log survives.
- Context: the old `Decision, need, and demand log` tables carry dated
  entries with sources.
- Question: preserve or summarize?
- Options:
  1. **Preserve every dated entry.** One running-record bullet per log row,
     with the source in parentheses.
     Meaning: append-only history starts intact.
     In practice: four entries per persona.
     Gain: no lost decisions.
     Cost: none.
  2. **Summarize.** One line per theme.
     Meaning: dates and sources are lost.
     In practice: weaker provenance.
     Gain: shorter.
     Cost: lost evidence.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Topic stances

- Status: closed 2026-09-18 (operator)
- Blocks: where topic demands live.
- Context: the schema separates topic-specific demands from the stable core.
- Question: which demands are stances?
- Options:
  1. **The recorded topic demands.** multi-scm-support (one mental model;
     contract shape stays inside givn) and kiekste hosting (out of scope).
     Meaning: dated stances, never in the core.
     In practice: one or two bullets per persona.
     Gain: schema conformance.
     Cost: none.
  2. **No stances.** The record is enough.
     Meaning: topic demands are lost.
     In practice: none.
     Gain: shorter.
     Cost: lost context.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: the arc42 assessment.
- Context: no durable chapter changes.
- Question: any Yes row?
- Options:
  1. **All No.** Truthful.
     Meaning: no chapter edits.
     In practice: none.
     Gain: none.
     Cost: none.
  2. **Chapter 12 Yes.** Restate the persona rule.
     Meaning: duplication.
     In practice: drift.
     Gain: none.
     Cost: maintenance.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Completeness | Every old log row is dated 2026-09-14 with a source. |
| Completeness | The developer persona's kiekste interest is a scope stance, not a core trait. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | No use-case delta; the change converts persona files | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | The three confirmed Personas are the subjects | accepted |
| Actor and Persona distinction | Personas remain review lenses; no Gherkin actor changes | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Converted | stable core, running record, topic stances |
| agent--technical-role | Converted | stable core, running record, topic stances |
| givn-maintainer--owner | Converted | stable core, running record, topic stances |

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
- [x] No qualified ADR candidate; the change edits persona records.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the mapping.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change migrate-personas-to-stable-core` has no parse
      errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
