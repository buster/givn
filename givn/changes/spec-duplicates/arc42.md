# Arc42 Assessment: spec-duplicates

| # | Chapter | Affected | Reason and change summary |
|---:|---|:---:|---|
| 1 | Introduction and Goals | Yes | Adds FR-32 and complete permanent-corpus visibility with a deterministic text/JSON contract as a secondary quality goal. |
| 2 | Architecture Constraints | Yes | Defines the permanent-spec, active-change, archive, and index read-only boundary, with model-cache creation as the only permitted retrieval side effect. |
| 3 | Context and Scope | Yes | Adds the `givn spec duplicates` CLI contract, fixed output surfaces, permanent-only evidence scope, and path-specific input failures. |
| 4 | Solution Strategy | Yes | Documents the deterministic exact/Fingerprint structural precedence, advisory E5 threshold candidates, in-memory pair scoring, and stable renderers. |
| 5 | Building Block View | Yes | Extends `cli::spec` and `spec::retrieval` responsibilities and records the acceptance modules, score seam, and output model. |
| 6 | Runtime View | Yes | Adds the complete permanent-pair audit flow, empty/malformed input behavior, threshold admission, and deterministic text/JSON rendering. |
| 7 | Deployment View | No | No runtime service, deployment artifact, platform target, or external dependency changes. |
| 8 | Cross-cutting Concepts | Yes | Defines the minimal Fingerprint structural vocabulary, fixed output grammar, score/null rules, read-only boundary, and advisory semantic evidence. |
| 9 | Architecture Decisions | Yes | Completes ADR-049 in the repository MADR format with alternatives, option-by-option pros/cons, consulted/informed metadata, and confirmation. |
| 10 | Quality Requirements | Yes | Strengthens Q-75/Q-76 with exact identities, ranks, facts, output keys, threshold boundaries, and non-mutation; adds measurable model-quality evidence. |
| 11 | Risks and Technical Debt | Yes | Adds measurable quadratic-scan limits and model recall/precision/advisory-risk mitigation. |
| 12 | Glossary | Yes | Defines the permanent audit contract, direction-specific subset relations, fingerprint evidence, and advisory semantic candidate terminology. |

## Status

STATUS: DONE
