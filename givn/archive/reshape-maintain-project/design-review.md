# Design Review: reshape-maintain-project

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Operator decision | Approve the three-child split and names? | agreed — `keep-documentation-current`, `upgrade-a-project`, `publish-a-release` |
| 2 | Model | Where does the retired-command notice live? | agreed — under `apply-a-managed-upgrade` |
| 3 | Model | Is the summary conjunction allowed? | agreed — the `+` coverage is the union |
| 4 | Discipline | Do the new narratives pass R1, R2, R5, R6, R7, R9, R10? | agreed — verified criterion by criterion |
| 5 | Architecture documentation | Any chapter affected? | no — all 12 rows No |

### Q1 — Split and names

- Status: closed 2026-09-18 (operator)
- Blocks: the final structure of the maintain-project family.
- Context: `release-0-3-0` bundles upgrade, release metadata, and a retired
  command notice.
- Question: approve the three-child split?
- Options:
  1. **Three children.** `keep-documentation-current` (`readme-enforcement`),
     `upgrade-a-project` (`apply-a-managed-upgrade`), `publish-a-release`
     (`describe-the-release`).
     Meaning: each behavior sits under a goal-shaped capability.
     In practice: one feature moves, one splits.
     Gain: model-conformant.
     Cost: two new use cases.
  2. **Two children.** Fold the release metadata into
     `apply-a-managed-upgrade`.
     Meaning: the capability still mixes publication with upgrade.
     In practice: no new release use case.
     Gain: fewer names.
     Cost: a mixed capability.
  3. **Adjust names.** Give corrections.
     Meaning: same structure, different labels.
     In practice: reference churn.
     Gain: preferred wording.
     Cost: one more round.
- Resolution: **Option 1** (operator-approved 2026-09-18). Rejected:
  Options 2 and 3.

### Q2 — Retired-command notice

- Status: closed 2026-09-18 (operator)
- Blocks: where the scenario lives.
- Context: the notice tells a consumer a removed command is unavailable.
- Question: `upgrade-a-project` or `publish-a-release`?
- Options:
  1. **`upgrade-a-project`.** The notice is part of crossing a release gap:
     the consumer meets the removal after upgrading.
     Meaning: one capability owns the upgrade lifecycle.
     In practice: the scenario stays with the upgrade scenarios.
     Gain: coherent.
     Cost: none.
  2. **`publish-a-release`.** The release removed the command.
     Meaning: the notice sits with release metadata.
     In practice: an e2e row under a no-e2e capability.
     Gain: release-centric.
     Cost: weaker fit.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Summary conjunction

- Status: closed 2026-09-18 (model rule)
- Blocks: whether the summary's union guarantee is allowed.
- Context: R1 fails leaves with conjunction guarantees.
- Question: is the union allowed on the `+`?
- Options:
  1. **Yes.** The `+` coverage is the union of its children.
     Meaning: the summary states the union.
     In practice: no direct capabilities.
     Gain: model-conformant.
     Cost: none.
  2. **No.** Keep the summary single-outcome.
     Meaning: the split is incomplete.
     In practice: R1 remains.
     Gain: none.
     Cost: a violation.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Discipline

- Status: closed 2026-09-18 (review criteria)
- Blocks: whether the new narratives pass.
- Context: R1, R2, R5, R6, R7, R9, R10 apply.
- Question: what was verified?
- Options:
  1. **Criterion by criterion.** Single goals; primary actors named;
     step-anchored extensions; preconditions before triggers;
     mechanism-free Goal, Trigger, Main flow, and Guarantees; flows under
     three goals; valid Includes.
     Meaning: conformance.
     In practice: verified in this review.
     Gain: model-conformant.
     Cost: none.
  2. **Spot-check.** Trust the rewrite.
     Meaning: possible gaps.
     In practice: none.
     Gain: faster.
     Cost: risk.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Architecture documentation

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
  2. **Chapter 7 Yes.** The release metadata moved.
     Meaning: no architecture fact changed.
     In practice: none.
     Gain: none.
     Cost: noise.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Model | `release-0-3-0.feature` holds sixteen scenarios: five metadata, ten upgrade, one retired command. |
| Model | The tree parser accepts zero-capability summaries and one-capability leaves. |
| Consistency | The retired-command scenario asserts the CLI surface, which the upgrade child owns. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | One leaf promoted; three children; two capabilities created from the release container | operator-approved |
| Typed Includes / Extends relationships | The summary includes the children; each leaf includes `fragment: support` | accepted |
| Confirmed Persona coverage | The maintainer family lists the two relevant Personas | accepted |
| Actor and Persona distinction | Personas remain review lenses | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Maintains projects and crosses release gaps | narratives |
| givn-maintainer--owner | Publishes releases and owns the upgrade contract | narratives |

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
- [x] `design.md` reflects the approved structure.
- [x] `specs` is intentionally skipped; no scenario delta exists.
- [x] `givn lint --change reshape-maintain-project` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
