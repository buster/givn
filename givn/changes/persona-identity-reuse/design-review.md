# Design Review: persona-identity-reuse

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Missing scenarios | Are the topic-persona home and the `--change` scope asserted? | agreed — two scenarios added |
| 2 | Missing scenarios | Is the persona template asset asserted? | agreed — scenario added |
| 3 | Missing scenarios | Are the preserved behaviors fully asserted? | agreed — assertions added |
| 4 | Missing scenarios | Is the disjunctive collapse example pinned? | agreed — phrase asserted |
| 5 | Missing scenarios | Are the command guardrails asserted? | agreed — command assertions added |
| 6 | Scope | Does a reasoned none with a visual interface flip to failing? | agreed — intended; scenario added, design corrected |
| 7 | Tech choices | What does a persona reference match? | agreed — ID-only; legacy advisory noise accepted |
| 8 | Risk | Which fixture creates the persona file? | agreed — the shared design-review Given |
| 9 | Architecture documentation | Is an ADR-028 amendment required? | agreed — fourth amendment recorded |

### Q1 — Topic-persona home and `--change` scope

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the topic resolution home and the change scope are proven.
- Context: the design named both; the spec asserted only a change document in
  the default scope.
- Question: add scenarios or drop the homes?
- Options:
  1. **Add both scenarios.**
     Meaning: a topic persona file resolves a reference; `--change` still
     reports a dangling one.
     In practice: two more scenarios with the existing fixture pattern.
     Gain: both design claims are executable.
     Cost: two scenarios.
  2. **Drop the homes.**
     Meaning: only the project persona home is scanned.
     In practice: forward references during a topic would warn.
     Gain: smaller scan.
     Cost: breaks the active-topic posture.
- Resolution: **Option 1**. Rejected: Option 2 because it contradicts the
  warning-only posture.

### Q2 — Persona template assertion

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the new template can be omitted silently.
- Context: no scenario read `assets/templates/persona.md`.
- Question: assert it?
- Options:
  1. **Add the template scenario.**
     Meaning: direct read; assert the stable core, the record, and the
     appendix.
     In practice: omitting the asset fails RED.
     Gain: the deliverable is guarded.
     Cost: one scenario.
  2. **Leave it to review.**
     Meaning: untested asset.
     In practice: silent omission.
     Gain: none.
     Cost: a missing deliverable.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Preserved behaviors

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the rewrite can silently drop confirmation, arbitration, or
  the actor distinction.
- Context: the scenario asserted the cap and no-auto-promotion only.
- Question: assert the missing behaviors?
- Options:
  1. **Add the assertions.**
     Meaning: "confirms every persona", "arbitrates", the command's
     "separate from Actors".
     In practice: a dropped behavior fails RED.
     Gain: the preserved set is complete.
     Cost: three assertions.
  2. **Keep the current set.**
     Meaning: confirmation and arbitration unguarded.
     In practice: silent regressions.
     Gain: none.
     Cost: weaker.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Disjunctive collapse example

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether an all-three conjunction reading is caught.
- Context: the criteria words appear, but nothing pins the disjunctive rule
  or the collapse example.
- Question: pin the collapse phrase?
- Options:
  1. **Assert "one persona with three stance blocks".**
     Meaning: the guidance must carry the collapse example.
     In practice: a conjunction reading fails RED.
     Gain: the acceptance criterion is verifiable.
     Cost: one assertion.
  2. **Rely on the criterion words.**
     Meaning: the disjunction is implicit.
     In practice: a conjunction rewrite passes.
     Gain: none.
     Cost: an unverifiable criterion.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Command guardrails

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the ideation command edit is guarded.
- Context: the design edits the command; no scenario read it.
- Question: assert the command?
- Options:
  1. **Add the command assertions.**
     Meaning: the generated command contains "inventory", "match", and
     "separate from Actors".
     In practice: a missing edit fails RED.
     Gain: the loaded surface is guarded.
     Cost: three assertions.
  2. **Leave it to review.**
     Meaning: untested command.
     In practice: silent gaps.
     Gain: none.
     Cost: weaker.
- Resolution: **Option 1**. Rejected: Option 2.

### Q6 — Reasoned none with a visual interface

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the behavior flip is intended and pinned.
- Context: today `PERSONAS: none — operator confirmed` passes for a visual
  interface (only the exact `none` is rejected); after ID resolution it
  fails. The design misstated today's behavior.
- Question: intended?
- Options:
  1. **Intended; pin it.**
     Meaning: a visual interface is a user-visible surface, so a reasoned
     none belongs only to a change without one; add a failing scenario and
     correct the design.
     In practice: the flip is documented and asserted.
     Gain: coherent contract.
     Cost: one scenario.
  2. **Preserve the old behavior.**
     Meaning: a reasoned none would pass for a visual interface.
     In practice: a visual change could bypass the persona requirement.
     Gain: no flip.
     Cost: an incoherent gate.
- Resolution: **Option 1**. Rejected: Option 2.

### Q7 — Reference matching rule

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether titles or IDs resolve.
- Context: givn's own ideation use cases name personas by title, so ID-only
  matching warns on this repository's own corpus.
- Question: ID-only or title matching?
- Options:
  1. **ID-only; accept the legacy noise.**
     Meaning: the entry must equal a persona file's name stem; about twenty
     advisories appear on givn's own corpus; exit code unchanged.
     In practice: the ID contract stays strict; migrating legacy labels is
     out of scope.
     Gain: a simple, strict rule.
     Cost: advisory noise.
  2. **Title matching too.**
     Meaning: also accept the persona file's heading.
     In practice: the scan reads every persona file.
     Gain: no noise.
     Cost: a weaker ID contract.
- Resolution: **Option 1** because the reuse protocol says "match by ID".
  Rejected: Option 2.

### Q8 — Fixture persona file

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the permanent passing visual scenarios stay green.
- Context: the fixture review.md names `fixture-persona`; no persona file
  exists, so ID resolution would break four permanent scenarios.
- Question: which fixture creates the file?
- Options:
  1. **The shared design-review Given.**
     Meaning: every visual fixture project carries
     `givn/personas/fixture-persona.md`.
     In practice: permanent and delta scenarios share the file.
     Gain: one edit covers all.
     Cost: none.
  2. **Each scenario's own Given.**
     Meaning: per-scenario setup.
     In practice: easy to miss one.
     Gain: locality.
     Cost: repetition.
- Resolution: **Option 1**. Rejected: Option 2.

### Q9 — ADR-028 amendment

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the fourth integrity-member extension is recorded.
- Context: three prior changes amended ADR-028 for the same member; this
  change tightens the persona disposition.
- Question: amend or route to 8.22?
- Options:
  1. **Amend ADR-028 a fourth time.**
     Meaning: one sentence in the ADR and chapter 9.
     In practice: the register stays consistent with its own precedent.
     Gain: accurate history.
     Cost: one sentence.
  2. **Route to chapter 8.22 only.**
     Meaning: no ADR change.
     In practice: an inconsistent register.
     Gain: smaller diff.
     Cost: precedent drift.
- Resolution: **Option 1** because the same boundary is tightened and the
  prior three extensions are recorded. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Tech choices | The design said the project root is two levels up from the change dir; it is three (the existing `check_promised_files_exist` uses three parent calls). Corrected in design.md. |
| Testability | `ensure_fixture_usecase` only applies under `givn/specs/`, so the lint fixture's Given must write a complete use-case document from `usecase_md_template()` at the change path; corrected in design.md. |
| Testability | The dangling-persona lint scenario exits 0: the fixture has no `givn/specs/` corpus, so the overlap and use-case scans are quiet, and the advisory never sets an exit flag. |
| Testability | Two scenarios are legitimate immediate-GREEN guards (the exempt-path reasoned none and the preserved-behaviors scenario); recorded in tasks.md as reuse. |
| Testability | Strict mode, `GIVN_FEATURES`, and the single-scenario command are valid. |
| E2E fidelity | No new `@e2e` scenario is needed; the matrix maps existing actions and the new scenarios are variants. |
| Architecture documentation | Chapters 5 and 6 needed the persona wording; chapter 3's reason was corrected; the ADR-028 amendment is recorded. |
| Scope | givn's own persona files still use the old schema; migrating them is out of scope per the handover and proposal. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `ideation-skill` keeps fragment ownership; `visual-review-gate` and `lint-command` extend their use cases. | accepted |
| Typed Includes / Extends relationships | The support fragment is included by the use cases; no relationship changes. | accepted |
| Confirmed Persona coverage | `agent--technical-role` (guidance) and `givn-maintainer--owner` (rule corpus, glossary) are confirmed permanent Personas. | accepted |
| Actor and Persona distinction | Gherkin keeps the maintainer, reviewer, and agent as actors; Personas stay review lenses, and the guidance's actor separation is asserted. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | Consumes the generated ideation guidance, the persona template, and the lint advisory. | The ideation and template scenarios |
| givn-maintainer--owner | Owns the persona convention and the glossary rows. | The preserved-behaviors scenario; chapter 8.22 |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | — |
| 2 | 02 architecture-constraints | No | No | Yes | — |
| 3 | 03 context-and-scope | No | No | Yes | — |
| 4 | 04 solution-strategy | No | No | Yes | — |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes — `cli::lint` and `integrity_hook` rows updated |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — 6.4 persona scan; 6.6 ID resolution |
| 7 | 07 deployment-view | No | No | Yes | — |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.22 |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — ADR-028 fourth amendment in the ADR and the register |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-46 mitigation |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — `Persona segment`, `Persona promotion` |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] The ADR-028 amendment is recorded; no new MADR is required.
- [x] The amendment's consequence (advisory noise on legacy title entries) is
      recorded in design.md and chapter 8.22.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/persona-identity-reuse/specs/fragments/ideation-skill.feature` | Added the collapse and record-only phrases, the preserved-behavior assertions, the command assertions, and the template scenario. |
| `givn/changes/persona-identity-reuse/specs/verify-change/lint-command.feature` | Rewritten to four scenarios: dangling, project-resolved, topic-resolved, and `--change` scope. |
| `givn/changes/persona-identity-reuse/specs/verify-change/visual-review-gate.feature` | Added the unknown-persona failure, the reasoned-none failure, and the exempt-path guard. |
| `givn/changes/persona-identity-reuse/design.md` | Corrected the root derivation and the fixture writer; ID-only matching note; reasoned-none behavior note; spec coverage update. |
| `givn/changes/persona-identity-reuse/arc42.md` | Row 3 corrected; row 9 marked Yes with the ADR-028 amendment. |
| `docs/arc42/05-building-block-view.md` | `cli::lint` row names the persona scan. |
| `docs/arc42/06-runtime-view.md` | 6.6 records the ID resolution. |
| `docs/arc42/09-architecture-decisions.md` and `docs/arc42/adr/ADR-028-hooks-and-mandatory-checks.md` | Fourth amendment sentence. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change persona-identity-reuse` exits 2 (WIP, no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
