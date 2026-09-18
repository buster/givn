# Design Review: convert-topic-persona-copies

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Approach | Delete the copies or convert them to reuse records? | agreed — convert; the topic file stays as historical record |
| 2 | Base reference | Hash or date? | agreed — the permanent file's current short content hash |
| 3 | Completeness | Do the topic copies add any entries? | no — they are byte-identical copies; the note says so |
| 4 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Convert or delete

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: the end state of the topic copies.
- Context: the reuse rule says the topic file contains a reuse header plus
  new entries only; the glossary's persona-promotion entry says the topic
  file stays as historical record.
- Question: convert or delete?
- Options:
  1. **Convert to a reuse record.** Header plus an honest no-entries note.
     Meaning: the topic record survives without the biography.
     In practice: one short file per persona.
     Gain: history stays, biography lives once.
     Cost: none.
  2. **Delete the copies.** The biography is in the permanent file.
     Meaning: the topic record is lost.
     In practice: no file.
     Gain: fewer files.
     Cost: lost provenance.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — Base reference

- Status: closed 2026-09-18 (operator)
- Blocks: what `base` names.
- Context: the schema allows a short hash or a date.
- Question: which?
- Options:
  1. **The permanent file's short content hash.** Verifiable against the
     file.
     Meaning: `sha256sum` proves the base.
     In practice: `3e48068e`, `93a37080`, `a3b1fa11`.
     Gain: reproducible.
     Cost: none.
  2. **The handoff date.** 2026-09-14.
     Meaning: coarser.
     In practice: no content binding.
     Gain: simpler.
     Cost: unverifiable.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — New entries

- Status: closed 2026-09-18 (inspection)
- Blocks: whether topic entries exist to carry.
- Context: the copies are byte-identical to the permanent files.
- Question: any topic-specific entries?
- Options:
  1. **None; say so.** The note states the topic reused the persona
     unchanged.
     Meaning: no invented stances.
     In practice: honest.
     Gain: truthful.
     Cost: none.
  2. **Invent review entries.** Fill the running record.
     Meaning: fabricated evidence.
     In practice: wrong.
     Gain: fuller file.
     Cost: fabrication.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Architecture documentation

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
  2. **Chapter 12 Yes.** Restate the rule.
     Meaning: duplication.
     In practice: drift.
     Gain: none.
     Cost: maintenance.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Completeness | The three topic copies are byte-identical to the permanent files (diff reports zero lines). |
| Consistency | The glossary's persona-promotion entry already states that the topic file stays and the biography lives once. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | No use-case delta; the change converts topic persona records | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | The three subjects; no biography is copied | accepted |
| Actor and Persona distinction | No Gherkin actors introduced | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Topic copy converted | reuse record |
| agent--technical-role | Topic copy converted | reuse record |
| givn-maintainer--owner | Topic copy converted | reuse record |

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
- [x] `design.md` reflects the reuse records.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change convert-topic-persona-copies` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
