---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - Gherkin delta design record
informed:
  - contributors and users
---

# Use the `@givn.*` delta namespace

## Qualification

- [x] **Hard to reverse** - tags are stored in active and archived feature files and changing them requires a repository-wide migration.
- [x] **Shapes quality attributes** - one namespace protects merge correctness and parser clarity.
- [x] **Structural** - the tags select delta operations and the feature path passed to the runner.
- [x] **Must be shared** - authors, lint, merge, retrieval, and runners must interpret the same tags.

## Context and Problem Statement

The OpenSpec prototype used a different tag namespace. givn needs an unambiguous namespace for feature-level deltas and scenario-level operations.

## Decision Drivers

- parser and merge clarity
- clean break from OpenSpec
- readable Gherkin source
- stable runner path via `GIVN_FEATURES`

## Considered Options

- Keep the `@openspec.*` namespace.
- Use `@givn.*` tags.
- Infer delta operations from file location only.

## Decision Outcome

Chosen option: **Use `@givn.delta`, `@givn.added`, `@givn.modified`, and `@givn.removed`**. A scenario without an operation tag defaults to added; `GIVN_FEATURES` identifies the feature tree passed to the test runner.

### Consequences

- Lint and merge do not interpret OpenSpec tags as givn operations.
- Archive strips all `@givn.*` tags from permanent output.
- Removed scenarios need a parseable placeholder and are excluded from relevant WIP checks.

### Confirmation

Delta parsing, lint, and archive tests assert operation selection, default-added behavior, tag stripping, and failure on a non-delta feature.

## Pros and Cons of the Options

### `@givn.*`

- Good, because the namespace is explicit and owned by the tool.
- Bad, because old tagged files need deliberate migration.

### Keep `@openspec.*`

- Good, because prototype files need fewer edits.
- Bad, because the successor remains coupled to the predecessor's vocabulary.

### Infer from file location

- Good, because feature contents contain fewer tags.
- Bad, because add/modify/remove intent becomes implicit and error-prone.

## More Information

See [chapter 8.3](../08-crosscutting-concepts.md#83-the-gherkin-delta-model).
