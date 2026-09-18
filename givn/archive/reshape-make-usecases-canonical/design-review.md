# Design Review: reshape-make-usecases-canonical

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Discipline | Does the anchored extension pass R5? | agreed — step, condition, handling, and guarantee |
| 2 | Scope | Is the one-capability advisory in scope? | no — acceptable for maintainer tooling |
| 3 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Extension

- Status: closed 2026-09-18 (model rule)
- Blocks: whether R5 holds.
- Context: the extension said only "Invalid narratives are reported with
  their source path."
- Question: anchor it or remove it?
- Options:
  1. **Anchor it.** "1: a narrative is invalid → the report names its source
     path; protects the minimal guarantee."
     Meaning: R5 holds; the behavior is documented.
     In practice: one bullet.
     Gain: conformance.
     Cost: none.
  2. **Remove it.** The behavior is in the spec.
     Meaning: the document loses the failure path.
     In practice: none.
     Gain: shorter.
     Cost: lost narrative.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — Advisory

- Status: closed 2026-09-18 (operator)
- Blocks: whether the one-capability warning is a finding.
- Context: `givn lint` reports `[WARN ]` for use cases with one capability.
- Question: split the capability?
- Options:
  1. **Accept the advisory.** The capability is one coherent contract.
     Meaning: the warning stays advisory.
     In practice: none.
     Gain: no artificial split.
     Cost: none.
  2. **Split the capability.** Remove the warning.
     Meaning: two behaviors where one exists.
     In practice: reference churn.
     Gain: a clean lint.
     Cost: artificial.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: the arc42 assessment.
- Context: one narrative edit.
- Question: any Yes row?
- Options:
  1. **All No.** Truthful.
     Meaning: no chapter edits.
     In practice: none.
     Gain: none.
     Cost: none.
  2. **Chapter 3 Yes.** Restate the use-case tree.
     Meaning: unchanged.
     In practice: none.
     Gain: none.
     Cost: noise.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Discipline | The use-case parser accepts the anchored bullet shape. |
| Consistency | No durable artifact changes. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | One capability, one owner | accepted |
| Typed Includes / Extends relationships | No relationships | accepted |
| Confirmed Persona coverage | `givn-maintainer--owner` reviews the surface | accepted |
| Actor and Persona distinction | No Persona change | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| givn-maintainer--owner | Inspects the use-case tree | narrative |

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
- [x] No qualified ADR candidate.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the edit.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change reshape-make-usecases-canonical` has no parse
      errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
