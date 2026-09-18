# Design Review: confirm-persona-coverage

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Mapping | Which Personas per use case? | agreed — the actor's Persona only, operator-chosen |
| 2 | Mechanism | Direct edit or delta narrative? | agreed — delta narratives with the base hash; permanent files merge at archive |
| 3 | Completeness | Any use case left as `none`? | no — all five gain their actor's Persona |
| 4 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Mapping

- Status: closed 2026-09-18 (operator)
- Blocks: which Personas the five use cases name.
- Context: the proposed broader lists mixed stakeholders per surface.
- Question: actor-only or broader?
- Options:
  1. **The actor's Persona only.** verify-change → developer; operate-product
     → developer; inspect-change-state → developer;
     discover-specifications → agent; make-usecases-canonical → maintainer.
     Meaning: the primary stakeholder lens per surface.
     In practice: one Persona per use case.
     Gain: simple and defensible.
     Cost: secondary lenses are not named.
  2. **Broader lists.** Include the agent and maintainer on more surfaces.
     Meaning: more lenses.
     In practice: five lists of two to three Personas.
     Gain: wider review.
     Cost: naming lenses that do not review the surface.
- Resolution: **Option 1** (operator). Rejected: Option 2.

### Q2 — Mechanism

- Status: closed 2026-09-18 (operator)
- Blocks: how the narratives change.
- Context: use-case documents merge with a base-hash check.
- Question: direct edit or delta?
- Options:
  1. **Delta narratives with the base hash.** The permanent files change at
     archive.
     Meaning: the normal narrative flow.
     In practice: five files under the change's `specs/`.
     Gain: validated replacement.
     Cost: none.
  2. **Edit the permanent files now.** Immediate change.
     Meaning: the archive base-hash check would fail.
     In practice: blocked archive.
     Gain: immediate visibility.
     Cost: a broken gate.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Coverage

- Status: closed 2026-09-18 (operator)
- Blocks: whether any surface stays without a Persona.
- Context: five use cases declared `none`.
- Question: does any stay `none` with a reason?
- Options:
  1. **No; all five gain their actor's Persona.**
     Meaning: every user-visible surface has a lens.
     In practice: five corrected sections.
     Gain: full coverage.
     Cost: none.
  2. **Leave one as `none` with a reason.**
     Meaning: one surface stays uncovered.
     In practice: a recorded reason.
     Gain: none.
     Cost: an uncovered surface.
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
  2. **Chapter 12 Yes.** Restate the coverage rule.
     Meaning: duplication.
     In practice: drift.
     Gain: none.
     Cost: maintenance.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Mechanism | The archive merge validates a delta narrative against the current permanent file's content hash. |
| Completeness | The five use cases are the only ones declaring `none`. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Five use-case narratives corrected | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | The actor's Persona added per use case | operator-confirmed |
| Actor and Persona distinction | Personas are review lenses; actors unchanged | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | verify-change, operate-product, inspect-change-state | delta narratives |
| agent--technical-role | discover-specifications | delta narrative |
| givn-maintainer--owner | make-usecases-canonical | delta narrative |

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
- [x] No qualified ADR candidate; the change edits narratives.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the mapping and mechanics.
- [x] `specs` carries five narrative deltas; no new step is needed.
- [x] `givn lint --change confirm-persona-coverage` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
