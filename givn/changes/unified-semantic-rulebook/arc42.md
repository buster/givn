# Arc42 impact assessment: unified-semantic-rulebook

| Chapter | Affected | Reason |
|---|---|---|
| 01 Introduction & Goals | No | No new goal; runtime-cost reduction serves existing FR quality gates. |
| 02 Constraints | No | No new technical, legal, or organisational constraint. |
| 03 Context & Scope | No | No new user-facing command; existing commands keep their contracts. |
| 04 Solution Strategy | Yes | Strategy rows `Semantic strictness`, `BGE/NLI`, and the rulebook/budget relationship rewritten (single 0.90 admission, hybrid selection, cache). |
| 05 Building Block View | Yes | `spec::retrieval` description gains the unified rulebook; persistent boundaries gain `.spec-pair-cache.json`. |
| 06 Runtime View | Yes | Advisory review (6.8.4), archive stage 4 (6.7), and the blocking report flow (6.9) now describe the rulebook sequence. |
| 07 Deployment View | No | No deployment boundary changes; cache file is project-local state like the index. |
| 08 Cross-Cutting Concepts | Yes | 8.5d report flow, classification table boundary values, and the unscored-vs-unselected distinction updated. |
| 09 Architecture Decisions | Yes | New ADR-066 (unified rulebook + pair-score cache); register and ADR index updated. |
| 10 Quality Requirements | Yes | Remediation boundary values updated from `0.9500` to `0.9000`. |
| 11 Risks & Technical Debt | Yes | R-13 reworded to the selection-boundary recall position; R-21 rewritten from budget tail to selection tail. |
| 12 Glossary | Yes | New terms: Unified rulebook, Pair cap, Neighbour count, Pair-score cache, Unselected pair; E5 candidate redefined. |

STATUS: DONE
