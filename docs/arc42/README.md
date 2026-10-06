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

The `archive-proof` change proposes ADR-070: archive publication becomes
forward-only and SCM-backed (Git or Fossil), with a published candidate, fail-closed
per-scope runner results, and a verification receipt published as the
evidence-only finalization. Durable archive facts live in chapters 2, 3, 5,
6.7, 8.6, 8.9, 8.12, and the glossary.

The `scm-backend` change records ADR-071: repository-state commands select the
version-control backend through one seam (a Git worktree first, then a Fossil
checkout) and refuse absence, ambiguity, a nested root, or a missing executable
by name. Durable facts live in chapters 2 (TC-21, TC-33), 3, 4, 5, 6.13/6.16,
8.18, 10 (Q-50), and 11 (R-63, R-64, D-20).

The `maintain-project` change amends ADR-071: the managed upgrade and
migration bundles run on Git and Fossil through the same seam, with
allowlist-only force staging on both. Durable facts live in chapters 2
(TC-21), 3, 4, 5, 6.13, 8.18, and the glossary; D-20 is repaid.

The `archive-change` change records ADR-072: publication through the seam is
witness-bounded, the archive runs on Git and Fossil, and a candidate whose
publication a hook rejected is republished on resume. Durable facts live in
chapters 2 (TC-21, TC-31, TC-32), 3, 4, 5, 6.7/6.13/6.16, 7.3, 8.18, 10
(Q-92), 11 (R-65, R-66), and the glossary.

The approved `make-user-questions-decision-ready` change records a pending
universal dialog-policy discovery and question-quality contract. Its target
stakeholder quality objective, ownership, flows, delivery, review boundaries,
and risks are documented in chapters 1, 3, 5, 6, 7, 8, 10, 11, and the refined
existing dialog glossary entry.
The source policy and generated guidance are not changed by the planning
artifacts; implementation and fresh-context verification remain to be proved.
No new ADR or runtime question gate is introduced.

Only accepted, non-superseded ADRs are current architecture. See the [chapter 9
register](adr/README.md) for status, relationships, evidence, reserved IDs, and
historical records.

The [planned OpenCode attribution compatibility change](../../givn/changes/fix-opencode-v2-attribution/design.md)
keeps one embedded adapter and generated path for V1 1.18.29+ within V1 and
V2 across initialization and regeneration, and on V2 links each shell command
to its issuing session (subagents included). Chapters 2, 3, 5–8, and 10–12
record its planned scope, retained fallback limitations, and pending runtime
evidence. No ADR is created or amended: the local reversible approach remains
owned by the design. Documentation completion is not implementation or test
proof.

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
- [`../workflow.md`](../workflow.md) - ordered workflow phases, gates, and chokepoints.
- [`../migrations/0.2.0-to-0.3.0.md`](../migrations/0.2.0-to-0.3.0.md) - LLM prompt pack for the 0.3.0 project migration.
- [`../migrations/README.md`](../migrations/README.md) - release-owned migration bundle and aggregate-plan contract.

## Reading Order

1. Read chapters 1 and 4 for purpose and strategy.
2. Read chapter 8 for cross-cutting mechanics and chapter 6 for runtime order.
3. Read the [ADR register](adr/README.md) and only then individual decisions.
4. Treat proposed, deprecated, and superseded records as historical or pending evidence, not as accepted architecture.
