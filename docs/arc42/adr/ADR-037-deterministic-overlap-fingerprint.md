---
status: accepted
date: 2026-08-14
decision-makers:
  - givn maintainers
consulted:
  - overlap gate implementation and fixtures
informed:
  - contributors and reviewers
---

# Use deterministic fingerprints for structural overlap

## Qualification

- [x] **Hard to reverse** - lint and review behavior depends on the fingerprint and its facts model.
- [x] **Shapes quality attributes** - deterministic overlap checks provide reproducible, explainable review evidence.
- [x] **Structural** - the fingerprint defines the relation between scenario assertions and the review disposition gate.
- [x] **Must be shared** - lint, review, duplicate audit, and agents need the same structural vocabulary.

## Context and Problem Statement

Scenario duplication and assertion overlap must be found without making model availability a prerequisite for basic workflow checks. Embedding similarity is useful evidence but is not a deterministic structural relation.

## Decision Drivers

- deterministic local execution
- explainable shape and subset findings
- conservative fact contradiction handling
- separation from semantic model evidence

## Considered Options

- Use only a semantic model.
- Use deterministic masked assertion fingerprints and extracted facts.
- Use title or token equality only.

## Decision Outcome

Chosen option: **Use a deterministic fingerprint of masked assertion steps plus extracted facts**. Identical active titles are lint errors; equal assertion shapes, strict assertion subsets, and scenarios over 14 steps are warnings. Shape findings involving a change require review dispositions unless extracted facts contradict.

### Consequences

- Quoted strings and numbers are masked; facts include exit codes, `contains` polarity, tables, `[DONE]`, and TTY mentions.
- Missing facts do not create a contradiction.
- Semantic E5/BGE/NLI evidence remains a separate layer; [ADR-041](ADR-041-e5-uniqueness-gate.md) governs its strict gate.

### Confirmation

Overlap and duplicate-audit scenarios assert deterministic order, relation direction, facts, contradiction behavior, exit codes, and disposition enforcement.

## Pros and Cons of the Options

### Deterministic fingerprint

- Good, because findings are local and explainable.
- Bad, because paraphrases outside the masked shape can be missed.

### Semantic model only

- Good, because paraphrases may be found.
- Bad, because setup, reproducibility, and model judgment become prerequisites for structural lint.

### Title or token equality

- Good, because implementation is simple.
- Bad, because it misses assertion subsets and meaningful structural overlap.

## More Information

See [chapter 8.8](../08-crosscutting-concepts.md#88-error-handling-and-exit-codes) and [chapter 6](../06-runtime-view.md#68-retrieval-primitives).
