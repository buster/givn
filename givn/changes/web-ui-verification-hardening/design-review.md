# Design Review: web-ui-verification-hardening

## Grilling log

A fresh-context grilling subagent read proposal.md, the three delta
`.feature` files, design.md, arc42.md, assumptions.md, the canonical assets,
the integrity hook, and the test fixtures, and returned a ranked finding
list. The operator pre-authorized assumptions for unresolved questions
(recorded in `assumptions.md`), so each question is answered and resolved.

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | Testability | The "state screenshot" assertion matches the pre-existing phrase "final-state screenshot" and can never fail RED. | Assert a phrase only the new guidance emits. | Resolved: the scenario now asserts "state screenshots the review saw" and "each declared UI state". |
| 2 | Testability | The positive gate scenarios pass without any production change. | Pair each positive with its negative in one task; the negative drives RED. | Resolved as A10; design.md records the pairing. |
| 3 | Testability | The persona check breaks four permanent visual scenarios because the shared fixture writes no `PERSONAS:` line. | Update the shared fixture in the same task. | Resolved: design.md's Step Definitions and Compatibility constraints name the fixture update. |
| 4 | Testability | A new "ejected review template" step would collide with the existing step in `arc42_review_alignment_steps.rs` (ambiguous match). | Reuse the existing step; add only proposal and design-review steps. | Resolved in design.md's Step Definitions table. |
| 5 | Scope | The proposal says per traversed scenario, the gate checks per change; the design claims assumptions.md records the split, which A5 did not. | Record the split explicitly. | Resolved: A5 and design.md now state the guidance/gate boundary. |
| 6 | Domain context | This change's `Personas: none` contradicts the user-visible-surface persona rule it introduces. | Record the reason and flag the disposition for revision; never invent a persona. | Resolved as A12; recorded in proposal.md, design.md, and assumptions.md. |
| 7 | Testability | Several assertions are vacuous because the phrases pre-exist (`confirmed Persona`, `disposition`, `reason`, `Playwright`, `browser driver`). | Assert new, disposition-specific phrases. | Resolved: the scenarios now assert "disposition for every confirmed Persona", "at least one confirmed Persona", "record the reason", "relevance and evidence", "is a Web UI row", "needs a browser driver", "how each is verified". |
| 8 | arc42 | R-57's mitigation is stale once state evidence lands. | Revise R-57. | Resolved: R-57 now names the declared-state evidence; arc42.md notes the revision. |
| 9 | Testability | Boundary scenarios are missing (empty persona value, empty state file, terminal persona, empty `States:`). | Add the decidable boundaries; record the rest. | Resolved: two boundary scenarios added; terminal uses the same persona check; empty `States:` is A14 with parser unit tests. |
| 10 | ADR | The AMEND_ADR route lacks the structured verdict; the ADR body is stale. | Record the verdict and update the ADR body. | Resolved: the verdict is in arc42.md; ADR-028's Decision Outcome names the three amendments and drops the stale "retrieval" wording. |
| 11 | Ubiquitous language | The glossary definition of "Persona disposition" conflates the proposal's `none` with the review marker. | Scope the definition to the review marker. | Resolved: the glossary now defines only the `PERSONAS:` review record. |
| 12 | arc42 | Chapter 8.17 lists `- Evidence:` as a machine-readable contract line, but no parser reads it. | Move the evidence path out of the parsed-line list. | Resolved: chapter 8.17 separates the fixed evidence path from the parsed lines. |
| 13 | Scope | The instruction rewrites must preserve phrases asserted by permanent scenarios. | List the preserved phrases as a design constraint. | Resolved: design.md's Compatibility constraints lists them. |
| 14 | Tech choices | A scaffolded `DEFERRAL:` line in the review template would trigger the check, and fence handling is unspecified. | Strip fenced blocks; omit both lines when nothing is deferred. | Resolved as A11; design.md's Gate semantics states it. |
| 15 | Scope | Proposal items 4 and 5 have no observable assertion. | Add the missing phrases. | Resolved: the review scenario asserts the state screenshots the review saw; the design scenario asserts "how each is verified". |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| E2E fidelity | The delta adds no new consumer action; the changed gate action is already covered by the permanent `@e2e` scenario "A visual change with complete evidence passes review". The new scenarios are regular variants; overlap dispositions are expected and owned by review. |
| Visual contract | This change introduces no screen; `parse_visual_contract` returns `None` and the visual gate is exempt. No `## Visual Design Contract` section is correct. |
| Interaction Coverage | The five matrix rows cover every touched inventory entry with valid CLI mechanisms. |
| Testability | The fenced-example unit test in `src/integrity_hook.rs` reads the archived `visual-ui-review` design and is a no-op; it is retargeted to an inline fixture. |
| Gate ordering | The persona check must run after the verdict check and before the evidence check so the existing evidence-failure scenarios keep their error message. |

## Open questions

None. Every question is resolved above or recorded as an assumption in
`assumptions.md` (A5, A10–A14).

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `author-change` owns `prose-consolidation` and `visual-design-contract`; `verify-change` owns `visual-review-gate`. The proposal's routing table matches the delta capabilities. | Accepted. |
| Typed Includes / Extends relationships | Both use cases keep `fragment: support`; no typed relationship changes. | Accepted. |
| Confirmed Persona coverage | Both use cases declare `Personas: none`; this change records `none` with the reason and flags it (A12). No persona was invented or promoted. | Accepted as A12; revisit after the change. |
| Actor and Persona distinction | Gherkin keeps `Developer`/`Reviewer`; personas remain review lenses. | Accepted. |

## Architecture documentation (arc42) check

Independent assessment first, then diffed against `arc42.md`.

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | N/A |
| 2 | 02 architecture-constraints | No | No | Yes | N/A |
| 3 | 03 context-and-scope | No | No | Yes | N/A |
| 4 | 04 solution-strategy | No | No | Yes | N/A |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes — the integrity hook row names the new checks |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — section 6.6 documents the extended gate |
| 7 | 07 deployment-view | No | No | Yes | N/A |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — section 8.17 names classification, states, persona, deferral |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — third ADR-028 amendment and the structured verdict |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes — Q-88 to Q-91 |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-57 revised; R-60 to R-62 and D-19 |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — classification, state evidence, persona disposition, deferral decision |

- [x] All 12 rows assessed independently against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md
      saying "No" or omitting the row.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry
      in chapter 09; non-qualifying decisions name exactly one canonical target.
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in
      chapter 11 (risks-and-technical-debt).
- [x] Every ADR candidate records the structured qualification verdict and its
      route-specific target; `Must be shared` is supporting evidence only.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `specs/verify-change/visual-review-gate.feature` | Assertion phrases made fail-first; two boundary scenarios added (empty persona value, empty state screenshot). |
| `specs/author-change/visual-design-contract.feature` | Classification, matrix, breakpoint, and persona assertions made fail-first; review/design-review template scenarios added. |
| `design.md` | Fixture update, step reuse, gate ordering, fence stripping, compatibility constraints, and the guidance/gate split recorded. |
| `arc42.md` | Structured ADR-028 amendment verdict added; chapter 11 summary reflects the R-57 revision. |
| `docs/arc42/05`, `06`, `08`, `09`, `10`, `11`, `12` | Extended with the new checks, quality scenarios, risks, debt, terms, and the ADR body amendment. |
| `assumptions.md` | A5 split, A10–A14 added. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated where grilling identified missing or
      vacuous scenarios.
- [x] `givn lint --change web-ui-verification-hardening` exits 0.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
