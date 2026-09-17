# Design Review: adr-qualification-hardening

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Testability | Does the row-9 scenario fail RED against the current wording? | agreed — assert the normalizing sentence's absence |
| 2 | Missing scenarios | Is the design-template prompt asserted? | agreed — scenario added |
| 3 | Missing scenarios | Is the genuine-boundary pass case asserted? | agreed — step added |
| 4 | Missing scenarios | Is the no-numeric-cap statement asserted? | agreed — step added |
| 5 | Testability | Are the cheaper-home phrases pinned per source file? | agreed — per-file assertions |
| 6 | Testability | Which ADR template does the blank check read? | agreed — the shipped asset |
| 7 | Tech choices | Does the impact marker copy the verdict schema? | agreed — blank placeholders only, no value lists |
| 8 | Scope | Is the stale template description in the instruction updated? | agreed — updated in the same change |
| 9 | Testability | Is the blank-template phrase casing pinned? | agreed — "Fill every value from the structured verdict" |
| 10 | Risk | What guards against green-without-edit? | agreed — unique phrases plus recorded RED runs |
| 11 | Architecture documentation | Does 8.15 record the design-template prompt? | agreed — sentence added |

### Q1 — Row-9 RED proof

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the row-9 scenario can fail before the reword.
- Context: the asserted phrases already exist in the current instruction, so
  the scenario would go green without any edit.
- Question: which assertion makes it RED?
- Options:
  1. **Assert the normalizing sentence's absence.**
     Meaning: not-contains "a single change routinely triggers rows 3, 5, 6,
     9, and 11" plus the surviving double-check wording.
     In practice: the unedited instruction fails RED.
     Gain: the reword is mechanically required.
     Cost: one assertion.
  2. **Assert a new unique phrase.**
     Meaning: force a phrase only the new wording carries.
     In practice: the wording is constrained further.
     Gain: RED too.
     Cost: an artificial phrase.
- Resolution: **Option 1** because the absence check proves the reword without
  forcing extra wording. Rejected: Option 2 as unnecessary.

### Q2 — Design-template prompt assertion

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the design-template edit is guarded.
- Context: no scenario asserts `assets/templates/design.md`'s Technology
  Decisions section.
- Question: add the scenario?
- Options:
  1. **Add it.**
     Meaning: read the template, assert the two statements.
     In practice: deleting the prompt fails RED.
     Gain: the proposal bullet is covered.
     Cost: one scenario.
  2. **Leave it to review.**
     Meaning: untested prompt.
     In practice: silent deletion.
     Gain: none.
     Cost: an unverified bullet.
- Resolution: **Option 1** because five of six bullets already have coverage.
  Rejected: Option 2.

### Q3 — Genuine-boundary pass case

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether an over-broad reword that fails every candidate is caught.
- Context: the scenario asserted only failures and non-examples.
- Question: assert the pass case?
- Options:
  1. **Add "the guidance still qualifies a genuine boundary decision".**
     Meaning: assert a phrase unique to the new block.
     In practice: the guidance must state the pass case.
     Gain: over-broad tests are caught.
     Cost: one step.
  2. **Rely on the five-dimension scenarios.**
     Meaning: no falsification-block pass case.
     In practice: an over-broad block passes.
     Gain: none.
     Cost: an uncaught failure mode.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — No-numeric-cap assertion

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the kind-not-count guarantee is fully covered.
- Context: the scenario asserted "classify by kind rather than count" only.
- Question: assert the no-cap statement?
- Options:
  1. **Add "the guidance states there is no numeric ADR cap".**
     Meaning: assert "no numeric ADR cap".
     In practice: the central risk (tests read as a budget) is guarded.
     Gain: direct coverage.
     Cost: one step.
  2. **Treat kind-not-count as sufficient.**
     Meaning: no cap assertion.
     In practice: the guarantee is implicit.
     Gain: none.
     Cost: weaker coverage.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Per-source cheaper-home assertions

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the command and skill edits are enforced.
- Context: the scenario asserted against concatenated guidance sources, so one
  edited file turns it green.
- Question: pin per file?
- Options:
  1. **Assert the phrase in each of the three files.**
     Meaning: instruction, command, and skill template each checked.
     In practice: a missing source fails RED; a `givn skills sync` run carries
     the pointer into the generated copies.
     Gain: all three edits are enforced.
     Cost: two extra steps.
  2. **Keep the combined assertion.**
     Meaning: review catches a missing source.
     In practice: silent gaps.
     Gain: none.
     Cost: weaker enforcement.
- Resolution: **Option 1**. Rejected: Option 2.

### Q6 — Which ADR template the blank check reads

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: the fixture path for the blank-template Given.
- Context: the design did not name the path.
- Question: shipped asset or project-local copy?
- Options:
  1. **The shipped asset.**
     Meaning: `assets/addons/arc42-docs/chapters/adr/adr-template.md`.
     In practice: fails RED on the current prefill; the project-local copy
     stays prefilled per the no-migration decision.
     Gain: matches the scenario title "ships" and the other asset Givens.
     Cost: none.
  2. **The project-local copy.**
     Meaning: `docs/arc42/adr/adr-template.md`.
     In practice: contradicts the no-migration decision.
     Gain: none.
     Cost: scope creep.
- Resolution: **Option 1**. Rejected: Option 2.

### Q7 — Impact marker schema duplication

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the marker copies the verdict schema.
- Context: the design gave the marker the full structured verdict field set.
- Question: how much of the schema goes in?
- Options:
  1. **Blank placeholders and the placement rule only.**
     Meaning: no allowed-value lists, no AND formula, no evidence schema.
     In practice: the marker stays thin and cannot be copied as a prefilled
     pass.
     Gain: consistent with the thin-derived-source rule.
     Cost: none.
  2. **Copy the full verdict block.**
     Meaning: a second schema copy.
     In practice: duplication and drift risk.
     Gain: one-stop recording.
     Cost: contradicts the change's own rule.
- Resolution: **Option 1**. Rejected: Option 2.

### Q8 — Stale template description

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the canonical instruction still describes the template as
  embedding the passing marker.
- Context: the instruction's template description names
  `ADR-QUALIFICATION: PASS` as template content.
- Question: update it?
- Options:
  1. **Update the sentence.**
     Meaning: describe the blank qualification section filled from the
     recorded verdict.
     In practice: no self-contradiction.
     Gain: canonical source stays truthful.
     Cost: one sentence.
  2. **Leave it.**
     Meaning: the instruction contradicts the blank template.
     In practice: an author trusts the stale sentence.
     Gain: smaller diff.
     Cost: exactly the drift this change fights.
- Resolution: **Option 1**. Rejected: Option 2.

### Q9 — Blank-template phrase casing

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the asserted phrase matches the asset byte-for-byte.
- Context: exact-substring steps make the capital F load-bearing.
- Question: pin the phrase?
- Options:
  1. **Pin "Fill every value from the structured verdict" and use it
     verbatim.**
     Meaning: one exact phrase in both the spec table and the asset.
     In practice: a casing mismatch fails GREEN.
     Gain: deterministic.
     Cost: none.
  2. **Lowercase both.**
     Meaning: a weaker phrase.
     In practice: also works.
     Gain: none.
     Cost: none.
- Resolution: **Option 1** for consistency with the existing exact-substring
  steps. Rejected: Option 2.

### Q10 — Green-without-edit guard

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: how missing wording is caught mechanically.
- Context: several asserted phrases already exist elsewhere in the corpus.
- Question: what is the primary mitigation?
- Options:
  1. **Unique phrases plus recorded RED runs.**
     Meaning: each new step asserts at least one phrase only the new text
     carries, and each RED task records the failing run.
     In practice: the RED/GREEN loop proves the edit exists.
     Gain: mechanical proof.
     Cost: discipline.
  2. **Rely on review grilling.**
     Meaning: no mechanical guard.
     In practice: gaps surface late.
     Gain: none.
     Cost: weak.
- Resolution: **Option 1**. Rejected: Option 2.

### Q11 — Chapter 8.15 and the design-template prompt

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether chapter 8.15 records all six guidance elements.
- Context: 8.15 omitted the design-template prompt.
- Question: add it?
- Options:
  1. **Add one sentence.**
     Meaning: "Technology-decision justification belongs in `design.md` ...".
     In practice: chapter 8 matches design.md.
     Gain: conformance.
     Cost: one sentence.
  2. **Accept the omission.**
     Meaning: 8.15 covers the ADR gate only.
     In practice: the chapter is incomplete for this change.
     Gain: none.
     Cost: an arc42 finding.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| E2E fidelity | No new `@e2e` scenario is needed: the fragment declares no interactions, the two existing `@e2e` CLI scenarios cover the actions, and the new scenarios are content variants. |
| Testability | The blank-template assertion fails RED: the asset holds "ADR-QUALIFICATION: PASS" and "qualification: QUALIFIED" today; the surviving prose uses a backticked form that does not match the asserted string. |
| Testability | Strict mode, `GIVN_FEATURES`, and the single-scenario command are valid and match the archived pattern. |
| Testability | The permanent template-conditional-rendering assertions survive the ADR-branch edit; the derived-sources parity check forbids only "qualification = alternatives". |
| Scope | The project-local ADR template copy is byte-identical to the asset; leaving it prefilled is the recorded no-migration decision. |
| Scope | The delta spec is lint-clean apart from the intended `@wip` tags. |
| Architecture documentation | The independent 12-row walk matches arc42.md; chapter 9's "No" is correct under the new single-change-reversibility test; no new ADR is required. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `adr-qualification-gate` keeps fragment ownership. | accepted |
| Typed Includes / Extends relationships | The support fragment is included by the use cases; no relationship changes. | accepted |
| Confirmed Persona coverage | `agent--technical-role` (Arc42 and design-review guidance) and `givn-maintainer--owner` (rule corpus, ADR register) are confirmed permanent Personas. | accepted |
| Actor and Persona distinction | Gherkin keeps the author, maintainer, and reviewer as actors; Personas stay review lenses. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | Consumes the qualification gate, the verdict placement rule, and the design-review challenge. | The falsification, verdict-placement, and cheaper-home scenarios |
| givn-maintainer--owner | Owns the ADR corpus and the qualification rules. | The chapter-selection scenario; chapter 8.15 |

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
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.15 records all six guidance elements including the design-template prompt |
| 9 | 09 architecture-decisions | No | No | Yes | Yes — no qualified candidate; the change's own choices fail single-change reversibility |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-23 names the tests and the challenge |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — `Falsification tests`, `Cheaper-home challenge` |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; the canonical destination is chapter 8.15.
- [x] No MADR "Bad, because..." consequence requires a chapter 11
      counterpart (no new MADR).
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/adr-qualification-hardening/specs/fragments/adr-qualification-gate.feature` | Added the pass-case, no-cap, design-template, per-source cheaper-home, and row-9-absence steps; split the cheaper-home scenario per source file. |
| `givn/changes/adr-qualification-hardening/design.md` | Pinned phrases, thin impact marker, stale-sentence update, per-file cheaper-home assertions, RED uniqueness note. |
| `givn/changes/adr-qualification-hardening/arc42.md` | Unchanged rows; chapter 8.15 gained the design-template sentence. |
| `docs/arc42/08-crosscutting-concepts.md` | 8.15 records the design-template prompt. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change adr-qualification-hardening` exits 2 (WIP, no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
