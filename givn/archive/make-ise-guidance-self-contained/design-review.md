# Design review: make-ise-guidance-self-contained

Review kind: self-review
Author: Main (omp session)
Reviewer context: Main (omp session)

The author performed the design review and the final review in the same context.

## Grilling

| Branch | Finding | Resolution |
|---|---|---|
| Scope | The review named two gaps: the served `docs/reference.md` delegation and the missing off-path sweep. | The delta has exactly two scenarios; no other behavior changes. |
| Tech choices | Inline definitions vs scaffolding `docs/reference.md`. | Inline; the instruction is the consumer surface and a scaffold would add a second copy of maintainer prose. D1. |
| Missing scenarios | An ise-enabled reference check and an ise-off sweep. | Both added. The sweep includes the startup guidance. D2, D3. |
| Testability | Can the reference scenario fail for the claimed reason? | Verified: with the old delegation restored, `givn instructions ise:16 — docs/reference.md` is reported and the scenario fails; with the inline definitions it passes. Strict mode `.fail_on_skipped()`, the single-scenario command, and the stub rule are in the design. |
| E2E fidelity | No new `@e2e`; the real interface is the CLI text. | The existing `instruction-integrity` smoke remains; the new scenarios are regular. |
| Interaction coverage | The two scenarios are variants of two existing `instruction-integrity` entries. | Matrix rows present; the remaining 44 rows are unchanged. No new inventory row. |
| Risk | Duplicate definitions could drift. | The inline definitions are short; `docs/reference.md` remains the detailed owner. |
| Use-case and Persona | The support fragment owns the capability; three Personas are dispositioned. | No invented Persona; no Gherkin actor change. |
| Decision ledger | D1–D4 resolve the proposal; D5 records this change's own `No rebuild`. | Complete. |
| Contracts and components | No Component or contract changes. | The design states none. |
| Decision timing | No carried decisions. | Not applicable. |
| Ise rebuild decision | The change is `No rebuild`. | D5, no `Rebuilt code` line. |
| Visual design contract | `Interface: none`; terminal text only. | Persona disposition present. |
| ADR qualification | No architectural commitment, no real alternative, no durable consequence. | `ADR action: NONE`. |
| Arc42 | All 12 rows are unaffected; the change is served text and tests. | Table complete in `arc42.md`. |
| Ubiquitous language | The glossary is unchanged; the instruction defines its own terms inline. | No glossary drift. |
| Structural model | `docs/domain-model.md` already carries the terms; no structural change. | No edit. |

No blocking findings. The two findings from the isolation review are resolved by the design and its RED proof.

DESIGN-REVIEW: PASS
