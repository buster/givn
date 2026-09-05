# arc42 Impact Assessment: no-flat-fallback-for-new-capabilities

Tightens one archive-merge branch from silent fallback to refusal. No new
component, interface, or flow.

ADR qualification: fails the gate — complete rationale owned by design.md +
the permanent spec (lower_level_artifact); reversal is one branch
(durable_consequence). Routing: `CANONICAL_ARTIFACT`.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals / Quality Requirements | No | Goals unchanged; correctness of an existing flow. |
| 2 | Architecture Constraints | No | No constraint changes. |
| 3 | Context and Scope | No | No new user-facing surface; the archive contract gains a blocking case. |
| 4 | Solution Strategy | No | Filesystem-derived state strategy unchanged. |
| 5 | Building-Block View | No | `cli::archive` responsibility unchanged; one branch now fails closed. |
| 6 | Runtime View | Yes | 6.7 stage 7 sentence says the flat layout "remains only for capabilities with no recorded decision" — now the archive blocks in that case; sentence updated. |
| 7 | Deployment View | No | No deployment change. |
| 8 | Crosscutting Concepts | No | No cross-cutting concept touched. |
| 9 | Architecture Decisions | No | No decision passes the gate. |
| 10 | Quality Requirements | No | No new quality requirement. |
| 11 | Risks and Technical Debt | No | Closes a silent-degradation path (related to R-46's class); no new risk. |
| 12 | Glossary | No | No new domain term. |

## Status

STATUS: DONE