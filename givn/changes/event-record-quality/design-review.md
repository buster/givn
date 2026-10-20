# Design review: event-record-quality

| Branch | Finding |
|---|---|
| Scope | The four ambiguities are addressed additively; no schema break, no analyzer rule change. |
| Tech choices | Derived marker plus parent link; optional answer fields; bounded first-line error; provider/variant. Simpler than omitting linked usage or storing full stderr. |
| Missing scenarios | One scenario covers usage, answer, error, and identity; unanswered silence is covered by the absence rule. |
| Testability | The scenario reads the real record after the real intake; each Then asserts a concrete field. |
| E2E fidelity | No new E2E action; the existing intake is the real boundary. |
| Interaction coverage | Existing intake interaction; no new inventory row. |
| Risk | Derived double counting is explicit; secret leakage is bounded by the existing first-line path. |
| Use case and Persona | The inspect-development-history contract gains the record rules; no new Persona. |
| Decision ledger | D1-D3 resolved; D4 is additive identity. |
| Visual contract | `Interface: none`. |
| ADR | ADR-073 checked; canonical destination `design.md`; no amendment. |
| Arc42 | 12 rows assessed; affected chapters updated; `arc42.md` matches. |
| Ubiquitous language | Derived usage, answered question, bounded command error added. |

DESIGN-REVIEW: PASS
