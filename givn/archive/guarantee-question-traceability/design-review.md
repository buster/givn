# Design Review: guarantee-question-traceability

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Are the templates, commands, and review instruction asserted? | agreed — template, review, and widened command coverage |
| 2 | Tech choices | Does the ledger link back to the seed question ID? | agreed — `Decides D2 (seed Q5)` shape |
| 3 | Missing scenarios | Is the advisory carve-out asserted? | agreed — step added |
| 4 | Testability | Which files does each Given read? | agreed — instruction plus command |
| 5 | Risk | Which qualifier spelling is canonical? | agreed — `(open: Q<N>)` everywhere |
| 6 | Architecture documentation | Is the ADR action block recorded? | agreed — `ADR action: NONE` added |

### Q1 — Unasserted surfaces

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the templates, commands, and review instruction are guarded.
- Context: ten surfaces are edited; three scenarios covered only the ideation
  skill and two instructions.
- Question: add coverage or narrow the promise?
- Options:
  1. **Add the template and review scenarios and widen the Givens.**
     Meaning: eject-based template assertions, a review-instruction scenario,
     and instruction-plus-command reads.
     In practice: six unasserted surfaces become executable.
     Gain: the promised shapes cannot silently vanish.
     Cost: one scenario plus wider steps.
  2. **Narrow the proposal wording.**
     Meaning: templates and commands become advisory shapes.
     In practice: the promise weakens.
     Gain: smaller delta.
     Cost: unverifiable deliverables.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — Seed reference in the ledger

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether a `Decides D3` entry can be traced to seed question Q5.
- Context: the ledger IDs are proposal-local; seed questions use `Q<N>`.
- Question: link them?
- Options:
  1. **Require the seed reference beside the ledger ID.**
     Meaning: `Decides D2 (seed Q5): …`.
     In practice: the handover's "Q5 must be dispositioned" criterion is
     checkable from the proposal alone.
     Gain: traceability.
     Cost: one phrase.
  2. **No link.**
     Meaning: the reader cannot tell which seed question resolved.
     In practice: the criterion is uncheckable.
     Gain: none.
     Cost: lost traceability.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Advisory carve-out

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the hard requirement's scope is pinned.
- Context: the handover says non-ideation changes get the structure advisory.
- Question: assert it?
- Options:
  1. **Add the advisory Then.**
     Meaning: the guidance must mark the rule advisory without an ideation
     source.
     In practice: the carve-out cannot be dropped silently.
     Gain: the scoping is guarded.
     Cost: one step.
  2. **Leave it to review.**
     Meaning: the carve-out is implicit.
     In practice: a blanket blocking rule passes.
     Gain: none.
     Cost: an unguarded scope.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Given read scope

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the command edits are covered.
- Context: the design said "direct asset reads" without naming the files.
- Question: instruction only, or instruction plus command?
- Options:
  1. **Instruction plus command.**
     Meaning: each Given reads both surfaces.
     In practice: a missing command edit fails RED.
     Gain: every edited surface for that artifact is covered.
     Cost: none.
  2. **Instruction only.**
     Meaning: commands are unguarded.
     In practice: silent gaps.
     Gain: none.
     Cost: weaker.
- Resolution: **Option 1**, matching the existing review Given's
  instruction-plus-template read. Rejected: Option 2.

### Q5 — Canonical qualifier spelling

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the asserted literal matches the asset.
- Context: the handover writes `(open: Q<N>)`, `(open: Qn)`, and
  `(open: Q5)` in different places.
- Question: pin one spelling?
- Options:
  1. **Pin `(open: Q<N>)` everywhere.**
     Meaning: the marker matches the `## Q<N>` question heading.
     In practice: one literal across all surfaces and the assertion.
     Gain: deterministic.
     Cost: none.
  2. **Loosen the assertion.**
     Meaning: assert the `(open: Q` prefix.
     In practice: either casing passes.
     Gain: tolerance.
     Cost: a weaker contract.
- Resolution: **Option 1**. Rejected: Option 2.

### Q6 — ADR action block

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the assessment follows the impact template's own shape.
- Context: the non-qualification was recorded inline only.
- Question: add the block?
- Options:
  1. **Add `ADR action: NONE` and the target.**
     Meaning: two lines in arc42.md.
     In practice: the template's shape is honoured.
     Gain: consistency.
     Cost: two lines.
  2. **Accept the inline row.**
     Meaning: the block stays absent.
     In practice: inconsistent with the template.
     Gain: none.
     Cost: a shape deviation.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Scope | The handover's review-disposition-gates scenario is intentionally dropped: that file drives the runtime `givn check review` gate, and runtime enforcement is out of scope; the design routes the verification to the instruction-integrity guidance scenario. |
| Scope | Routing `NEW in fragments` is valid for both capabilities; `fragment.md` lists them and the deltas live under `specs/fragments/`. |
| Testability | The single-scenario command, strict mode, and stub are valid; the ideation scenario reuses existing steps and the renderer passes `(open: Q<N>)` verbatim. |
| E2E fidelity | No new `@e2e` scenario is needed; the design applies the normalized action scope and names real driving mechanisms. |
| Architecture documentation | The independent 12-row walk matches arc42.md; section 8.23, R-70, and the two glossary terms contain current content; chapter 9 is correctly untouched. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `ideation-skill` and `instruction-integrity` keep fragment ownership. | accepted |
| Typed Includes / Extends relationships | The support fragment is included by the use cases; no relationship changes. | accepted |
| Confirmed Persona coverage | `agent--technical-role` (guidance, templates) and `givn-maintainer--owner` (rule corpus) are confirmed permanent Personas. | accepted |
| Actor and Persona distinction | Gherkin keeps the author, maintainer, and reviewer as actors; Personas stay review lenses. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | Consumes the qualifier rule, the disposition rule, the ledger, and the templates. | The four scenarios |
| givn-maintainer--owner | Owns the rule corpus and the ledger contract. | The ledger scenario; chapter 8.23 |

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
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.23 |
| 9 | 09 architecture-decisions | No | No | Yes | Yes — no qualified candidate; `ADR action: NONE` with target 8.23 |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-70 |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — `Open-question qualifier`, `Decision ledger` |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; `ADR action: NONE` with target 8.23.
- [x] No MADR "Bad, because..." consequence requires a chapter 11
      counterpart (no new MADR).
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/guarantee-question-traceability/specs/fragments/instruction-integrity.feature` | Added the review-instruction and template scenarios; widened the disposition and ledger assertions. |
| `givn/changes/guarantee-question-traceability/design.md` | Seed reference, pinned literal, widened Givens, spec coverage update. |
| `givn/changes/guarantee-question-traceability/arc42.md` | `ADR action: NONE` block added. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change guarantee-question-traceability` exits 2 (WIP, no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
