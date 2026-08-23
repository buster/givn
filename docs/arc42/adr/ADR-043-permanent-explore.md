---
status: accepted
date: 2026-08-16
decision-makers:
  - givn maintainers
consulted:
  - permanent exploration scenarios and retrieval implementation
informed:
  - authors exploring complete draft scenarios
---

# Explore one complete scenario against the permanent corpus

## Qualification

- [x] **Hard to reverse** - the input, identity, index refresh, and output contracts form a durable authoring interface.
- [x] **Shapes quality attributes** - incremental reuse and deterministic results affect authoring performance and correctness.
- [x] **Structural** - `givn spec explore` defines a separate query boundary from search and blocking review.
- [x] **Must be shared** - authors need a common pre-creation check that does not mutate feature specifications.

## Context and Problem Statement

Free-text search cannot prove that a draft has the same complete Gherkin shape as a specification, and a query may need to see related scenarios in the same file. Exploration must remain advisory and non-mutating.

## Decision Drivers

- exactly one complete file or stdin scenario
- permanent index reuse
- same-file candidates except exact identity
- stable text and JSON output

## Considered Options

- Accept arbitrary free text.
- Exclude every scenario from the query file.
- Accept exactly one complete scenario and exclude only its exact identity.

## Decision Outcome

Chosen option: **Accept exactly one complete scenario via `--file` or `--stdin`**. Parse the query first, refresh the permanent index, embed it with the query role, rank permanent entries, and return at most five results. A file query excludes only the normalized `(file, title)` identity; stdin has no file identity. The command is advisory and never edits feature files.

### Consequences

- Invalid, missing, or multi-scenario input fails before index refresh.
- An over-cap query is reported as cap evidence without a fabricated rank or score.
- Optional BGE/NLI evidence attaches to the already ranked result set; it does not turn Explore into a gate.

### Confirmation

Explore scenarios cover file/stdin validation, same-file filtering, incremental index reuse, cap visibility, stable JSON, non-mutation, and feature-free failure.

## Pros and Cons of the Options

### One complete scenario with exact identity filtering

- Good, because the query matches the real authoring boundary and preserves useful same-file evidence.
- Bad, because callers must provide valid Gherkin.

### Arbitrary free text

- Good, because input is convenient.
- Bad, because results do not represent the artifact the author will commit.

### Exclude the whole query file

- Good, because self-matches are easy to avoid.
- Bad, because related scenarios in the same capability disappear.

## More Information

The optional current-change extension is [ADR-044](ADR-044-current-change-explore.md).
