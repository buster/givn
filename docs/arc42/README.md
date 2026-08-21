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
review --change <id>`. The opt-in `cross-encoder-reranker` layer adds
bidirectional BGE evidence to the five complete-scenario/audit E5 surfaces;
`GIVN_SPEC_MAX_RERANK_PAIRS` selects the highest E5-scoring eligible pairs,
defaulting to `2048`. Free-text `givn spec search` remains E5-only. E5 raw
candidates remain visible, omitted budget tails are `BGE_UNSCORED`, and
`BGE_RECOMMENDED`/`BGE_FILTERED` remain non-classifying evidence.
Remediation verification remains agent-owned for admitted candidates. A build
without retrieval fails this blocking layer rather than passing.

The independent `nli-evidence` layer uses the same eligible E5 pool and the
same ordered pair budget on the same five surfaces. It reports bidirectional
entailment, neutral, and contradiction probabilities, emits
`NLI_RECOMMENDED` or non-blocking `NLI_FILTERED` evidence at the inclusive
symmetric entailment point `0.5000`, and records the budget tail as
`NLI_UNSCORED`. It intersects its recommendations with BGE when both layers
are active. E5 raw candidates remain visible and NLI never assigns a Givn
classification.

All semantic layers now consume one complete, deterministic Gherkin
serialization containing Feature/Rule context and descriptions, inherited
backgrounds, resolved step roles, tables, docstrings, outline templates, and
concrete example rows. Optional sections and fields are emitted only when they
contain normalized content; empty markers are omitted so shared boilerplate
cannot raise cosine similarity. The exact serialized E5 passage is the
`content_hash` boundary: an existing vector is reused only when identity and
hash match, with no separate serializer-version field. Paths, tags, scores,
classifications, and deterministic fingerprints remain outside model text. The actual local
tokenizer preflights untruncated and unpadded E5 passages and BGE/NLI pair
encodings, including pair special tokens; `512` fits and `513` is over the
limit. Fitting E5 passages remain indexed; an over-cap passage is retained as
additive `E5_TOKEN_CAP` metadata with no vector or semantic pair score. Its
file/title, complete token count, limit, model/tokenizer provenance,
`refactoring_required: true`, and shorten/split-and-rerun guidance remain
visible and blocking without an automatic Givn classification. `BGE_TOKEN_CAP`
and `NLI_TOKEN_CAP` remain visible, unresolved, non-filtered, and blocking per
candidate, while run-level unavailable means no pair in that layer was
scoreable. BGE and NLI independently score the same eligible E5 pool, and
combined recommendations are their intersection.

The same retrieval surface also provides `givn spec duplicates`, a complete
read-only permanent-corpus audit. It reports exact and existing-Fingerprint
structural relationships plus advisory E5 candidates in one fixed text/JSON
contract, with optional BGE/NLI raw/recommended/filtered/unscored extensions
selected by the E5-ranked `GIVN_SPEC_MAX_RERANK_PAIRS` budget, without reading,
refreshing, or writing the persistent index and without blocking ordinary
change work. It excludes active changes and archive contents; only the existing
local model cache may be created.

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
| 9 | [Architecture Decisions](09-architecture-decisions.md) | ADRs — rationale for the complete serializer/hash boundary, tokenizer guard, and independent second-stage pool |
| 10 | [Quality Requirements](10-quality-requirements.md) | Quality tree and scenarios for semantic completeness, exact token limits, cap evidence, and pool independence |
| 11 | [Risks and Technical Debt](11-risks-and-technical-debt.md) | Known risks and planned debt, including stale hashes, tokenizer mismatch, cap-state loss, and pool divergence |
| 12 | [Glossary](12-glossary.md) | Domain and technical terms, including serialization, token budgets, cap states, and combined admission |

## Related documents

- [`../../README.md`](../../README.md) — user-facing project overview.
- [`../TASKS.md`](../TASKS.md) — phase-by-phase implementation task breakdown.

## How to use this

1. Read chapter 1 and 4 for the big picture.
2. Read chapter 8 (cross-cutting concepts) — it is the heart of the design.
3. Read chapter 9 to understand *why* each decision was made (do not relitigate).
4. See [`../TASKS.md`](../TASKS.md) for the phase-by-phase task breakdown and current completion status.
