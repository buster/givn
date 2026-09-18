# Design Review: re-eject-impact-assessment-template

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Approach | Re-eject or hand-patch the missing block? | agreed — re-eject from the 0.8.0 asset |
| 2 | Risk | Are there genuine project edits to preserve? | no — the diff is only the missing structured verdict block |
| 3 | Risk | Does `--force` touch anything else? | no — one file under `givn/artifacts/arc42-docs/` |
| 4 | Testability | How is the result proven? | agreed — byte comparison plus the verdict-block check |
| 5 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Repair method

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: whether the project copy matches the 0.8.0 contract.
- Context: the copy lacks the structured verdict block; the managed asset
  has it.
- Question: re-eject or hand-patch?
- Options:
  1. **Re-eject with `--force`.** The file comes from the managed asset.
     Meaning: one command.
     In practice: byte-identical to the asset.
     Gain: no drift.
     Cost: none.
  2. **Hand-patch the block.** Add the missing lines.
     Meaning: the copy stays hand-maintained.
     In practice: future asset changes drift again.
     Gain: smaller diff.
     Cost: drift.
- Resolution: **Option 1** (the migration bundle's Phase 5 rule). Rejected:
  Option 2.

### Q2 — Genuine edits

- Status: closed 2026-09-18 (inspection)
- Blocks: whether `--force` discards project intent.
- Context: the diff against the managed asset shows only the missing block.
- Question: any edit to re-apply after the eject?
- Options:
  1. **None.** The copy is the old template.
     Meaning: a clean re-eject.
     In practice: no follow-up edit.
     Gain: truthful.
     Cost: none.
  2. **Assume edits exist.** Merge manually.
     Meaning: unnecessary work.
     In practice: none.
     Gain: caution.
     Cost: none.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Blast radius

- Status: closed 2026-09-18 (codebase inspection)
- Blocks: whether the eject touches other files.
- Context: the eject target writes only
  `givn/artifacts/arc42-docs/impact-assessment-template.md`.
- Question: any other facet to re-eject?
- Options:
  1. **One file.** `givn/skills/` is empty; config carries settings.
     Meaning: the eject is the whole change.
     In practice: no other facet.
     Gain: minimal.
     Cost: none.
  2. **Re-eject every facet.** Broad sweep.
     Meaning: noise.
     In practice: no benefit.
     Gain: none.
     Cost: clutter.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Testability

- Status: closed 2026-09-18 (operator)
- Blocks: the evidence.
- Context: no spec covers the project copy.
- Question: what proves the repair?
- Options:
  1. **Byte comparison and the verdict-block check.** `cmp` plus `rg`.
     Meaning: reproducible evidence.
     In practice: recorded in tasks.md.
     Gain: verifiable.
     Cost: none.
  2. **Inspection only.** Read the file.
     Meaning: weaker.
     In practice: no command trail.
     Gain: none.
     Cost: unproven.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: the arc42 assessment.
- Context: the repair restores a generated file.
- Question: any Yes row?
- Options:
  1. **All No.** No chapter text changes.
     Meaning: truthful.
     In practice: none.
     Gain: none.
     Cost: none.
  2. **Chapter 8 Yes.** Document the template.
     Meaning: duplication.
     In practice: drift.
     Gain: none.
     Cost: maintenance.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Risk | The project copy differs from the asset only by the missing structured verdict block. |
| Consistency | `givn/skills/` is empty and `givn/config.yaml` carries settings only. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | No use-case delta; the repair restores an ejected template | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | No user-visible surface; `none` with the reason | accepted |
| Actor and Persona distinction | No Gherkin actors introduced | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| none | The repair restores a generated documentation template; no user-visible surface. | accepted |

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
- [x] No qualified ADR candidate; the repair aligns a file with an existing
      contract.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the repair.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change re-eject-impact-assessment-template` has no parse
      errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
