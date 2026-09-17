# Design Review: finding-severity-routing-signal

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Tech choices | Are all emitted finding kinds classified? | agreed — two kinds added, default advisory, enumeration test |
| 2 | Scope | Is the no-signal rule recorded in all three surfaces? | agreed — instruction, command, template, asserted |
| 3 | Missing scenarios | Is the signal confidence asserted? | agreed — "confidence: signal" and the no-signal reason added |
| 4 | Tech choices | Is the capability-name/use-case ownership term explicit? | agreed — term added; interaction weight exercised |
| 5 | Testability | Are the route fixtures and scores pinned? | agreed — arithmetic pinned; unit test first |
| 6 | E2E fidelity | Does the matrix cite a real inventory entry? | agreed — the spec-overlap-gates entry |
| 7 | Missing scenarios | Is the prose severity asserted for a finding? | agreed — `[WIP  ] finding`; e2e-intent line carries severity |
| 8 | Risk | Can ownership credit leak to the mismatched fixture? | agreed — pinned mismatch and tolerant loading |
| 9 | Scope | Does the modified scenario keep a contradicting title? | agreed — removed plus added under the corrected title |
| 10 | Architecture documentation | Is R-71 ordered? | agreed — moved after R-70 |

### Q1 — Complete severity classification

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether unlisted kinds fall through to an undefined severity.
- Context: `singleton_usecase` and `usecase_interaction_invalid` are emitted
  by the tree scan but absent from the table.
- Question: add them and define the default?
- Options:
  1. **Add both; default advisory; enumeration test.**
     Meaning: `singleton_usecase` and `usecase_interaction_invalid` are
     advisory; the default arm is advisory; a unit test lists every kind.
     In practice: a new kind cannot ship unclassified.
     Gain: total coverage.
     Cost: two rows and a test.
  2. **Leave them to the default.**
     Meaning: implicit advisory.
     In practice: a future blocking kind could silently become advisory.
     Gain: none.
     Cost: an unguarded classification.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — No-signal rule surfaces

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the template guidance drifts.
- Context: the design named the template and command; the scenario reads the
  instruction and command.
- Question: record in all three and assert the template?
- Options:
  1. **All three plus an eject assertion.**
     Meaning: instruction, command, and template state the rule; the ejected
     template is checked.
     In practice: no surface can silently drop it.
     Gain: complete coverage.
     Cost: one step.
  2. **Two surfaces.**
     Meaning: the template is unasserted.
     In practice: drift.
     Gain: none.
     Cost: a gap.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Signal confidence and no-signal reason

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the confidence contract is observable.
- Context: only the tie case asserted "no signal"; the signal suffix and the
  reason were unasserted.
- Question: add the assertions?
- Options:
  1. **Add "confidence: signal" and the reason.**
     Meaning: the goal-aligned scenario asserts the suffix; the tie scenario
     asserts "tie".
     In practice: both halves of the contract are pinned.
     Gain: no decorative strings.
     Cost: two assertions.
  2. **Leave them.**
     Meaning: the suffix could vanish.
     In practice: silent.
     Gain: none.
     Cost: weaker.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Capability-name ownership term

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the handover's "capability name and owning use case" weight
  exists.
- Context: the capability's scenario text does not include its own name.
- Question: add the term?
- Options:
  1. **Add an explicit +2.0 term and exercise the interaction weight.**
     Meaning: name or use-case id matches add `+2.0`; the aligned fixture
     exercises the interaction inventory.
     In practice: the requirement is verified.
     Gain: handover compliance.
     Cost: one term.
  2. **Rely on the scenario text.**
     Meaning: the requirement is unimplemented.
     In practice: a gap.
     Gain: none.
     Cost: an unmet requirement.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Fixture scores pinned

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the route scenarios are achievable.
- Context: the tie and aligned fixtures need exact scores.
- Question: pin the text and arithmetic?
- Options:
  1. **Pin both and add a unit test first.**
     Meaning: the design records the arithmetic; a unit test asserts the
     ranking before the Gherkin loop.
     In practice: a calibration miss fails fast.
     Gain: deterministic fixtures.
     Cost: one unit test.
  2. **Discover during implementation.**
     Meaning: the acceptance loop is the first check.
     In practice: slow failures.
     Gain: none.
     Cost: guesswork.
- Resolution: **Option 1**. Rejected: Option 2.

### Q6 — Interaction matrix entry

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the matrix invents a lint-command inventory row.
- Context: `verify-change/usecase.md` has no lint-command row; adding one
  would trip the interaction gate.
- Question: cite the existing entry?
- Options:
  1. **Cite the spec-overlap-gates entry.**
     Meaning: the severity scenarios are variants of the proven action.
     In practice: no new inventory row, no new `@e2e`.
     Gain: gate-consistent.
     Cost: none.
  2. **Add a lint-command row.**
     Meaning: the gate reports an interaction mismatch.
     In practice: a blocking finding.
     Gain: none.
     Cost: a broken gate.
- Resolution: **Option 1**. Rejected: Option 2.

### Q7 — Prose severity for a finding

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the prose rendering is asserted beyond advisory.
- Context: only the JSON field and the advisory prose were asserted.
- Question: add the WIP prose assertion and the e2e-intent severity?
- Options:
  1. **Add both.**
     Meaning: `[WIP  ] finding` is asserted; the e2e-intent line carries its
     severity.
     In practice: prose rendering is covered for two levels.
     Gain: no silent drop.
     Cost: one assertion.
  2. **JSON only.**
     Meaning: prose severity is unguarded.
     In practice: silent.
     Gain: none.
     Cost: a gap.
- Resolution: **Option 1**. Rejected: Option 2.

### Q8 — Ownership-credit leakage

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the mismatched fixture can earn ownership credit.
- Context: the affinity scorer reads the whole `usecase.md`.
- Question: constrain and document?
- Options:
  1. **Pin the mismatch and require tolerant loading.**
     Meaning: the mismatched goal shares no proposal word; missing
     `usecase.md` contributes nothing and never errors.
     In practice: existing route unit tests keep passing.
     Gain: deterministic fixtures.
     Cost: none.
  2. **Discover during implementation.**
     Meaning: flaky ranking.
     In practice: slow debugging.
     Gain: none.
     Cost: risk.
- Resolution: **Option 1**. Rejected: Option 2.

### Q9 — Modified scenario title

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the permanent spec describes the opposite of its body.
- Context: the modified scenario kept "recommends NEW" while asserting no
  signal.
- Question: rename?
- Options:
  1. **Remove the old title and add the corrected one.**
     Meaning: `@givn.removed` plus `@givn.added`.
     In practice: the permanent title matches the behavior.
     Gain: truthful spec.
     Cost: two scenarios.
  2. **Keep the title.**
     Meaning: contradiction persists.
     In practice: confusing.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

### Q10 — R-71 ordering

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: register hygiene.
- Context: R-71 sat before R-70.
- Question: move it?
- Options:
  1. **Move R-71 after R-70.**
     Meaning: numeric order restored.
     In practice: consistent register.
     Gain: hygiene.
     Cost: none.
  2. **Leave it.**
     Meaning: out of order.
     In practice: cosmetic drift.
     Gain: none.
     Cost: hygiene.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Testability | The JSON assertion must use the `stdout mentions` step, which unescapes Gherkin quotes; `stdout contains` does not. Corrected in the delta. |
| Testability | The single-scenario command, strict mode, and stub are valid; the lint fixture scores 4.9 as a unique top, so the permanent route scenarios stay green. |
| Testability | The no-signal expectation for the internal-widget proposal is reachable (affinity 0.0). |
| Tech choices | The scatter penalty cannot zero a candidate (minimum 0.1); it only orders. |
| Scope | `NEW in fragments` is the only accepted decision for fragment capabilities. |
| Architecture documentation | ADR action NONE is correct; ADR-027 owns the exit-code contract and preserves it. |
| Architecture documentation | The design's "additive for JSON consumers" claim is removed: route has no JSON flag. |
| Missing scenarios | The empty-scope JSON branch also needs the severity field; recorded in the design. |
| Tech choices | The `interaction_mismatch` prose label stays `[WARN ]`; only the severity word is added. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `lint-command` and `spec-explore-change` extend their owners; `instruction-integrity` keeps fragment ownership. | accepted |
| Typed Includes / Extends relationships | The support fragment is included by the use cases; no relationship changes. | accepted |
| Confirmed Persona coverage | `agent--technical-role` (CLI output) and `givn-maintainer--owner` (tooling) are confirmed permanent Personas. | accepted |
| Actor and Persona distinction | Gherkin keeps the user and the agent as actors; Personas stay review lenses. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | Consumes the lint severity output and the route report. | The three lint scenarios and the three route scenarios |
| givn-maintainer--owner | Owns the tooling and the severity registry. | Same scenarios; chapters 6.4 and 6.17 |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | — |
| 2 | 02 architecture-constraints | No | No | Yes | — |
| 3 | 03 context-and-scope | No | No | Yes | — |
| 4 | 04 solution-strategy | No | No | Yes | — |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes — `cli::lint` and `spec::route` rows |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — 6.4 and 6.17 |
| 7 | 07 deployment-view | No | No | Yes | — |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.24 |
| 9 | 09 architecture-decisions | No | No | Yes | Yes — `ADR action: NONE`, target 6.17 |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-71 (moved after R-70) |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — `Finding severity`, `Route confidence` |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; `ADR action: NONE` with target 6.17.
- [x] No MADR "Bad, because..." consequence requires a chapter 11
      counterpart (no new MADR).
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/finding-severity-routing-signal/specs/verify-change/lint-command.feature` | JSON step corrected to `stdout mentions`; WIP prose assertion added. |
| `givn/changes/finding-severity-routing-signal/specs/fragments/spec-explore-change.feature` | Signal confidence, no-signal reason, and the removed/added rename. |
| `givn/changes/finding-severity-routing-signal/specs/fragments/instruction-integrity.feature` | Template eject assertion added. |
| `givn/changes/finding-severity-routing-signal/design.md` | Two kinds added, default advisory, empty-scope JSON, e2e-intent severity, ownership term, three surfaces, fixture arithmetic, JSON claim removed, leakage risk. |
| `docs/arc42/11-risks-and-technical-debt.md` | R-71 moved after R-70. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change finding-severity-routing-signal` exits 2 (WIP, no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
