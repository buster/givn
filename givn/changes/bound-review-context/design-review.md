# Design Review: bound-review-context

Review kind: self-review
Author: agent
Reviewer context: agent

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Which gate requires independence? | The proposal's recorded `Review kind: independent`; the outline identity check stays separate. |
| 2 | Missing scenarios | How do pre-policy changes keep passing? | A record with no `Review kind` line keeps the previous result; only a recorded kind is enforced. |
| 3 | Testability | Where does the new check run? | The review integrity hook and the archive static gates, both naming the offending artifact and value. |
| 4 | Interaction coverage | Is a new E2E scenario needed? | No; the existing `check review` E2E action already covers the real CLI boundary. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Scope | `src/integrity_hook.rs` already owns review-artifact checks and receives the change directory; `src/cli/archive.rs::run_static_review_gates` is the archive re-entry point. The new module fits both without touching the CLI surface. |
| Tech choices | `src/outline/gate/markers.rs` shows the established fenced-block-aware parser pattern; `review_context` reuses that shape and adds no dependency. |
| Testability | `tests/steps/workflow_enforcement_steps.rs` already owns the `check review` fixtures and the `scaffold_init_root_change` helper. No new step file is needed. |
| Risk | The outline gate's `OUTLINE: AUTHOR` / `OUTLINE: REVIEWER` identity check is independent of the review kind. Merging the two would change an existing gate; the design keeps them separate. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `verify-change` owns `givn-workflow-enforcement`; the new record extends it. | Accepted from the proposal's route. |
| Typed Includes / Extends relationships | The use case includes the support fragment and extends none; unchanged. | No action. |
| Confirmed Persona coverage | `agent--technical-role` and `developer--primary-user` are the workflow and CLI consumers. | Proposal records both. |
| Actor and Persona distinction | Gherkin actors stay `developer`; the personas are review lenses. | No collapse. |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| `agent--technical-role` | Runs the workflow steps and reads the review instruction. | `givn instructions review` and `givn check review` scenarios. |
| `developer--primary-user` | Receives the CLI review diagnostic and the change record. | Review-kind gate scenarios and the existing review-check E2E scenario. |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | No goal or stakeholder change. |
| 2 | 02 architecture-constraints | Yes | Yes | Yes | TC-44 matches the review-context contract. |
| 3 | 03 context-and-scope | No | No | Yes | The CLI check boundary is reused. |
| 4 | 04 solution-strategy | No | No | Yes | Existing module and asset cascade. |
| 5 | 05 building-block-view | Yes | Yes | Yes | `review_context` row matches the design. |
| 6 | 06 runtime-view | Yes | Yes | Yes | The check-review step includes the new record. |
| 7 | 07 deployment-view | No | No | Yes | No deployment change. |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Section 8.26b matches the policy. |
| 9 | 09 architecture-decisions | No | No | Yes | Workflow policy; no ADR qualification. |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Q-120 matches the gate scenarios. |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | R-124 states the attestation limit. |
| 12 | 12 glossary | Yes | Yes | Yes | Review terms match the design. |

- [x] All 12 rows assessed independently against proposal.md/design.md before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md saying "No" or omitting the row.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry in chapter 09; the change records `ADR action: NONE` with a canonical target.
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in chapter 11; no new ADR means no new consequence.
- [x] Every ADR candidate records the structured qualification verdict and its route-specific target; the verdict routes to `CANONICAL_ARTIFACT`.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `proposal.md` | Recorded `Review kind: self-review` and the confirmed personas. |
| `specs/verify-change/givn-workflow-enforcement.feature` | Removed the outline-requires-independence scenario; the proposal record is the requirement. |
| `arc42.md` and chapters | Added the constraint, component, runtime step, concept, quality scenario, risk, and glossary terms. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D1`) resolves in the design's `## Decisions` ledger.
- [x] All questions are resolved.
- [x] `design.md` reflects decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the removed outline scenario.
- [x] `givn lint --change bound-review-context` exits 0 or 2 (advisories only).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
