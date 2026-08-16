# arc42 Documentation Update: spec-explore-change

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The change adds a user-facing current-change exploration capability and extends the authoring ergonomics goal. | Added FR-27 and broadened the deterministic, advisory, non-mutating exploration goal. |
| 2 | 02 architecture-constraints | No | The capability reuses the existing Rust, Gherkin, CLI, local retrieval, feature-gated, and project-local constraints; it introduces no new fixed legal, technical, or organisational constraint. | |
| 3 | 03 context-and-scope | Yes | The CLI interface gains an optional current-change scope and source-labeled text/JSON results. | Documented `--change`, request-local active candidates, and the later-layer classification boundary. |
| 4 | 04 solution-strategy | Yes | The retrieval strategy now combines permanent candidates with transient active-delta candidates without changing the persistent catalogue. | Added the request-local current-change strategy and additive source labeling. |
| 5 | 05 building-block-view | Yes | Retrieval and CLI responsibilities expand to validate/collect deltas and normalize all exploration identities. | Documented request-local collection, lifecycle filtering, the shared path helper, and result metadata. |
| 6 | 06 runtime-view | Yes | The command has new validation, delta collection, identity filtering, ranking, and rejection-before-refresh flows. | Added current-change runtime sequence, source/target self-filtering, and no-persistence-on-rejection flow. |
| 7 | 07 deployment-view | No | No service, port, runtime, installation, distribution, or deployment artifact changes. | |
| 8 | 08 crosscutting-concepts | Yes | Path identity normalization, lifecycle filtering, additive output metadata, and rejected-request persistence behavior cross-cut the feature. | Added the current-change exploration contract, one canonical path rule, and pre-refresh rejection rule. |
| 9 | 09 architecture-decisions | Yes | The request-local corpus, additive metadata contract, and single path-identity rule are new architecture decisions. | Added ADR-044 and ADR-045 with alternatives, consequences, confirmation, and source links. |
| 10 | 10 quality-requirements | Yes | The feature adds observable quality boundaries for active candidates, identity filtering, path-form parity, metadata, stdin parity, and rejected requests. | Updated Q-42 and added Q-46 through Q-51. |
| 11 | 11 risks-and-technical-debt | Yes | Shared identity normalization, no-persistence-on-rejection, and additive JSON metadata introduce explicit failure risks. | Expanded R-30 and added R-32 and R-33 with mitigations and executable coverage. |
| 12 | 12 glossary | Yes | Current-change candidate, request-local corpus, projected target identity, source metadata, and project-relative path identity are new terms. | Added the current-change exploration and identity vocabulary. |

## Notes

All twelve durable chapter files remain substantive. The change is limited to
PR2 advisory exploration: it does not add semantic classification, remediation,
BGE/NLI evidence, archive projection, or review gating. The two e2e inventory
entries remain the distinct terminal-text and machine-readable-JSON consumer
interactions; `--change --stdin` is covered as regular behavior only.

## Status

STATUS: DONE
