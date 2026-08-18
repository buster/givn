# givn — Architecture Documentation (arc42)

This is the architecture documentation for **givn**, following the
[arc42](https://arc42.org) template. Each chapter is a separate file to keep
documents focused and reviewable.

> **Status:** Phases 0–5 implemented (core engine complete). Phases 6–7 (distribution,
> dogfood) pending. Characterization of the `new` command in progress
> (change `reverse-engineer-new-command`). All architecture decisions remain
> binding — see chapter 9.

The semantic retrieval surface includes advisory exploration through
`givn spec explore`, an optional request-local current-change corpus, and the
blocking E5 `0.9500` semantic review worklist generated only by `givn check
review --change <id>`. The opt-in `cross-encoder-reranker` layer adds bounded,
bidirectional BGE evidence to the five complete-scenario/audit E5 surfaces in
PR 7; free-text `givn spec search` remains E5-only. It keeps E5 raw
candidates visible, emits `BGE_RECOMMENDED` and non-blocking `BGE_FILTERED`
evidence, and never assigns a Givn classification or edits feature files.
Remediation verification remains agent-owned for admitted candidates. A build
without retrieval fails this blocking layer rather than passing.

The same retrieval surface also provides `givn spec duplicates`, a complete
read-only permanent-corpus audit. It reports exact and existing-Fingerprint
structural relationships plus advisory E5 candidates in one fixed text/JSON
contract, with optional bounded BGE raw/recommended/filtered extensions,
without reading, refreshing, or writing the persistent index and without
blocking ordinary change work. It excludes active changes and archive contents;
only the existing local model cache may be created.

When the `arc42` addon is enabled, completion review also cross-checks durable
Arc42 facts against the change's design, mapped tasks, and completed
implementation. See chapter 8.14 and ADR-035.

## Chapters

| # | Chapter | Purpose |
| --- | --- | --- |
| 1 | [Introduction and Goals](01-introduction-and-goals.md) | What givn is, top requirements, quality goals, stakeholders |
| 2 | [Architecture Constraints](02-architecture-constraints.md) | Technical and organisational constraints that are fixed |
| 3 | [Context and Scope](03-context-and-scope.md) | System boundary, external interfaces, in/out of scope |
| 4 | [Solution Strategy](04-solution-strategy.md) | Key strategic decisions in brief |
| 5 | [Building Block View](05-building-block-view.md) | Static decomposition: crate, modules, responsibilities |
| 6 | [Runtime View](06-runtime-view.md) | Key scenarios: init, propose, lint, check (hooks and semantic report gate), permanent/current-change explore, permanent duplicate audit, strict similarity review, archive, eject |
| 7 | [Deployment View](07-deployment-view.md) | Distribution, install methods, release pipeline |
| 8 | [Cross-cutting Concepts](08-crosscutting-concepts.md) | Cascade engine, Gherkin delta model, TDD loop, coverage gate, skills, similarity, duplicate audit, and semantic review gates |
| 9 | [Architecture Decisions](09-architecture-decisions.md) | ADRs — every decision from the Q&A, with rationale |
| 10 | [Quality Requirements](10-quality-requirements.md) | Quality tree and concrete scenarios, including duplicate-audit contract and model metrics |
| 11 | [Risks and Technical Debt](11-risks-and-technical-debt.md) | Known risks and planned debt |
| 12 | [Glossary](12-glossary.md) | Domain and technical terms |

## Related documents

- [`../../README.md`](../../README.md) — user-facing project overview.
- [`../TASKS.md`](../TASKS.md) — phase-by-phase implementation task breakdown.

## How to use this

1. Read chapter 1 and 4 for the big picture.
2. Read chapter 8 (cross-cutting concepts) — it is the heart of the design.
3. Read chapter 9 to understand *why* each decision was made (do not relitigate).
4. See [`../TASKS.md`](../TASKS.md) for the phase-by-phase task breakdown and current completion status.
