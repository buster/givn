# arc42 Documentation Update: spec-explore-permanent

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Adds a user-facing permanent-scenario exploration capability and deterministic authoring goal. | Added FR-26 and the non-mutating exploration quality goal. |
| 2 | 02 architecture-constraints | No | The change uses the existing Rust, Gherkin, Clap, feature-gated retrieval, and local-first constraints. | |
| 3 | 03 context-and-scope | Yes | Adds two CLI input forms and a stable JSON exploration interface. | Added the explore interface, scope boundary, and later-layer non-goal. |
| 4 | 04 solution-strategy | Yes | Chooses embedding-only advisory exploration before the existing reranker review path. | Clarified the semantic retrieval strategy and model boundary. |
| 5 | 05 building-block-view | Yes | Extends CLI/retrieval and adds capability-specific black-box step modules. | Documented explore responsibilities and test building blocks. |
| 6 | 06 runtime-view | Yes | Introduces file/stdin validation, refresh, rank, identity filtering, and rendering flow. | Added the permanent exploration runtime flow. |
| 7 | 07 deployment-view | No | No service, port, runtime, or distribution artifact changes. | |
| 8 | 08 crosscutting-concepts | Yes | Adds input validation, deterministic output, content-hash reuse, and read-only semantic boundaries. | Added the permanent exploration cross-cutting contract. |
| 9 | 09 architecture-decisions | Yes | The complete-scenario, permanent-only, exact-identity decision is new. | Added ADR-043. |
| 10 | 10 quality-requirements | Yes | The new CLI contract requires explicit quality scenarios for parity, filtering, reuse, and no-feature behavior. | Added Q-42 through Q-45. |
| 11 | 11 risks-and-technical-debt | Yes | Path identity and JSON diagnostic contamination are new implementation risks. | Added R-30 and R-31. |
| 12 | 12 glossary | Yes | Explore query, exact identity, and embedding score are new domain terms. | Added the three terms. |

## Notes

The permanent retrieval documentation remains distinct from later change-aware
exploration, semantic classification, remediation, and full audit layers.

## Status

STATUS: DONE
