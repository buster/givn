# givn - Architecture Documentation (arc42)

This is the durable architecture documentation for `givn`, organized according
to the [arc42](https://arc42.org) template. Each chapter is a separate file.
Chapter 9 is a register whose decisions are stored as individual MADR files.

## Documentation Status

The checkout contains the core Rust CLI, lifecycle, embedded assets, the
deterministic specification toolchain (`spec find`, `spec tree`, `spec
duplicates`, `spec progress`, `spec coverage`, `spec route`), and the
repository-owned acceptance fixtures. Distribution through release binaries,
Homebrew, and the shell installer remains planned work.

The `drop-embedding-features` change removed the semantic retrieval engine;
ADR-069 records the single feature-free binary and supersedes the engine
records. The `run-command` change records ADR-067 (one tool declares and
executes the product start command) and amends ADR-028 with the
run-declaration gate; the durable run-declaration facts live in chapters 3,
6.15, 8.16, and the glossary.

Only accepted, non-superseded ADRs are current architecture. See the [chapter 9
register](adr/README.md) for status, relationships, evidence, reserved IDs, and
historical records.

## Chapters

| # | Chapter | Purpose |
|---|---|---|
| 1 | [Introduction and Goals](01-introduction-and-goals.md) | Product purpose, requirements, quality goals, stakeholders |
| 2 | [Architecture Constraints](02-architecture-constraints.md) | Technical, organizational, and repository constraints |
| 3 | [Context and Scope](03-context-and-scope.md) | System boundary, interfaces, and scope |
| 4 | [Solution Strategy](04-solution-strategy.md) | Strategic architecture choices |
| 5 | [Building Block View](05-building-block-view.md) | Crate, modules, and responsibilities |
| 6 | [Runtime View](06-runtime-view.md) | Lifecycle, gates, and archive flows |
| 7 | [Deployment View](07-deployment-view.md) | Binary and planned distribution |
| 8 | [Cross-cutting Concepts](08-crosscutting-concepts.md) | Cascade, manifest, Gherkin, hooks, skills, and coverage |
| 9 | [Architecture Decisions](09-architecture-decisions.md) | Pointer to the [ADR register](adr/README.md) and individual records |
| 10 | [Quality Requirements](10-quality-requirements.md) | Quality tree, scenarios, and verification boundaries |
| 11 | [Risks and Technical Debt](11-risks-and-technical-debt.md) | Known risks, limitations, and historical measurements |
| 12 | [Glossary](12-glossary.md) | Domain and technical vocabulary |

## Related Documents

- [`../../README.md`](../../README.md) - user-facing project overview.
- [`../TASKS.md`](../TASKS.md) - project implementation task breakdown.
- [`../migrations/0.2.0-to-0.3.0.md`](../migrations/0.2.0-to-0.3.0.md) - LLM prompt pack for the 0.3.0 project migration.
- [`../migrations/README.md`](../migrations/README.md) - release-owned migration bundle and aggregate-plan contract.

## Reading Order

1. Read chapters 1 and 4 for purpose and strategy.
2. Read chapter 8 for cross-cutting mechanics and chapter 6 for runtime order.
3. Read the [ADR register](adr/README.md) and only then individual decisions.
4. Treat proposed, deprecated, and superseded records as historical or pending evidence, not as accepted architecture.
