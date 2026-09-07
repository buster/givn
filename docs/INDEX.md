# Documentation Index

This is the central entry point for the durable documentation under `docs/`.
Current evidence is separated from historical measurements; archived documents
remain linked so that their provenance is not lost.

Status values:

- `current` - current project or evidence document.
- `historical` - retained measurement or analysis from an earlier corpus or run.
- `archive` - retained historical planning and decision context.
- `reference` - optional, planned, or operational reference material.

## Index

- [Documentation Index](INDEX.md) - `current`; this navigation document.

## Process and Reference

- [Workflow in Detail](workflow.md) - `current`; ordered phases with artifacts and completion criteria, the scenario TDD loop, advisory versus blocking gates, semantic duplicate assurance, status semantics, and archive mechanics.
- [Workflow Swimlane (SVG)](workflow.svg) - `current`; GitHub-safe swimlane rendering (pure shapes/text, no scripts or styles) of the full lifecycle with artifacts, checks, thresholds, and chokepoints. Regenerate via `python3 scripts/render-workflow-svg.py`.
- [Workflow Swimlane (HTML)](workflow.html) - `current`; self-contained HTML swimlane view with one lane per artifact, creating actions, checking processes, semantic pipeline panel, and chokepoint summary. No JavaScript, no external assets, dark-mode aware.
- [Reference](reference.md) - `current`; CLI command table, agent skills, addons, extras, configuration keys, generated project layout, repository development scripts.

## Project Planning

- [Givn 0.2.0 to 0.3.0 Migration Guide](migrations/0.2.0-to-0.3.0.md) - `reference`; ordered LLM prompts for upgrading projects created from the 0.2.0 master branch.
- [Migration Bundles](migrations/README.md) - `reference`; authoring contract for future release-owned migration bundles and aggregate plans.
- [Use-Case Migration Release Plan](usecase-migration-release-plan.md) - `reference`; ordered implementation sessions and release migration rules for replacing groups with semantic use cases, fragments, personas, and ideation-aligned coverage.

## Architecture

- [Arc42 Architecture README](arc42/README.md) - `current`; architecture-documentation entry point and chapter map.
- [ADR Index](arc42/adr/README.md) - `current`; active and archived architecture decisions, qualification basis, relationships, and individual ADR links.
- [Arc42 01: Introduction and Goals](arc42/01-introduction-and-goals.md) - `current`; product purpose, goals, stakeholders, and quality objectives.
- [Arc42 02: Architecture Constraints](arc42/02-architecture-constraints.md) - `current`; fixed technical, organisational, and process constraints.
- [Arc42 03: Context and Scope](arc42/03-context-and-scope.md) - `current`; system boundary, external interfaces, and scope.
- [Arc42 04: Solution Strategy](arc42/04-solution-strategy.md) - `current`; high-level architectural strategy and key implementation choices.
- [Arc42 05: Building Block View](arc42/05-building-block-view.md) - `current`; crate, module, and responsibility decomposition.
- [Arc42 06: Runtime View](arc42/06-runtime-view.md) - `current`; runtime flows for lifecycle, hooks, exploration, duplicate audit, review, and archive.
- [Arc42 07: Deployment View](arc42/07-deployment-view.md) - `current`; distribution, installation, and release concerns.
- [Arc42 08: Cross-cutting Concepts](arc42/08-crosscutting-concepts.md) - `current`; cascade, Gherkin deltas, skills, serialization, retrieval, and evidence gates.
- [Arc42 09: Architecture Decisions](arc42/09-architecture-decisions.md) - `current`; binding ADR record and decision rationale.
- [Arc42 10: Quality Requirements](arc42/10-quality-requirements.md) - `current`; quality tree and measurable requirements.
- [Arc42 11: Risks and Technical Debt](arc42/11-risks-and-technical-debt.md) - `current`; known risks, limitations, and planned debt.
- [Arc42 12: Glossary](arc42/12-glossary.md) - `current`; domain and technical terminology.

## Archived Analysis

- [Givn 0.3 Refactor Analysis](archive/analysis/analysis.md) - `archive`; 2026-08-14 pre-retrieval refactor decisions, findings, and sequencing rationale.
- [Combined Givn/Watn Approach Comparison](archive/analysis/approach-comparison-report.md) - `historical`; combined 425-scenario experiment with a 199-pair strict E5 pool and 1,428-pair top-five union.
- [Semantic Serialization Token Budget Analysis](archive/analysis/semantic-serialization-token-budget-analysis.md) - `historical`; tokenizer and pair-cap measurements for the earlier 425-scenario corpus and 199-pair pool.
- [Real-Model Semantic Duplicate Analysis](archive/analysis/semantic-duplicates-real-model-analysis.md) - `historical`; 2026-08-19 real-model run based on 385 permanent scenarios and 100 E5 candidates.
- [Manual Classification Questionnaire](archive/analysis/semantic-duplicates-manual-classification-questionnaire.md) - `historical`; 20-case stress questionnaire derived from the old 100-candidate baseline.
- [Future Scenario-Level Branch Coverage Experiment](archive/analysis/future-coverage-experiment.md) - `reference`; optional, unimplemented coverage experiment proposal retained for future consideration.

## Archived Benchmarks

- [Cross-Encoder Benchmark](archive/benchmarks/cross-encoder-benchmark.md) - `historical`; earlier 208-scenario BGE reranker branch comparison with standalone and integrated candidate pools.
- [NLI Experiment Benchmark](archive/benchmarks/nli-benchmark.md) - `historical`; earlier NLI comparison branch run on 208 permanent scenarios and 839 integrated candidates.
