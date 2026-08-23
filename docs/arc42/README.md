# givn - Architecture Documentation (arc42)

This is the durable architecture documentation for `givn`, organized according
to the [arc42](https://arc42.org) template. Each chapter is a separate file.
Chapter 9 is a register whose decisions are stored as individual MADR files.

## Documentation Status

The checkout contains the core Rust CLI, lifecycle, embedded assets, local
retrieval, semantic review, duplicate audit, BGE/NLI evidence paths, and the
repository-owned acceptance fixtures. Distribution through release binaries,
Homebrew, and the shell installer remains planned work.

The active change `semantic-reranker-budget` is not complete. `givn status
--change semantic-reranker-budget` reports open implementation tasks and final
verification. Therefore its ADR-063 is `proposed`, not an accepted current
architecture contract.

Only accepted, non-superseded ADRs are current architecture. See the [chapter 9
register](adr/README.md) for status, relationships, evidence, reserved IDs, and
historical records.

## Current Semantic Surface

The retrieval feature is opt-in at build time. E5 uses one compact deterministic
serialization of the complete parsed Gherkin context and hashes that exact text
for vector reuse. The actual tokenizer preflights E5 passages and selected
BGE/NLI pairs without truncation or padding; `512` fits and `513` is over.
Fitting E5 passages are indexed, while over-cap passages are visible as
classification-free `E5_TOKEN_CAP` findings.

The second-stage pools are surface-dependent:

| Surface | BGE | NLI |
|---|---|---|
| `spec duplicates` | Endpoint top-five union, then E5-ranked budget | Full E5 semantic candidate pool, independently budgeted |
| `spec explore` | At most the five ranked Explore results | The same at-most-five Explore result set, independently scored |
| advisory `spec review` | The surface's retrieved candidate pool, then budget | The same surface pool, independently scored |
| blocking `check review` | Complete semantic-report candidates, then budget | The same report candidates, independently budgeted |

BGE and NLI never assign a Givn classification. `*_FILTERED` evidence is
non-blocking; budget tails are `*_UNSCORED`; selected token-cap evidence remains
visible and blocking. Combined recommendations intersect only when both layers
participate.

`givn spec duplicates` reads the persistent E5 index to reuse entries whose
identity and complete content hash still match. It does not refresh or write the
persistent index as part of the audit, and computes missing audit embeddings in
memory. It excludes active changes and archive contents.

The current PR #27 real-model report records `394` permanent scenarios, `1274`
E5 candidates, and `795` BGE top-five-union pairs. The report relies on local,
ephemeral raw JSON reports. Older measurements such as `288/414?`, `385/100`,
and `397/1311/803` are historical snapshots and are not current baselines.

## Chapters

| # | Chapter | Purpose |
|---|---|---|
| 1 | [Introduction and Goals](01-introduction-and-goals.md) | Product purpose, requirements, quality goals, stakeholders |
| 2 | [Architecture Constraints](02-architecture-constraints.md) | Technical, organizational, and repository constraints |
| 3 | [Context and Scope](03-context-and-scope.md) | System boundary, interfaces, and scope |
| 4 | [Solution Strategy](04-solution-strategy.md) | Strategic architecture choices |
| 5 | [Building Block View](05-building-block-view.md) | Crate, modules, and responsibilities |
| 6 | [Runtime View](06-runtime-view.md) | Lifecycle, gates, retrieval, audit, and archive flows |
| 7 | [Deployment View](07-deployment-view.md) | Binary, feature builds, caches, and planned distribution |
| 8 | [Cross-cutting Concepts](08-crosscutting-concepts.md) | Cascade, manifest, Gherkin, hooks, skills, coverage, and retrieval |
| 9 | [Architecture Decisions](09-architecture-decisions.md) | Pointer to the [ADR register](adr/README.md) and individual records |
| 10 | [Quality Requirements](10-quality-requirements.md) | Quality tree, scenarios, and verification boundaries |
| 11 | [Risks and Technical Debt](11-risks-and-technical-debt.md) | Known risks, limitations, and historical measurements |
| 12 | [Glossary](12-glossary.md) | Domain and technical vocabulary |

## Related Documents

- [`../../README.md`](../../README.md) - user-facing project overview.
- [`../TASKS.md`](../TASKS.md) - project implementation task breakdown.
- [`../../givn/changes/semantic-reranker-budget/tasks.md`](../../givn/changes/semantic-reranker-budget/tasks.md) - open current-change verification status.
- [`../migrations/0.2.0-to-0.3.0.md`](../migrations/0.2.0-to-0.3.0.md) - LLM prompt pack for the 0.3.0 project migration.

## Reading Order

1. Read chapters 1 and 4 for purpose and strategy.
2. Read chapter 8 for cross-cutting mechanics and chapter 6 for runtime order.
3. Read the [ADR register](adr/README.md) and only then individual decisions.
4. Treat proposed, deprecated, and superseded records as historical or pending evidence, not as accepted architecture.
