# Design Review: resync-adr-template

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Approach | Delete and re-scaffold, or hand-edit the stale copy? | agreed — delete and re-scaffold from the 0.8.0 asset |
| 2 | Risk | Does `givn addons enable arc42` overwrite other chapters? | no — the scaffolder writes only missing files |
| 3 | Testability | How is the restored content proven? | agreed — byte comparison plus the existing scenario |
| 4 | Architecture documentation | Does any durable chapter change? | no — all 12 rows No |

### Q1 — Repair method

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: whether the restored template matches the 0.8.0 contract.
- Context: the project copy carries the old gate's pre-passed verdict; the
  managed asset ships blank.
- Question: hand-edit or re-scaffold?
- Options:
  1. **Delete and re-scaffold.** The file comes from the managed asset, so it
     cannot drift from it.
     Meaning: one delete plus `givn addons enable arc42`.
     In practice: byte-identical to the asset.
     Gain: no hand-maintained copy.
     Cost: none.
  2. **Hand-edit the verdict lines.** Keep the file, blank the values.
     Meaning: the rest of the stale copy may still differ from the asset.
     In practice: a partial repair.
     Gain: no delete.
     Cost: unverified drift.
- Resolution: **Option 1** (the migration bundle's Phase 3 step 4). Rejected:
  Option 2.

### Q2 — Scaffolder safety

- Status: closed 2026-09-18 (codebase inspection)
- Blocks: whether re-enabling touches other chapters.
- Context: `src/cli/addons.rs` writes only embedded chapter paths whose
  destination does not exist.
- Question: is a backup needed?
- Options:
  1. **No backup.** All other chapters exist, so only the deleted template is
     written.
     Meaning: the re-enable prints one created file.
     In practice: no other file changes.
     Gain: minimal diff.
     Cost: none.
  2. **Back up all chapters first.** Defensive copy.
     Meaning: noise in the tree.
     In practice: no benefit.
     Gain: paranoia.
     Cost: clutter.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Testability

- Status: closed 2026-09-18 (operator)
- Blocks: how the repair is proven.
- Context: the existing scenario reads the managed asset, not the project
  copy.
- Question: what evidence does the change record?
- Options:
  1. **Byte comparison plus the existing scenario.** `cmp` proves the project
     copy equals the asset; the scenario proves the asset is blank.
     Meaning: two commands and the suite.
     In practice: a verifiable repair.
     Gain: complete.
     Cost: none.
  2. **Visual inspection only.** Read the file.
     Meaning: no reproducible proof.
     In practice: weaker.
     Gain: none.
     Cost: an unproven claim.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: whether chapters need updates.
- Context: the change restores a generated file to the existing contract.
- Question: any chapter affected?
- Options:
  1. **No chapter.** All 12 rows No with reasons.
     Meaning: the arc42 assessment is all-No.
     In practice: no durable text changes.
     Gain: truthful assessment.
     Cost: none.
  2. **Chapter 8.** Document the template contract.
     Meaning: the contract is already owned by the ADR qualification rules.
     In practice: duplication.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Risk | The scaffolder writes only missing chapter files, so re-enabling cannot overwrite existing chapters. |
| Testability | The project copy differs from the managed asset only by the missing structured verdict block. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | No use-case delta; the repair restores a generated project file | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | No user-visible surface; `none` recorded with the reason | accepted |
| Actor and Persona distinction | No Gherkin actors introduced | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| none | The repair restores a generated documentation file; no user-visible surface. | accepted |

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
- [x] `design.md` reflects the repair method.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change resync-adr-template` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
