# Documentation Index

Central entry point for the durable documentation under `docs/`. Current
contracts and historical evidence are separated so that neither is mistaken for
the other.

Status values:

- `current` — current project, contract, or evidence document.
- `reference` — current operational or optional-tool reference.
- `historical` — retained measurement, analysis, or handoff from an earlier
  state; not current behavior.
- `archive` — retained historical planning and decision context.

## Start Here

- [Project README](../README.md) — purpose, installation, quick start, brief
  guarantees, and agent targets.
- [Workflow Overview](workflow.md) — lifecycle, phases, behavior batches, hard
  gates, verification scopes, archive recovery, and limits.
- [Reference](reference.md) — authoritative CLI, configuration authority,
  skills and invocation, addons, runner and receipt contracts, recovery, and
  generated project layout.
- [Workflow Diagram](workflow.svg) — three-column lifecycle view: action,
  producing artifact, and checks. The diagram is embedded in
  [workflow.md](workflow.md).
- [Documentation Index](INDEX.md) — this navigation document.

## Project and Process

- [Use-Case Model](use-case-model.md) — current; canonical use-case definition,
  scoping rules, authoring requirements, and review rejection criteria.
- [Glossary](glossary.md) — current; the sole vocabulary authority for the
  project's domain and technical terms.
- [Domain Model](domain-model.md) — current; thin structural map of the working
  domain, its bounded contexts, aggregates, and relationships.
- [Architecture Principles addon](architecture-principles.md) — reference;
  enabled-by-default guidance for decision timing, consumer contracts,
  component boundaries, and ports/adapters, with its expected outcome and
  warning signs.
- [Outline addon](outline.md) — reference; advisory per-component call-tree
  report, supported Rust/Python/Java/TypeScript/JavaScript scope, REFACTOR and review use, honest
  measurement limits, and no gate or verdict.
- [Development Record and Report](development-report.md) — current; local,
  ignored recording, its sensitivity and best-effort limits, report screens, and
  analytics over the JSONL store.
- [Design System](design/design-system.md) — current; the development report's
  tokens, primitives, breakpoints, accessibility rules, and reference views.
- [Manual Transition to 1.0](migrations/README.md) — current; the supported
  migration baseline, manual configuration repair, skills-only transition,
  explicit commit paths, and checkpoint recovery.
- [Migration Catalog Contract](migrations/README.md#future-migration-bundles) —
  current; the empty shipped catalog, schema, and the future-route contract.

## Architecture

- [Arc42 Architecture README](arc42/README.md) — current; architecture entry
  point, documentation ownership, chapter map, and explicit unknowns.
- [ADR Register](arc42/adr/README.md) — current; accepted decisions, status
  rules, amendments, relationships, evidence, and archived records.
- [Arc42 01: Introduction and Goals](arc42/01-introduction-and-goals.md) —
  current; purpose, requirements, quality goals, and stakeholders.
- [Arc42 02: Architecture Constraints](arc42/02-architecture-constraints.md) —
  current; technical, configuration, and process constraints.
- [Arc42 03: Context and Scope](arc42/03-context-and-scope.md) — current; system
  boundary, external interfaces, and scope.
- [Arc42 04: Solution Strategy](arc42/04-solution-strategy.md) — current;
  strategic choices and their consequences.
- [Arc42 05: Building Block View](arc42/05-building-block-view.md) — current;
  crate, module, and responsibility decomposition.
- [Arc42 06: Runtime View](arc42/06-runtime-view.md) — current; init, guidance,
  checks, verification, archive, upgrade, reporting, and advisory analysis
  flows.
- [Arc42 07: Deployment View](arc42/07-deployment-view.md) — current;
  distribution, installation, target environments, and release channels.
- [Arc42 08: Cross-cutting Concepts](arc42/08-crosscutting-concepts.md) —
  current; cascade, manifest, Gherkin deltas, state, guidance, hooks,
  verification, addons, records, and structural analysis.
- [Arc42 09: Architecture Decisions](arc42/09-architecture-decisions.md) —
  current; pointer to the register and the amendment map.
- [Arc42 10: Quality Requirements](arc42/10-quality-requirements.md) — current;
  quality tree, scenarios, and verification signals.
- [Arc42 11: Risks and Technical Debt](arc42/11-risks-and-technical-debt.md) —
  current; current risks, deliberate debt, resolved boundaries, and the concrete
  post-1.0 backlog.
- [Arc42 12: Glossary](arc42/12-glossary.md) — current; links to the
  standalone [glossary](glossary.md), the sole vocabulary authority.

## Historical Handoffs

These are retained for provenance only. They are not current navigation for
project behavior; the current contracts live in the sections above.

- [Multi-SCM support](archive/ideation/multi-scm-support.md) — historical; the
  completed Git+Fossil ideation handoff and its judgment calls.
- [Local-only integrations](archive/ideation/local-only-integrations.md) —
  historical; the completed rules-policy ideation handoff. Statements about
  source availability and then-current staging reflect its handoff date.
- [Rules through instructions](archive/ideation/rules-through-instructions.md) —
  historical; the completed served-instruction ideation handoff. Skills-only
  invocation superseded its generated-command assumptions.
- [Ise rebuildability](ideation/ise-rebuildability.md): historical; the
  completed Specification-rebuild ideation handoff. Its six ready change seeds
  live under `givn/ideation/ise-rebuildability/changes/`; the term merge is
  deferred to the seeds that make each row true.

## Archived Analysis

- [Givn 0.3 Refactor Analysis](archive/analysis/analysis.md) — archive; 2026-08-14
  pre-retrieval refactor decisions, findings, and sequencing rationale.
- [Combined Givn/Watn Approach Comparison](archive/analysis/approach-comparison-report.md) —
  historical; combined 425-scenario experiment with a 199-pair strict E5 pool and
  a 1,428-pair top-five union.
- [Semantic Serialization Token Budget Analysis](archive/analysis/semantic-serialization-token-budget-analysis.md) —
  historical; tokenizer and pair-cap measurements for the earlier corpus.
- [Real-Model Semantic Duplicate Analysis](archive/analysis/semantic-duplicates-real-model-analysis.md) —
  historical; 2026-08-19 real-model run based on 385 permanent scenarios and 100
  E5 candidates.
- [Manual Classification Questionnaire](archive/analysis/semantic-duplicates-manual-classification-questionnaire.md) —
  historical; 20-case stress questionnaire from the old candidate baseline.
- [Future Scenario-Level Branch Coverage Experiment](archive/analysis/future-coverage-experiment.md) —
  reference; an optional, unimplemented coverage experiment retained for future
  consideration.
- [Use-Case Migration Release Plan](archive/analysis/usecase-migration-release-plan.md) —
  archive; ordered sessions and release rules for the completed
  groups-to-use-cases migration; superseded by the shipped use-case corpus.
- [Rule Refinement Handovers](archive/rulesrefinement/README.md) — archive; the
  eight handover documents that seeded archived rule-refinement changes.

## Archived Benchmarks

- [Cross-Encoder Benchmark](archive/benchmarks/cross-encoder-benchmark.md) —
  historical; earlier 208-scenario BGE reranker branch comparison.
- [NLI Experiment Benchmark](archive/benchmarks/nli-benchmark.md) — historical;
  earlier NLI comparison branch run on 208 permanent scenarios and 839
  integrated candidates.

## Maintenance Rule

A current document names current behavior and links to the one authoritative
owner of each contract. A completed handoff, measurement, or superseded
operational guide moves to a historical or archive section with its status
stated; it is not silently deleted when its rationale still has provenance
value.
