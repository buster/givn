# Arc42 Impact: nli-evidence

## Chapter Assessment

| # | Chapter | Affected | Summary |
|---:|---|---|---|
| 1 | Introduction and goals | Yes | Adds the NLI candidate-reduction requirement and semantic-review integrity goal. |
| 2 | Architecture constraints | Yes | Adds the independent feature, shared bounded pool, fixed admission point, and non-blocking boundary. |
| 3 | Context and scope | Yes | Extends the CLI evidence interfaces and removes NLI from the deferred scope. |
| 4 | Solution strategy | Yes | Extends retrieve-then-rerank strategy and combined admission behavior. |
| 5 | Building block view | Yes | Extends `spec::retrieval` and repository-owned acceptance modules with NLI evidence. |
| 6 | Runtime view | Yes | Documents NLI scoring, unavailable fallback, and BGE/NLI intersection flows. |
| 7 | Deployment view | Yes | Documents the project-local NLI artifact cache and no-service deployment boundary. |
| 8 | Cross-cutting concepts | Yes | Documents NLI probabilities, statuses, schema additions, bounds, and error semantics. |
| 9 | Architecture decisions | Yes | Adds ADR-058 for independent NLI evidence on the shared bounded E5 pool. |
| 10 | Quality requirements | Yes | Adds NLI reduction, symmetry, boundary, compatibility, and bounded-runtime scenarios. |
| 11 | Risks and technical debt | Yes | Adds false-negative, calibration, pool-parity, and local-artifact risks. |
| 12 | Glossary | Yes | Adds NLI evidence/status terms and the shared bounded second-stage pool. |

## Status

STATUS: DONE
