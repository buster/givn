# Design Review: require-persona-for-interface-none

Status: all questions resolved; hardening applied. No blocking question remains.

Reviewed 2026-10-21 against `proposal.md`, the delta feature, `design.md`,
`arc42.md`, all twelve live Arc42 chapter files, the permanent
`verify-change` use case and `visual-review-gate` feature, the current
`src/integrity_hook.rs`, the `visual-review-gate` step definitions, the
design/review/design-review instruction assets, the templates, and the
permanent `create-change/visual-design-contract` corpus. This review changes no
production code, no `tasks.md`, no `review.md`, and no revision.

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Does the spec contain every proposal point, including the guidance/template requirement? | Finding: the gate behavior was covered, but proposal point 5 (guidance/template text states the rule) had no executable assertion. Added a review-guidance scenario. |
| 2 | Tech choices | Is one control-flow branch in `check_visual_review` the right fit, with no new primitive or dependency? | Confirmed. `parse_visual_contract`, `VisualContract`, `VisualInterface`, `check_persona_disposition`, and the cucumber-rs runner are reused. |
| 3 | Missing scenarios | Are `PERSONAS: none`, empty, missing `review.md`, a default-`none` contract, and a misdeclared interface value covered? | Finding: a contract section with no recognized `- Interface:` line defaults to `Interface: none` and is a declared contract, but no test locked that boundary. Added `interface_none_default_requires_persona`. |
| 4 | Testability | Strict mode, not-implemented stub, single-scenario command, concrete assertions? | Confirmed. `.fail_on_skipped()` plus the `skipped > 0` exit, `todo!` step bodies, and the named `cargo test --test features_runner -- --name ...` command. The three RED runs below were executed. |
| 5 | E2E fidelity | Does the change use the real CLI boundary and avoid new `@e2e` scenarios? | Confirmed. Every scenario drives `CARGO_BIN_EXE_givn` through `common::run_givn`; the two inventory mappings stay on their existing `@e2e` scenarios; the new scenarios are regular variants. |
| 6 | Interaction coverage | Does every `## Interactions` entry have a matrix row, with no new action? | Confirmed. Two entries, two rows; the `Interface: none` and guidance cases are variants of the existing review-check and guidance actions. |
| 7 | Risk | What is the most likely implementation failure and its mitigation? | Finding: a guidance rewrite can drop a phrase asserted by the permanent corpus, and the misdeclared-`none` visual-evidence bypass can be overclaimed as closed. Completed the phrase list, required the full non-e2e run, and clarified R-59. |
| 8 | Use-case and Persona context | Are the use-case contract, capability ownership, typed relationships, and confirmed Personas preserved? | Confirmed. `verify-change` owns the goal; `visual-review-gate` owns the capability; the Includes fragment is unchanged; all three confirmed Personas are mapped; no Persona biography is used as an actor. |
| 9 | Decision ledger | Does D1 resolve in the design? | Confirmed. `## Decisions` resolves D1; no ideation seed and no carried question. |
| 10 | Visual design contract | Does the change's own `Interface: none` contract require a Persona while needing no verdict or evidence? | Confirmed. The design declares the contract deliberately, records the Persona requirement, and carries no viewports or evidence. No rendered options are needed because no screen is introduced. |
| 11 | Deferrals | Is any mandatory check deferred? | None. No `DEFERRAL:` or operator decision is required. |
| 12 | Interface classification | Is the CLI classification tested against the opposite Web UI classification? | Confirmed. The change introduces no rendered page; a rendered page would be a Web UI, so `Interface: none` is correct and no browser driver is proposed. |
| 13 | ADR qualification | Is `NOT_QUALIFIED`/`CANONICAL_ARTIFACT` correct, with a completed existing-ADR check? | Confirmed. The structured verdict has the required `existing_adr_check: PASS`; ADR-028's boundary and requirement are unchanged; the trigger is canonical capability behavior; no ADR file is added or amended. |
| 14 | Arc42 | Does the independent twelve-row assessment match `arc42.md`, and does each affected chapter match `design.md`? | Finding: the independent set matches (05, 06, 08, 09, 10, 11, 12), but `design.md`'s affected-files table omitted 05 and 09 and described 10 and 11 as unchanged while `arc42.md` marks them Yes and the chapters carry the new text. Corrected the table and clarified R-59 in chapter 11. |
| 15 | Ubiquitous language | Are the design's terms already in the glossary? | Confirmed. `Visual Design Contract`, `Persona disposition`, `Visual verdict`, and `Visual evidence` already exist; two definitions and the invariants are updated; no new term is introduced. |
| 16 | Scenario title | Is the modified scenario title honest and merge-safe? | Finding: the title is merge-safe because it is unchanged, but it is broad enough to be read as a full exemption. Added a scenario description that pins the exemption to the visual verdict and evidence; rejected a rename because it would turn an in-place modification into a removed+added pair. |

### Q1 — Guidance point 5 has no executable assertion

- Status: closed 2026-10-21 (review finding, resolved by hardening)
- Blocks: whether the change proves the served review instruction teaches the
  new rule, not only that the binary enforces it.
- Context: the proposal's point 5 requires the design guidance, review
  guidance, template, and durable documentation to state the contract-based
  rule. The permanent corpus asserts the old phrases but would still pass if
  the review instruction kept saying only "a user-visible surface requires at
  least one confirmed Persona." A reviewer could therefore implement the gate
  and leave the teaching text stale.
- Question: should the delta prove that the served review instruction names
  `Interface: none` and the contract-based Persona rule?
- Resolution: **yes**. Added `Review guidance requires a persona for a
  declared interface none contract` as a regular `@givn.added @wip` scenario.
  It runs `givn instructions review --change visual-review-gate` and asserts
  `Interface: none` and `declared visual design contract`. It reuses the
  existing guidance steps, so no new step definition is needed. Rejected:
  relying on the gate scenarios alone, because they do not test the served
  text; adding a `create-change` scenario, because the proposal's Capability
  Routing decision is `EXTEND visual-review-gate` and the design guidance and
  template edits remain durable-documentation changes reviewed against the
  permanent phrase corpus.

### Q3 — The default-`none` contract boundary

- Status: closed 2026-10-21 (review finding, resolved by hardening)
- Blocks: whether "any declared `## Visual Design Contract`" includes a
  section with no recognized `- Interface:` line or an unrecognized value.
- Context: `parse_visual_contract` starts with `VisualInterface::None` and
  falls back to `None` for any unrecognized value. A section that says
  `- Interface: tablet`, or omits the line, is therefore a declared contract
  that takes the new Persona branch. The design's explicit tests covered
  `- Interface: none` and no contract, but not this boundary.
- Question: should the parser default be locked to the contract-based rule?
- Resolution: **yes**. Added `interface_none_default_requires_persona` to the
  test plan: a contract section with no recognized `- Interface:` line and a
  contract with an unrecognized value both fail without a confirmed Persona.
  This keeps the parser default from silently becoming an exemption.

### Q7 — The two implementation risks

- Status: closed 2026-10-21 (review finding, resolved by hardening)
- Blocks: whether the guidance rewrite preserves the permanent corpus and
  whether the residual-risk record tells the truth about misclassification.
- Context: the change edits `assets/instructions/*` and the templates, which
  the permanent `create-change/visual-design-contract` and `verify-change`
  features assert phrase by phrase. The design's first phrase list omitted
  `per persona`, `relevance and evidence`, `Relevance`, `Evidence`,
  `PERSONAS:`, `DEFERRAL:`, `OPERATOR-DECISION:`, and the template marker
  lines. Separately, the contract-based Persona rule closes only the Persona
  half of the declared-`none` bypass; a screen misclassified as
  `Interface: none` still avoids the visual verdict and evidence.
- Question: is the mitigation sufficient, and is the residual risk stated
  accurately?
- Resolution: **yes, after hardening**. The phrase list now enumerates the
  asserted phrases in the paragraphs this change edits and requires surgical
  edits that leave the other guidance sections verbatim; the full
  `./run-tests.sh --non-e2e` run is the regression net. `design.md` and
  `docs/arc42/11-risks-and-technical-debt.md` now state that the Persona half
  closes, the misdeclared-`none` visual-evidence half remains, and the
  `Interface:` classification check plus the fabrication audit are the
  mitigations. Rejected: claiming the whole declared-`none` bypass is closed;
  that would hide a known limitation.

### Q14 — The affected-files table disagreed with `arc42.md`

- Status: closed 2026-10-21 (review finding, resolved by hardening)
- Blocks: whether the design's implementation plan names every chapter the
  change actually edits.
- Context: the independent chapter assessment is 05, 06, 08, 09, 10, 11, and
  12 affected. `arc42.md` claims the same. The chapter files carry the
  updates. `design.md`'s affected-files table omitted 05, 09, and the
  `docs/arc42/README.md` index, and described 10 and 11 as merely reviewed
  and unchanged, while chapter 10 has the broadened Q-89 and new Q-119 and
  chapter 11 has the updated R-59.
- Question: should the table be corrected?
- Resolution: **yes**. Added `docs/arc42/05-building-block-view.md`,
  `docs/arc42/09-architecture-decisions.md`, and `docs/arc42/README.md`;
  changed the 10 and 11 rows to describe the actual updates. No chapter was
  added to `arc42.md`; the independent assessment and the claim already
  matched.

### Q16 — The modified scenario title

- Status: closed 2026-10-21 (review finding, resolved by hardening)
- Blocks: whether the permanent scenario's title can be read as the old full
  exemption after the rule changes.
- Context: `A change that declares no visual interface is exempt` is the
  existing title. `@givn.modified` replaces a scenario by title, so keeping
  the title gives a clean in-place merge. The body now requires a confirmed
  Persona, which makes the bare word "exempt" ambiguous. Renaming would need
  `@givn.removed` plus `@givn.added`, turning the in-place modification into a
  removed+added pair and adding an overlap disposition.
- Question: keep the title, rename it, or pin the exemption in the body?
- Resolution: **keep the title and pin the exemption in the scenario
  description**: "A declared Interface: none contract still requires a
  confirmed Persona; only the visual verdict and visual evidence are exempt."
  The title remains merge-safe, the description removes the ambiguity, and
  the Persona step proves the requirement. Rejected: a rename, because the
  added churn buys no behavioral coverage and the description already states
  the narrower exemption.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Existing fixtures | `Given a givn project with a change that has passed design-review` lives in `tests/steps/workflow_enforcement_steps.rs`, writes `DESIGN-REVIEW: PASS`, creates `givn/personas/fixture-persona.md`, sets the run declaration, and writes the change receipt. The new `Given` step only has to write `review.md` with `README-IMPACT: none`, `REVIEW: PASS`, and `PERSONAS: fixture-persona`, and no `VISUAL-REVIEW:` line. |
| Existing failure steps | `the change's review.md declares no persona disposition` writes `VISUAL-REVIEW: PASS` and no `PERSONAS:` line; `the change's review.md declares the persona disposition "<value>"` writes `VISUAL-REVIEW: PASS` and the value. Both reuse cleanly for `Interface: none`. |
| Existing unit tests | `persona_disposition_requires_a_confirmed_persona` covers resolved, unknown, `none`, reasoned none, empty, and missing-line values. `no_visual_contract_accepts_not_applicable` covers the no-contract `VISUAL-REVIEW: NOT APPLICABLE` path. Both remain valid; the new tests add the `Interface: none` branch and the parser default. |
| Guidance corpus | `givn/specs/create-change/visual-design-contract.feature` asserts the design/proposal/review/design-review guidance and template phrases; `givn/specs/verify-change/visual-review-gate.feature` asserts the review guidance and template phrases. The completed phrase list covers every assertion found; the full non-e2e run is the guard. |
| Review instruction | `assets/instructions/review.md` keeps the visual review block conditional on a non-`none` interface; the Persona requirement belongs in the unconditional `Use-case and Persona conformance` paragraph. The `givn-review` skill template carries no rule and needs no edit. |
| ADR-028 | The register records four prior amendments that added members or resolution rules. This change adds no member and moves no boundary; the current-trigger note in chapter 09 and the `NOT_QUALIFIED`/`CANONICAL_ARTIFACT` verdict are sufficient. |
| Arc42 chapters | All twelve files exist, contain real content, and contain no box-drawing or pseudo-diagram art. Chapters 01, 02, 03, 04, and 07 need no edit. Chapters 05, 06, 08, 09, 10, 11, and 12 carry the described text. The `docs/arc42/README.md` index carries the change's current-fact summary. |
| Use-case inventory | `givn/specs/verify-change/usecase.md` has two `visual-review-gate` interactions: run the review check on a visual change, and read the visual review guidance. Both matrix rows map to existing `@e2e` scenarios; no third action is introduced. |
| Lint | `givn lint --change require-persona-for-interface-none` exits 2 with four `@wip` findings and shape/subset advisories only; no parse error, duplicate title, tag error, or `UCDOC` finding. |
| RED proof | The guidance scenario fails on missing `Interface: none`; the missing-Persona scenario fails because the current gate exits 0; the modified scenario fails on the undefined new `Given` step. Each is a concrete RED observation, not "it ran." |

## Open questions

None. Every grilling question was resolved from the artifacts, the live code,
or a hardening edit recorded above.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `verify-change` keeps its goal, main flow, rules, guarantees, and capability set. `visual-review-gate` owns the behavior; no new capability or interaction is introduced. | None required; confirmed. |
| Typed Includes / Extends relationships | `Includes` the `support` fragment; no `Extends`. Unchanged. | None required. |
| Confirmed Persona coverage | All three proposal Personas exist in `givn/personas/` and are named by the permanent use case: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`. | None required; confirmed. |
| Actor and Persona distinction | Actors remain Developer and Supporting Reviewer; Personas remain review lenses. No biography is used as a Gherkin actor. | None required. |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| `developer--primary-user` | Runs `givn check review` and now must record a Persona for a change whose design declares `Interface: none`. | Passing and failing `Interface: none` scenarios; unit tests. |
| `agent--technical-role` | Authors `design.md` and `review.md` and reads the served review guidance; the rule changes what it must record and what it is told. | New review-guidance scenario; `assets/instructions/*` updates. |
| `givn-maintainer--owner` | Owns the rule corpus, templates, and durable architecture documentation; this change keeps them aligned. | Affected-files table, Arc42 chapters 05/06/08/09/10/11/12, and the ADR register note. |

## Architecture documentation (arc42) check

The reviewer's own assessment was formed from `proposal.md`, the delta
feature, and `design.md` before reading `arc42.md`'s claims.

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | N/A — no new goal or stakeholder. |
| 2 | 02 architecture-constraints | No | No | Yes | N/A — no new constraint. |
| 3 | 03 context-and-scope | No | No | Yes | N/A — no new external system or surface. |
| 4 | 04 solution-strategy | No | No | Yes | N/A — no strategy change. |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes — `integrity_hook` row states the contract-based Persona trigger and the interface-based verdict/evidence. |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — 6.6 states the absent-section exemption, the `none` Persona-only branch, the `web`/`terminal` order, and the Mermaid flow. |
| 7 | 07 deployment-view | No | No | Yes | N/A — no deployment change. |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.17 and 8.22 define the section's presence as the Persona boundary and `none` as a declared contract. |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — current-trigger register note; no ADR file or amendment; ADR-028 boundary unchanged. |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes — Q-89 broadened and Q-119 added. |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-59 records the closed Persona half, the remaining misdeclared-`none` visual-evidence path, and the omitted-section exemption. |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — `Visual Design Contract` and `Persona disposition` definitions and the visual-review invariants match the design. |

- [x] All 12 rows assessed independently against `proposal.md`/`design.md`
      before reading `arc42.md`'s own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and `arc42.md`
      saying "No" or omitting the row.
- [x] Every qualified ADR candidate in `design.md` has a corresponding MADR
      entry in chapter 09; the only candidate is
      `NOT_QUALIFIED`/`CANONICAL_ARTIFACT` to `visual-review-gate`, and no
      MADR entry is expected.
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in
      chapter 11; no new ADR exists, so this is N/A.
- [x] Every ADR candidate records the structured qualification verdict and its
      route-specific target; `Must be shared` is supporting evidence only.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/require-persona-for-interface-none/design.md` | Added Arc42 chapters 05, 09, and the `docs/arc42/README.md` index to the affected-files table; corrected the 10 and 11 rows; completed the permanent-corpus phrase list; added the default-`none` unit test and the review-guidance scenario to the test plan; changed the local run command to the configured `cargo run -- instructions`; clarified the misdeclared-`none` residual risk; recorded the scenario-title-description rationale and the overlap dispositions. |
| `givn/changes/require-persona-for-interface-none/specs/verify-change/visual-review-gate.feature` | Added the scenario description pinning the exemption to the visual verdict and evidence; added the review-guidance scenario. |
| `docs/arc42/11-risks-and-technical-debt.md` | Clarified R-59: the Persona half of the declared-`none` bypass closes; the misdeclared-`none` visual-evidence path remains; the classification check and fabrication audit remain the mitigation. |

## Verification

- `givn lint --change require-persona-for-interface-none`: exit 2 with four
  `@wip` findings and shape/subset advisories only. No parse error, duplicate
  title, tag error, or `UCDOC` finding.
- RED runs executed:
  - `Review guidance requires a persona for a declared interface none contract`
    fails on `stdout missing "Interface: none"`.
  - `A change that declares the interface none without a persona disposition
    fails review` fails because the current gate exits 0; the fixture and the
    new scenario are valid.
  - `A change that declares no visual interface is exempt` fails on the
    undefined new `Given` step, so it is a genuine RED before the step is
    written.
- `givn status --change require-persona-for-interface-none`: the
  `design-review` artifact is present; `tasks` is the next artifact.
- `git diff --check`: clean.
- No production code, `tasks.md`, `review.md`, `arc42.md`, or revision was
  written by this review.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D1`) resolves in the design's `## Decisions` ledger; no question is carried.
- [x] All questions are resolved.
- [x] `design.md` reflects the decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing guidance and default-`none` cases.
- [x] `givn lint --change require-persona-for-interface-none` exits 0 or 2 (no parse errors).
- [x] Architecture documentation (arc42) check completed above for all twelve rows.

**DESIGN-REVIEW: PASS**
