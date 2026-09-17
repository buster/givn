# Design Review: use-case-model-scoping

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Is the test-only template edit the right surface? | agreed — keep it; record fixture-only reach |
| 2 | Missing scenarios | Is the sixth rejection criterion asserted? | agreed — step added |
| 3 | Missing scenarios | Are both worked examples and their criteria tie-ins asserted? | agreed — scenario extended |
| 4 | Testability | Are the fixture placeholders guarded? | agreed — placeholder assertion added |
| 5 | Missing scenarios | Is the design-review branch reference asserted? | agreed — scenario added |
| 6 | Missing scenarios | Are the workflow and reference pointers asserted? | agreed — steps added |
| 7 | Testability | Are the coarse Then steps pinned to new content? | agreed — "System responsibility" and all ten titles |
| 8 | Missing scenarios | Does the lint-unchanged criterion need a scenario? | agreed — no; recorded in design |
| 9 | Tech choices | Is guidance inside contract fields acceptable? | agreed — trade-off recorded |
| 10 | Risk | What is the most likely implementation failure? | agreed — phrase drift, with verbatim-copy mitigation |

### Q1 — Test-only template surface

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether editing `usecase_md_template()` delivers guidance to a real
  author.
- Context: no command emits the template today; it is the fixture and
  archive-test source. The handover nevertheless names the template edit.
- Question: keep the edit, add a real surface, or drop it?
- Options:
  1. **Keep it; record the reach.**
     Meaning: the edit stays and design.md states that author-visible guidance
     is the generated skill, the specs instruction, and the durable document.
     In practice: an author reads the model document; the template keeps the
     skeleton aligned.
     Gain: handover scope preserved; no new CLI action.
     Cost: the skeleton is not command-emitted today.
  2. **Add `givn eject usecase:template`.**
     Meaning: a real CLI surface for the guided skeleton.
     In practice: a new action, inventory row, and `@e2e` scenario.
     Gain: the author can eject the skeleton.
     Cost: a new CLI action beyond the decided scope.
- Resolution: **Option 1** because the handover keeps CLI surfaces out and the
  author-visible delivery is already covered. Rejected: Option 2 because it
  expands scope.

### Q2 — Sixth rejection criterion assertion

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether R6 has executable cover.
- Context: the scenario carried nine steps; the design promised ten criteria.
- Question: add the step or drop the phrase?
- Options:
  1. **Add the step.**
     Meaning: assert "Inconsistent preconditions" in the review instruction.
     In practice: a review instruction missing R6 fails the scenario.
     Gain: all ten criteria are guarded.
     Cost: one step.
  2. **Keep nine.**
     Meaning: R6 is covered only by the document scenario's count.
     In practice: the criterion can be dropped silently.
     Gain: none.
     Cost: an unverified promise.
- Resolution: **Option 1** because the proposal promises ten criteria tied to
  observable sections. Rejected: Option 2 because it leaves R6 unverified.

### Q3 — Worked examples and criteria tie-ins

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the acceptance criteria about the shaped cases are
  executable.
- Context: the acceptance criteria say an observing mega-leaf fails R4/R8 and
  a conjunction-goal leaf fails R1/R9, but only one worked split was asserted.
- Question: extend the scenario?
- Options:
  1. **Assert both worked examples and their tie-ins.**
     Meaning: the promotion example, the observing tie-ins, and the
     conjunction tie-ins become Then steps.
     In practice: the document must show the split, the promotion, and the
     criteria mapping.
     Gain: both acceptance criteria have mechanical proof.
     Cost: three steps.
  2. **Accept one example.**
     Meaning: the promotion example stays unasserted.
     In practice: the acceptance criterion has no check.
     Gain: smaller scenario.
     Cost: an unproven acceptance criterion.
- Resolution: **Option 1** because the handover names both shaped cases.
  Rejected: Option 2 because it leaves an acceptance criterion unverified.

### Q4 — Fixture placeholder guard

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether a placeholder edit silently breaks fixture documents.
- Context: fixture writers replace exact placeholder substrings; nothing
  guards them, and the parser only requires non-empty sections.
- Question: assert placeholder preservation?
- Options:
  1. **Assert all six placeholders survive.**
     Meaning: one Then checks each substring in the template.
     In practice: folding guidance into a placeholder fails the scenario.
     Gain: the silent fixture breakage is caught.
     Cost: one step.
  2. **Rely on the parse test.**
     Meaning: the existing test parses the raw template.
     In practice: a broken replacement is invisible.
     Gain: none.
     Cost: silent fixture drift.
- Resolution: **Option 1** because it is the only mechanical guard. Rejected:
  Option 2 because parsing does not exercise replacement.

### Q5 — Design-review branch reference

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the design-review edit is guarded.
- Context: the proposal promises the criteria in the design-review use-case
  branch; no scenario reads that instruction.
- Question: add the scenario or drop the edit?
- Options:
  1. **Add the scenario.**
     Meaning: a direct read of the design-review instruction asserting the
     criteria pointer.
     In practice: dropping the pointer fails the scenario.
     Gain: the promised branch edit is guarded.
     Cost: one scenario.
  2. **Drop the edit.**
     Meaning: criteria live only in the review instruction.
     In practice: the proposal wording narrows.
     Gain: one less edit.
     Cost: the design-review branch loses the pointer.
- Resolution: **Option 1** because the pointer is one sentence and the
  proposal names it. Rejected: Option 2 because it narrows a promised surface.

### Q6 — Workflow and reference pointers

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the two document references are guarded.
- Context: the handover lists the references as an acceptance criterion; no
  step read either document.
- Question: assert them?
- Options:
  1. **Add both steps.**
     Meaning: direct reads of `docs/workflow.md` and `docs/reference.md`
     asserting `use-case-model.md`.
     In practice: a dropped reference fails.
     Gain: the acceptance criterion is executable.
     Cost: two steps.
  2. **Leave them to review.**
     Meaning: references are unasserted.
     In practice: a dropped pointer surfaces only at human review.
     Gain: none.
     Cost: an unverified criterion.
- Resolution: **Option 1** because the change already reads project documents
  for the model itself. Rejected: Option 2 because it leaves a listed
  criterion unverified.

### Q7 — Coarse Then steps

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether two steps can pass on unchanged content.
- Context: "the template numbers the main-flow example" could pass on the
  current `1. <first step>` line, and the criteria step planned to assert only
  "R1" and "R10".
- Question: pin exact phrases?
- Options:
  1. **Pin "System responsibility" and all ten titles.**
     Meaning: the main-flow step asserts a phrase only the new pairing
     produces; the criteria step asserts every title.
     In practice: an unchanged template fails the step.
     Gain: RED is genuine.
     Cost: longer assertions.
  2. **Leave as designed.**
     Meaning: the template could ship unchanged.
     In practice: the scenario passes without the new content.
     Gain: none.
     Cost: a false green.
- Resolution: **Option 1** because a Then must assert new content. Rejected:
  Option 2 because it permits a no-op pass.

### Q8 — Lint-unchanged criterion

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether "no new parser warning" needs a scenario.
- Context: the only production change is a string constant with no parser or
  lint code path.
- Question: add a scenario?
- Options:
  1. **No; record it in design.**
     Meaning: the existing lint suite is the evidence.
     In practice: design.md states the boundary.
     Gain: no scenario testing no code path.
     Cost: none.
  2. **Add a negative scenario.**
     Meaning: assert no oversize or unreferenced-extension finding appears.
     In practice: a scenario guards wording that does not exist.
     Gain: an explicit guard.
     Cost: maintenance for no code path.
- Resolution: **Option 1** because a negative assertion over unchanged code
  adds no signal. Rejected: Option 2 as unnecessary.

### Q9 — Guidance inside contract fields

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether instruction prose may live in Actors/Extensions/Goal.
- Context: the parser accepts guidance only inside its fixed shapes.
- Question: accept, change the parser, or restrict to free-text sections?
- Options:
  1. **Accept; record the trade-off.**
     Meaning: the fields are slots an author replaces; no lint or tree path
     consumes the extensions list.
     In practice: fixture documents carry slot text in contract fields.
     Gain: no parser change.
     Cost: a documented trade-off.
  2. **Change the parser.**
     Meaning: a dedicated guidance section or comment tolerance.
     In practice: a parser change with its own scenario.
     Gain: clean fields.
     Cost: contradicts the decided scope.
- Resolution: **Option 1** because the parser stays untouched and the fields
  are templates. Rejected: Option 2 because it expands the change.

### Q10 — Most likely implementation failure

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: what the design's risk section should lead with.
- Context: the same sentences are hand-written into three artifacts and
  matched as exact substrings.
- Question: which risk leads?
- Options:
  1. **Phrase drift; mitigation: verbatim copy plus per-artifact delta runs.**
     Meaning: the definition, rule titles, criterion titles, and authoring
     phrases must match exactly.
     In practice: one typo leaves the scenario RED or breaks a sibling.
     Gain: the real failure mode is named.
     Cost: none.
  2. **Fixture placeholder breakage.**
     Meaning: guard the replacements.
     In practice: covered by Q4's assertion.
     Gain: none new.
     Cost: misses the dominant risk.
- Resolution: **Option 1**; placeholder breakage is kept as a secondary risk
  with its Q4 mitigation. Rejected: Option 2 as the lead because Q4 already
  guards it mechanically.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| E2E fidelity | No new `@e2e` scenario is needed: no new consumer action exists, and the three matrix rows map existing `@e2e` scenarios to their real CLI interfaces; the fragments declare `Interactions: none`. |
| Testability | Strict mode, `GIVN_FEATURES`, and the single-scenario command are valid; every new scenario is genuinely RED against the current tree (no model document, absent phrases). |
| Testability | The planned template edits do not collide with `ensure_fixture_usecase`, which rewrites only the Interactions table; placeholder preservation is asserted (Q4). |
| Architecture documentation | The independent 12-row walk matches arc42.md; 8.21, R-69, and the glossary rows contain real content; no ADR qualifies; no placeholder chapters. |
| Scope | `skill-rewrites.feature` asserts none of the rewritten phrases, so no update is needed there. |
| Scope | A pre-existing orphaned sentence in `assets/skills/givn-spec/SKILL.md.tmpl` ("rerun review; do not chunk or edit scenarios automatically.") is unrelated to this change; leaving it is scope-correct. |
| Missing scenarios | The permanent ideation scenarios assert the section headings ("Use Case Definition", "Use Case Elaboration", "Handoff") and the seed/term/dialog phrases; the rewrite must keep those headings and phrases. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `canonical-usecase-contract` extends the make-usecases-canonical use case; `ideation-skill` and `instruction-integrity` keep fragment ownership. | accepted |
| Typed Includes / Extends relationships | The support fragment is included by the use cases; no relationship changes. | accepted |
| Confirmed Persona coverage | `agent--technical-role` (generated guidance, template) and `givn-maintainer--owner` (model document, criteria) are confirmed permanent Personas. | accepted |
| Actor and Persona distinction | Gherkin keeps the maintainer, reviewer, author, and agent as actors; Personas stay review lenses. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | Consumes the generated ideation guidance, the specs placement test, and the template. | The ideation guidance scenarios; the specs guidance scenario |
| givn-maintainer--owner | Owns the durable model document, the rules, and the criteria. | The durable use-case model scenario; chapter 8.21 |

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
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.21 use-case model discipline |
| 9 | 09 architecture-decisions | No | No | Yes | Yes — no qualified candidate; destination 8.21 |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-69 |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — `Use case`, `Primary actor`, `Summary use case` |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; the canonical destination is chapter 8.21.
- [x] No MADR "Bad, because..." consequence requires a chapter 11
      counterpart (no new MADR).
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/use-case-model-scoping/specs/make-usecases-canonical/canonical-usecase-contract.feature` | Added the promotion example, both criteria tie-ins, the two document-reference steps, and the placeholder assertion. |
| `givn/changes/use-case-model-scoping/specs/fragments/instruction-integrity.feature` | Added the sixth criterion step and the design-review branch scenario. |
| `givn/changes/use-case-model-scoping/design.md` | Fixture-only reach note, trade-off note, pinned phrases, lint-criterion note, phrase-drift risk. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change use-case-model-scoping` exits 2 (WIP, no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
