# Arc42 Assessment: semantic-reranker-budget

| # | Chapter | Affected | Assessment |
|---:|---|---|---|
| 1 | Introduction and Goals | Yes | Adds the operational goal of configurable second-stage work budgets without changing E5 recall boundaries. |
| 2 | Architecture Constraints | Yes | Replaces the fixed BGE/NLI pair refusal with a positive runtime budget whose default is 2048. |
| 3 | Context and Scope | Yes | Extends the retrieval CLI's BGE/NLI output contract with the active budget and explicit unscored tails. |
| 4 | Solution Strategy | Yes | Uses deterministic E5-score ordering and prefix selection before model-tokenizer preflight. |
| 5 | Building Block View | Yes | Adds shared budget parsing and ordered-prefix selection inside `spec::retrieval`; no new production module is needed. |
| 6 | Runtime View | Yes | Changes duplicate, Explore, advisory-review, and blocking-review flows from overflow refusal to partial scoring. |
| 7 | Deployment View | No | No packaging, deployment topology, service, or model-artifact distribution change. |
| 8 | Crosscutting Concepts | Yes | Defines configuration validation, unscored evidence, token-cap precedence, and independent BGE/NLI selection. |
| 9 | Architecture Decisions | Yes | Adds the decision to use a configurable ordered budget and advances the Rust pipeline attestation. |
| 10 | Quality Requirements | Yes | Adds budget-boundary, ordering, partial-scoring, and runtime-accounting requirements. |
| 11 | Risks and Technical Debt | Yes | Replaces the old overflow-refusal mitigation with an explicit CPU-budget and recall-tail risk. |
| 12 | Glossary | Yes | Defines the second-stage pair budget and clarifies `BGE_UNSCORED`/`NLI_UNSCORED` for budget tails. |

## Updated Durable Chapters

- `docs/arc42/README.md`
- `docs/arc42/01-introduction-and-goals.md`
- `docs/arc42/02-architecture-constraints.md`
- `docs/arc42/03-context-and-scope.md`
- `docs/arc42/04-solution-strategy.md`
- `docs/arc42/05-building-block-view.md`
- `docs/arc42/06-runtime-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/09-architecture-decisions.md`
- `docs/arc42/10-quality-requirements.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`

## Status

STATUS: DONE
