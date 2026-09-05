# arc42 Impact Assessment: grouped-merge-for-new-capabilities

The change alters one archive-merge decision: a new capability's declared
group now determines the merge target. No new component, interface, or
runtime flow; the routing decision itself stays proposal-time.

ADR qualification: the candidate (merge consumes the recorded routing
decision instead of the flat fallback) fails the gate — the complete
rationale is owned by this change's design and the archival group's
permanent spec (lower_level_artifact fails); reversal is a one-branch change
(durable_consequence fails). Routing: `CANONICAL_ARTIFACT` (design.md +
spec delta).

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals / Quality Requirements | No | Lifecycle goals unchanged; the merge becomes consistent with the already-documented group model. |
| 2 | Architecture Constraints | No | No constraint changes. |
| 3 | Context and Scope | No | No new user-facing surface; the archive command's observable contract gains correctness, not surface. |
| 4 | Solution Strategy | No | Filesystem-derived state and delta merge strategy unchanged. |
| 5 | Building-Block View | No | `cli::archive` keeps its responsibility ("Merge (or retire) delta features into givn/specs/"); the target resolution inside it now honors the declared group — same block, corrected behavior. |
| 6 | Runtime View | Yes | 6.7 archive stage 7 documents the merge target as `givn/specs/<group>/<capability>.feature` "(the permanent corpus is grouped by use case, one `group.md` per group)" — the flat fallback for new capabilities with a declared group contradicts that text; one clarifying sentence added. |
| 7 | Deployment View | No | No deployment change. |
| 8 | Crosscutting Concepts | No | No cross-cutting concept touched. |
| 9 | Architecture Decisions | No | No decision passes the five-dimension gate (see qualification note). |
| 10 | Quality Requirements | No | No new quality requirement. |
| 11 | Risks and Technical Debt | No | The defect class (merge/gate divergence) is fixed, not introduced; no new risk line. |
| 12 | Glossary | No | No new domain term; the fix reuses Group, Capability, and the routing vocabulary already recorded. |

## Status

STATUS: DONE