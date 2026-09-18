---
status: accepted
date: 2026-09-08
decision-makers:
  - givn maintainers
consulted:
  - retire-semantic-capabilities tombstone and the permanent spec corpus
informed:
  - contributors, agents, and CI users
---

# Ship givn as a single feature-free binary without the semantic retrieval engine

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - the engine source, its model dependencies, and the feature matrix leave the codebase and lockfile; reintroduction is a new subsystem decision.
- [x] **Shapes quality attributes** - build time, binary size, CI disk footprint, and the absence of any model-download surface are permanent properties of every install.
- [x] **Structural** - the `[features]` matrix, the engine module, and the semantic gates in check/archive disappear from the architecture.
- [x] **Must be shared** - build contract, command surface, and gate semantics must be interpreted consistently by contributors, agents, and CI.

## Context and Problem Statement

`retire-semantic-capabilities` tombstoned the fifteen capabilities that
specified the semantic engine, leaving a deterministic-only permanent spec
corpus. The engine code, its four Cargo features, its model dependencies
(E5 embedding, BGE reranker, NLI evidence), and its gates survived that
retirement. The consequences observed in the current checkout:

- The documented install contract no longer matches reality: `default = []`
  builds feature-free, yet the register and README still describe engine
  capabilities and feature selection.
- The test suite still partitions every run into five feature × profile
  build combinations with simulated-model environments, and the coverage
  replay executes roughly ten groups whose scenarios cannot exist.
- The installed-build strictness gate executes zero checks while still
  paying a full release build, because the release_strict test modules are
  feature-gated out of the default build.
- Every feature × profile transition recompiles the engine crate and its
  heavy native dependencies, dominating suite wall time after any source
  change.

## Decision Drivers

- one observable build contract: a single binary, no build variants
- no model dependencies, no model-download surface, no stub-model
  environments in the test suite
- the permanent spec corpus is deterministic-only; no capability specifies
  the engine any more
- deterministic discovery, duplicate-audit, progress, coverage, and route
  tools keep their contracts unchanged

## Considered Options

- Keep the engine and features as dead, unreachable code behind opt-in flags.
- Keep the engine source unconditionally compiled (drop only the flags).
- Delete the engine, its dependencies, and the feature matrix; ship one binary.

## Decision Outcome

Chosen option: **Delete the engine, its dependencies, and the feature matrix; ship one feature-free binary.**

The Cargo `[features]` block, the optional model dependencies, the
`spec::retrieval` engine, and the semantic gates in `check review` and
`archive` are removed. The engine-backed commands (`spec index`, `spec
search`, `spec explore`) leave the CLI surface. The deterministic tools
(`spec find`, `spec tree`, `spec duplicates`, `spec progress`, `spec
coverage`, `spec route`) keep their contracts, and deterministic overlap
fingerprints under [ADR-037](ADR-037-deterministic-overlap-fingerprint.md)
remain the review-evidence boundary.

### Consequences

- Every build and install is feature-free; the build contract scenarios
  assert that no optional features are declared.
- The suite runs as a single feature-free pass; the coverage replay produces
  two distinct reports (non-e2e / e2e) from two single groups.
- The installed-build strictness capability is retired; no strictness gate
  proves engine behaviour any more.
- Build and CI wall time lose the feature × profile recompilation churn.
- Model-backed search can only return as a new decision with new evidence.

### Confirmation

The build-contract scenario in the `release-0-3-0` capability, the
retired-command e2e scenario, and the feature-free suite runs assert this
decision.

## Pros and Cons of the Options

### Delete engine and feature matrix

- Good, because documentation, build, tests, and binary finally agree.
- Good, because native model dependencies and their compile cost leave CI.
- Bad, because measured semantic evidence machinery is discarded and would
  need rebuilding from scratch if semantic review returns.

### Keep engine as opt-in dead code

- Good, because nothing is deleted.
- Bad, because unreachable code still compiles in the matrix and still
  misleads contributors and agents.

### Compile engine unconditionally

- Good, because the feature matrix disappears.
- Bad, because every binary carries unused native model dependencies and
  the corpus specifying the engine is already retired.

## More Information

Supersedes [ADR-039](archive/ADR-039-local-retrieval.md),
[ADR-041](archive/ADR-041-e5-uniqueness-gate.md),
[ADR-046](archive/ADR-046-semantic-review-report.md),
[ADR-047](archive/ADR-047-projected-remediation.md),
[ADR-052](archive/ADR-052-bge-advisory-evidence.md),
[ADR-055](archive/ADR-055-independent-semantic-features.md),
[ADR-056](archive/ADR-056-bge-approval.md),
[ADR-057](archive/ADR-057-bge-local-artifacts.md),
[ADR-058](archive/ADR-058-nli-evidence.md),
[ADR-059](archive/ADR-059-complete-semantic-serialization.md),
[ADR-060](archive/ADR-060-actual-tokenizer-caps.md),
[ADR-061](archive/ADR-061-surface-specific-independent-pools.md),
[ADR-063](archive/ADR-063-reranker-pair-budget.md),
[ADR-064](archive/ADR-064-shared-model-thread-budget.md),
[ADR-066](archive/ADR-066-unified-semantic-rulebook.md), and
[ADR-068](archive/ADR-068-remediation-log.md),
[ADR-044](archive/ADR-044-current-change-explore.md),
[ADR-045](archive/ADR-045-project-relative-identity.md), and
[ADR-054](archive/ADR-054-versioned-evidence-output.md).

Executed by the `drop-embedding-features` change, following the
`retire-semantic-capabilities` tombstone.
