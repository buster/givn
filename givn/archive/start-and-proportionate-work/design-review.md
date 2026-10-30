# Design Review: start-and-proportionate-work

Review kind: self-review
Author: Maki
Reviewer context: Maki

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| Q1 | Scope | Do the specs cover all four outcomes, proportionality, direct-work obligations, authorization, and handoff? | Closed — the start instruction scenarios cover all four. |
| Q2 | Scope | Is the no-active routing decision expressed in both text and JSON? | Closed — text, JSON, and the direct alternative are separate scenarios. |
| Q3 | Scope | Does the plan remove explore without adding an alias? | Closed — the start instruction replaces it; a retired-id refusal is explicit, including a project override. |
| Q4 | Scope | Does the plan cover old generated output in existing projects? | Closed — fresh init, recognizable skill removal, custom preservation, legacy command removal, upgrade staging, and override notice are covered. |
| Q5 | Tech choices | Is a new command or runtime classifier required? | Closed — no. The work reuses the asset cascade, skill generation, and SCM staging seam. |
| Q6 | Missing scenarios | Is the upgrade path testable and covered? | Closed after hardening — a committed project with the versioned rules policy exercises tracked removal through `givn upgrade`. |
| Q7 | Missing scenarios | Is a project-owned explore override covered? | Closed after hardening — the override is preserved and `givn instructions explore` still refuses before the cascade. |
| Q8 | Testability | Can each new scenario fail in RED without production code? | Closed — missing templates, missing JSON fields, and retained explore output fail before implementation. |
| Q9 | Testability | Are strict mode and the single-scenario command named? | Closed — `.fail_on_skipped()` and the `GIVN_FEATURES=... --name` command are in the design. |
| Q10 | E2E fidelity | Does the change invent a new E2E action? | Closed — the existing startup scenario is modified; the transcript is the terminal evidence. |
| Q11 | Interaction Coverage | Does every inventory entry touched by the change appear in the matrix? | Closed after hardening — serve-extended-rules, instruction-integrity, refresh-generated-guidance, and initialize-a-project are mapped; the ideation feature's pre-existing missing inventory row is not invented in this change. |
| Q12 | Risk | What is the most likely implementation failure? | Closed — deleting a custom same-named explore skill, or failing to stage a tracked retirement. Recognition requires generated frontmatter and the served-instruction pointer; upgrade stages tracked retired paths. |
| Q13 | Risk | Can the no-active output still imply mandatory change creation? | Closed — start is the recommendation, `givn new` is explicitly optional, and JSON carries both fields. |
| Q14 | Use-case and Persona context | Are the owning use cases and confirmed Personas preserved? | Closed — obtain-effective-guidance owns the guidance; initialize-project and inspect-change-state are supporting owners; the three confirmed Personas are review lenses. |
| Q15 | Decision ledger | Does every `D<n>` resolve? | Closed — D1 is user-answered; D2–D4 are answered by the improvement verification expectations and scope. |
| Q16 | Contracts and components | Does one component implement one contract? | Closed after hardening — effective guidance, generated guidance, project initialization, and managed upgrade are separate contracts with one implementing component each. |
| Q17 | Decision timing | Is any foundational question carried? | Closed — none. |
| Q18 | Visual design contract | Is the interface classification correct? | Closed — terminal text and Markdown; no browser-rendered screen. |
| Q19 | Deferrals | Is any mandatory check deferred? | Closed — none. |
| Q20 | Persona disposition | Are Personas recorded for the visible surface? | Closed — agent, developer, and maintainer. |
| Q21 | ADR qualification | Does the entry-policy change require an ADR? | Closed — no. The recorded qualification routes to the use cases, design, and chapter 08. |
| Q22 | Architecture documentation | Do all 12 chapters remain real, and do affected chapters match? | Closed — chapters 3, 4, 5, 6, 8, 11, and 12 were updated; all 12 files exist and contain real content. |
| Q23 | Ubiquitous language | Are Start, Direct work, and Proportional routing recorded? | Closed — added to `docs/glossary.md` and used consistently. |
| Q24 | Scope | Does the plan silently create a lightweight lifecycle? | Closed — direct work has no proposal, design, tasks, or archive artifact and no gate. |
| Q25 | Scope | Does the plan weaken existing change gates? | Closed — gates for an active formal change are unchanged; only state reporting gains a scope qualification. |
| Q26 | Testability | Does the plan promise that string presence proves routing judgment? | Closed — it explicitly does not; representative conversations remain manual review evidence. |
| Q27 | Missing scenarios | Is archive recovery still truthful with no active change? | Closed — the existing unproven-candidate branch stays `givn archive`; the start recommendation applies only when no candidate needs recovery. |

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `specs/obtain-effective-guidance/serve-extended-rules.feature` | Added a project-owned explore override refusal scenario. |
| `specs/fragments/refresh-generated-guidance.feature` | Added the versioned-policy upgrade scenario and the project-owned explore override notice scenario. |
| `design.md` | Added the retired instruction refusal, override notice, upgrade staging, and corrected contract/matrix rows. |
| `docs/arc42/03-context-and-scope.md` | Updated the `givn instructions` interface. |
| `docs/arc42/04-solution-strategy.md` | Added proportionate entry to the lifecycle and guidance strategy. |
| `docs/arc42/05-building-block-view.md` | Updated init, instructions, and skill-generation responsibilities. |
| `docs/arc42/06-runtime-view.md` | Updated startup guidance and skill synchronization. |
| `docs/arc42/08-crosscutting-concepts.md` | Added 8.28 and updated the skill-generation section. |
| `docs/arc42/11-risks-and-technical-debt.md` | Added R-128 and R-129. |
| `docs/glossary.md` | Added Start, Direct work, and Proportional routing. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in the design's `## Decisions` ledger or is carried with a named owner.
- [x] All questions are resolved.
- [x] `design.md` reflects the decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change start-and-proportionate-work` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
