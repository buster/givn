# arc42 Impact Assessment: enforce-spec-similarity-gate

This change supersedes the advisory-only retrieval review behavior with a
strict, binary-fixed semantic uniqueness gate. It also requires the current
Givn permanent specification tree to be dogfooded and cleaned at the same
threshold before the change is complete.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals / Quality Requirements | Yes | Retrieval review becomes a strict semantic-uniqueness quality requirement while interactive search remains advisory. |
| 2 | Architecture Constraints | Yes | The feature-enabled binary has a fixed `0.95` threshold with no project, CLI, or environment override. |
| 3 | Context and Scope | Yes | Review and archive behavior now exposes strict permanent and delta similarity auditing through the CLI. |
| 4 | Solution Strategy | Yes | The strategy changes from advisory-only retrieval review to projected-tree pair auditing and remediation below the threshold. |
| 5 | Building Block View | Yes | `spec::retrieval`, `cli::check`, and `cli::archive` gain pair scanning, strict failure, and post-archive refresh responsibilities. |
| 6 | Runtime View | Yes | Review now computes delta-to-baseline and projected permanent flows; archive refreshes the index after successful publication. |
| 7 | Deployment View | Yes | The feature-enabled local model/index footprint and the no-feature no-op behavior are clarified; deployment topology is otherwise unchanged. |
| 8 | Crosscutting Concepts | Yes | Threshold immutability, projection, pair identity, remediation, feature gating, and failure semantics are cross-cutting policies. |
| 9 | Architecture Decisions | Yes | ADR-041 supersedes the advisory review portion of ADR-039 and records the fixed-threshold gate. |
| 10 | Quality Requirements | Yes | Same-file/cross-file uniqueness, override resistance, projected-tree cleanup, and archive refresh become quality scenarios. |
| 11 | Risks and Technical Debt | Yes | Existing high-score pairs, model revision drift, false positives, and model availability become strict-gate risks. |
| 12 | Glossary | Yes | Similarity candidate and the `MERGE`/`KEEP-AS-BOUNDARY` remediation terms require precise definitions. |

## Status

STATUS: DONE
