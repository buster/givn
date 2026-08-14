# arc42 Impact Assessment: skill-rewrites

This change rewrites embedded guidance and generated workflow choreography. It
does not add runtime components, interfaces, deployment behavior, or new
architecture decisions; the canonical source and architecture decision were
established by the preceding prose-consolidation change.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals / Quality Requirements | No | Existing agent-guidance quality goals remain unchanged. |
| 2 | Architecture Constraints | No | No legal, technical, organizational, or platform constraint changes. |
| 3 | Context and Scope | No | No new external interface or user-facing command is introduced. |
| 4 | Solution Strategy | No | The existing canonical-source and cascade strategy is reused. |
| 5 | Building-Block View | No | Existing asset, renderer, and target blocks remain; only their prose changes. |
| 6 | Runtime View | No | `givn skills sync` and instruction resolution flows are unchanged. |
| 7 | Deployment View | No | No deployment or environment change. |
| 8 | Crosscutting Concepts | No | The canonical policy and override concept were established in Change 3; this change supplies gate-specific choreography. |
| 9 | Architecture Decisions | No | No new trade-off; ADR-038 remains the governing single-source decision. |
| 10 | Quality Requirements | No | Scenarios verify guidance content but introduce no system quality requirement. |
| 11 | Risks and Technical Debt | No | No new risk beyond R-17 already recorded for policy drift. |
| 12 | Glossary | No | No new domain term. |

## Status

STATUS: DONE
