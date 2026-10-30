# Design Review: standalone-domain-model

Review kind: self-review
Author: Maki
Reviewer context: Maki

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| Q1 | Scope | Does the spec contain exactly the proposal's vocabulary move, structural model, scaffold, and guidance changes? | Closed — every proposal claim maps to a capability row and at least one scenario. The template scaffold scenario was added during hardening. |
| Q2 | Tech choices | Is the scaffold better served by a new command or schema? | Closed — no. Two embedded assets plus `scaffold_document` reuse the existing asset tree and addon lifecycle. |
| Q3 | Missing scenarios | Are boundary cases covered: existing files, `--no-addons`, arc42-only, domain-modeling-only, disabling? | Closed — after hardening, all five are explicit scenarios. |
| Q4 | Missing scenarios | Is the chapter 12 link actually asserted, not just the glossary file? | Closed — hardening added `../glossary.md` and the absence of a term table. |
| Q5 | Testability | Can each scenario fail in RED without production code? | Closed — the new file assertions and served-instruction phrase assertions fail before the assets and scaffold land. Existing step bindings are extended in the named permanent files. |
| Q6 | Testability | Is the strict-mode mechanism and single-scenario command named? | Closed — `.fail_on_skipped()` and the `GIVN_FEATURES=... --name` command are in the design. |
| Q7 | E2E fidelity | Does the change invent a new E2E action or weaken an existing one? | Closed — no new action. Existing CLI smokes remain the evidence; file assertions are secondary scaffold evidence. |
| Q8 | Interaction Coverage | Does every inventory entry touched by the change appear in the matrix? | Closed — the matrix maps enable-an-addon, initialize, and sync/instructions to existing smokes. |
| Q9 | Risk | What is the most likely implementation failure? | Closed — phrase drift between the template and the instruction assertions, and accidental overwrite. Mitigations: exact-phrase assertions and an existence check before every write. |
| Q10 | Risk | What is the migration risk? | Closed — R-127 records the two-location interval; guidance requires completion before relying on the new glossary. |
| Q11 | Use-case and Persona context | Are the use cases and confirmed Personas preserved? | Closed — `obtain-effective-guidance` owns the guidance, `initialize-project` the scaffold, and the support fragment the routing/review rules. The three confirmed Personas are review lenses, not Gherkin actors. |
| Q12 | Decision ledger | Does every `D<n>` resolve? | Closed — D1–D4 are user-answered and D5–D7 are answered by the improvement direction; the ledger names the realization. |
| Q13 | Contracts and components | Does one component implement one contract? | Closed — hardening split effective guidance, generated guidance, and addon enablement into three contracts with one component each. |
| Q14 | Decision timing | Is any foundational question carried? | Closed — none carried. |
| Q15 | Visual design contract | Is the interface classification correct? | Closed — terminal text, no browser-rendered screen; the opposite classification fails because no HTML is rendered. |
| Q16 | Deferrals | Is any mandatory check deferred? | Closed — none. |
| Q17 | Persona disposition | Are Personas recorded for the visible surface? | Closed — agent, developer, and maintainer are named with their existing authority. |
| Q18 | ADR qualification | Does the documentation move require an ADR? | Closed — no. The recorded qualification routes to the use cases, design, and chapter 08. |
| Q19 | Architecture documentation | Do all 12 chapters remain real, and do the affected chapters match the design? | Closed — chapters 05, 06, 08, 11, and 12 were updated; all 12 files exist and contain real content. |
| Q20 | Ubiquitous language | Are the new terms recorded and used consistently? | Closed — domain model, standalone glossary, documentation scaffold, and thin map are defined in the design and will be recorded in `docs/glossary.md`. |
| Q21 | Scope | Does the design silently add migration tooling? | Closed — no CLI content transformation; migration is agent-directed. |
| Q22 | Scope | Does the design silently require a Mermaid renderer? | Closed — no; ELK is requested, the limitation is stated, and a rendered review is manual. |

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `specs/fragments/guide-domain-modeling.feature` | Added the boundaries/relationships template scaffold scenario. |
| `specs/initialize-project/enable-optional-addons.feature` | Added chapter 12 link/table assertions and the disable-leaves-documents scenario. |
| `design.md` | Split contracts so each has exactly one implementing component; aligned the interaction matrix capability names; named the new scenario results. |
| `docs/arc42/05-building-block-view.md` | Updated `cli::addons`, `cli::instructions`, and `skills::targets` responsibilities. |
| `docs/arc42/06-runtime-view.md` | Updated init, instruction routing, and addon-enable runtime behavior. |
| `docs/arc42/08-crosscutting-concepts.md` | Added 8.27 and updated 8.11. |
| `docs/arc42/11-risks-and-technical-debt.md` | Added R-127 for the migration two-location interval. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in the design's `## Decisions` ledger or is carried with a named owner.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change standalone-domain-model` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
