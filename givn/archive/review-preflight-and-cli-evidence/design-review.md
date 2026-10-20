# Design review: review-preflight-and-cli-evidence

| Branch | Finding |
|---|---|
| Scope | The strict per-scenario commit rule, early blocking check, README preflight, CLI not-applicable review, and transcript evidence are covered. No one-pass mode. |
| Tech choices | SCM seam inspects existence, ancestry, and message; one parser in `check`. No new dependency or marker. |
| Missing scenarios | Missing, duplicate, untitled, and valid commit lists; visual not-applicable; CLI transcript evidence. |
| Testability | Every scenario asserts a concrete exit, stderr line, or stdout line; fixtures create real commits. |
| E2E fidelity | The check runs through the real CLI; no new E2E action. |
| Interaction coverage | Variants of check tasks and archive; no new inventory row. |
| Risk | The rule is strict; a one-pass implementation must split commits before check tasks. That is the intended cost. |
| Use case and Persona | verify-change and archive-change updated; no new Persona. |
| Decision ledger | D1-D3 resolved; D4's blocking boundary recorded. |
| Visual contract | `Interface: none`. |
| ADR | ADR-070/072 checked; canonical destination `design.md`. |
| Arc42 | 12 rows assessed; affected chapters updated; `arc42.md` matches. |
| Ubiquitous language | "Scenario revision" and "Interaction transcript" added. |

DESIGN-REVIEW: PASS
