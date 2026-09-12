# Design Review: visual-ui-review

## Grilling log

A fresh-context grilling subagent read the proposal, the delta specs, the
design, `arc42.md`, the canonical specs policy, the integrity hook, and the
test harness, then returned eight ranked questions. Each was answered from
codebase evidence or an established repository convention; none required user
arbitration. The subagent's answers are recorded here and applied in the
hardening pass.

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | ADR qualification | Is the visual gate a canonical-artifact route, or does it amend ADR-028 like the run-declaration gate did? | ADR-028 already carries the run-declaration amendment in chapter 09; the visual gate extends the same non-deactivatable mandatory-check boundary. | Applied: `AMEND_ADR`, target ADR-028; chapter 09 amendment added; `arc42.md` row 9 changed to Yes. |
| 2 | Gate mechanics | Is the gate fail-closed when a contract declares an interface but evidence is degenerate? | Fail-closed: web without viewports fails, both verdict markers fail, zero-byte evidence counts as missing. | Applied to `design.md` `### Gate semantics (fail-closed)`. |
| 3 | Missing scenarios | Are the boundary cases `Interface: none`, missing `review.md`, and zero-byte evidence covered? | Add the boundary scenarios; the missing-`review.md` case is already owned by the overlap gate's review.md check. | Applied: `Interface: none`, no-viewports, and empty-evidence scenarios added. |
| 4 | Scope | Where do "Samsung S24" and "Full HD" live — specs instruction or design instruction? | The canonical specs instruction owns the viewport classes and the evidence rule; the design instruction owns the exact dimensions and the contract format. | Applied: both surfaces name the classes; `design.md` architecture impact records the split. |
| 5 | Scope | The design-review instruction and both templates were promised but unasserted. | Assert them: one `givn instructions design-review` scenario and one ejected-template scenario per template. | Applied: two new scenarios; `visual_design_contract_steps.rs` gains the design-template step. |
| 6 | Tech choices | Is the `- Evidence:` contract line parsed or decorative? | It was decorative; drop it from the parsed format and keep the fixed path convention in prose. | Applied to `design.md`. |
| 7 | Testability / risk | The gate's line parser would read the fenced example contract in this change's own `design.md` and demand screenshots. | Strip fenced code blocks before scanning; unit-test that this change's `design.md` parses to exempt. | Applied: `design.md` gate semantics and the `parse_visual_contract` primitive. |
| 8 | Gate mechanics | Does "committed evidence" need a Git-tracked check? | No; the machine check verifies presence in the change tree, keeping non-Git fixtures testable. "Committed" stays a process obligation. | Applied to `design.md`. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Scope | Proposal items map to scenarios; the two gaps found (contract content, promised templates) were closed in hardening. |
| Tech choices | No new dependency; `check_visual_review` fits `run_integrity_hook`, which already resolves the change directory. |
| Missing scenarios | Three boundary scenarios added (no interface, no viewports, empty evidence); the missing-`review.md` case is owned by the existing overlap gate. |
| Testability | Existing fixtures (`a givn project with a change that has passed design-review`, `the project declares a complete run declaration`, generic run/stdout/exit steps, generated-guidance steps) cover every new scenario; the fixture's `review.md` must also carry `README-IMPACT: none` for the readme addon. |
| E2E fidelity | One `@e2e` per action: the modified prose-consolidation scenario keeps its tag; the two new capabilities each carry their distinct actions; variants stay regular scenarios. CLI subprocess is the real driver. |
| Interaction coverage | Every delta `usecase.md` interaction row maps to a matrix row and an existing `@e2e` title; no mismatch. |
| Risk | The self-blocking fenced example was the top risk; closed by fence-aware parsing plus a regression unit test. Second risk: archive blockers from routing names and narrative base hashes; both fixed. |
| Use-case / Persona | Both use cases declare `Personas: none`; no Persona was invented or promoted; actors stay distinct. |
| ADR qualification | The gate is a refinement of ADR-028 (mandatory checks cannot be disabled by hooks); routed `AMEND_ADR`, not a new record. |
| arc42 | Independent 12-row walk matched `arc42.md` after row 9 was corrected to Yes; all Yes chapters contain decision-specific content; no ASCII-art diagrams; glossary holds all seven new terms; chapter 11 holds R-56..R-59 and D-18. |
| Ubiquitous language | All new terms are in `docs/arc42/12-glossary.md`; "style tile" is glossary-held and used by the design-review guidance scenario. |
| Capability routing | The proposal table used display names, not kebab-case capability IDs, which would fail the exact-match routing check; fixed. Delta usecase.md files lacked correct `base-sha256` markers; fixed to the current permanent hashes. |
| Gate mechanics | Integration is clean; pitfalls closed: fence handling, marker ordering, `@givn.removed` exclusion, zero-viewport failure, zero-byte evidence, exemption for this change's own `design.md`. |

## Open questions

None. Every grilling question had a codebase- or convention-backed answer.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `prose-consolidation` extended under `author-change`; `visual-design-contract` is new in `author-change`; `visual-review-gate` is new in `verify-change`. | Accepted |
| Typed Includes / Extends relationships | Both use cases include `fragment: support`; no new typed relationship is introduced. | Accepted |
| Confirmed Persona coverage | Both use cases declare `Personas: none`; the persona vision lens activates only for confirmed Personas, so the fixed technical lenses carry this change. | Accepted |
| Actor and Persona distinction | Actors remain interaction roles (`Developer`, `Reviewer`, `Authoring agent`); Personas remain review lenses; no biography is used as an actor. | Accepted |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | N/A |
| 2 | 02 architecture-constraints | No | No | Yes | N/A |
| 3 | 03 context-and-scope | No | No | Yes | N/A |
| 4 | 04 solution-strategy | No | No | Yes | N/A |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes — integrity_hook row names the visual gate |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — 6.6 documents the gate and its exemptions |
| 7 | 07 deployment-view | No | No | Yes | N/A |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.17 matches the contract format and gate semantics |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — ADR-028 amendment recorded, matching the run-command precedent |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes — Q-85 to Q-87 |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-56 to R-59, D-18 |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — seven new terms plus visual-review invariants |

- [x] All 12 rows assessed independently against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md
      saying "No" or omitting the row.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry
      in chapter 09; the gate is a refinement and amends ADR-028.
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in
      chapter 11 (risks-and-technical-debt) — R-56 to R-59.
- [x] Every ADR candidate records the structured qualification verdict and its
      route-specific target; `Must be shared` is supporting evidence only.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `proposal.md` | Capability Routing table cells changed to the kebab-case capability IDs so the exact-match routing check passes. |
| `specs/author-change/usecase.md`, `specs/verify-change/usecase.md` | Added the current permanent `base-sha256` markers. |
| `specs/author-change/visual-design-contract.feature` | Contract-content assertions added; design-review-guidance and design-template scenarios added. |
| `specs/verify-change/visual-review-gate.feature` | `Interface: none`, web-without-viewports, empty-evidence, and review-template scenarios added. |
| `design.md` | Fence-aware parsing, fixed gate ordering, fail-closed rules, dropped decorative `- Evidence:` line, viewport-ownership split, README-IMPACT fixture note, ADR-028 amendment row. |
| `arc42.md` | Row 9 changed to Yes with the ADR-028 amendment; ADR action changed to `AMEND_ADR`. |
| `docs/arc42/09-architecture-decisions.md` | ADR-028 amendment paragraph added. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change visual-ui-review` exits 2 (WIP and advisory shape
      warnings only, no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
