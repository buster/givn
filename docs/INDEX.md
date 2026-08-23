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

## Current Evidence

- [Real-Model Duplicate Filter Report](semantic-duplicates-real-model-filter-report.md) - `current`; PR #27 full-corpus duplicate audit with 394 permanent scenarios, 1,274 E5 candidates, real BGE/NLI runs, and explicit token-cap and unscored evidence.
- [E5 Threshold Versus Serialization Analysis](semantic-duplicates-e5-threshold-vs-serialization-analysis.md) - `current`; current E5 distribution, 2,048 default second-stage budget, ranked prefixes, and runtime measurements.
- [Manual Semantic Evaluation](semantic-duplicates-manual-evaluation.md) - `current`; manual classifications for the current PR #27 BGE and NLI top-five outputs. This is a local evaluation record; its raw JSON inputs are intentionally unversioned and ephemeral.
- [BGE Reranking Admission Benchmark](bge-reranking-benchmark.md) - `current`; BGE admission criteria and the current PR #27 run, with older measurements explicitly separated as historical.

## Project Planning

- [Implementation Task Breakdown](TASKS.md) - `current`; phase-by-phase implementation tasks and completion markers.
- [Root Plan](../PLAN.md) - `reference`; six-change refactor plan and dependency context outside the `docs/` tree.
- [Givn 0.2.0 to 0.3.0 Migration Guide](migrations/0.2.0-to-0.3.0.md) - `reference`; ordered LLM prompts for upgrading projects created from the 0.2.0 master branch.

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

## Evidence Boundary

The current operating point is the PR #27 run: `394` permanent scenarios,
`1274` E5 candidates, `795` BGE-eligible top-five-union pairs, and the default
`GIVN_SPEC_MAX_RERANK_PAIRS=2048`. The archived `385/100`, `425/199`, and old
pair-cap measurements are historical evidence only.
