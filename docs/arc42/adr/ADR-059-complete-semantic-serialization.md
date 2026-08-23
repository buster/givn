---
status: accepted
date: 2026-08-19
decision-makers:
  - givn maintainers
consulted:
  - semantic serialization and token-cap change records
informed:
  - retrieval maintainers and agents consuming semantic evidence
---

# Use one complete semantic serialization and hash its exact E5 text

## Qualification

- [x] **Hard to reverse** - serialized content and hash boundaries determine vector reuse and evidence compatibility.
- [x] **Shapes quality attributes** - complete context improves semantic precision while increasing token and runtime cost.
- [x] **Structural** - the serializer is the shared input boundary for E5, BGE, and NLI.
- [x] **Must be shared** - every semantic surface must score the same meaning and invalidate stale vectors consistently.

## Context and Problem Statement

The former short projection omitted feature/rule context, inherited backgrounds, roles, tables, docstrings, outline templates, and examples. Model input and reviewed endpoint evidence could therefore disagree, and old vectors could be reused after a model-input change.

## Decision Drivers

- complete Gherkin meaning
- deterministic serialization
- exact content-based invalidation
- separation of model text and audit metadata

## Considered Options

- Keep the short projection.
- Put complete context only in report evidence.
- Use one complete deterministic serialization as model input and hash that text.

## Decision Outcome

Chosen option: **Serialize complete parsed Gherkin context once for E5, BGE, and NLI**. Stable order is feature, optional rule, inherited backgrounds, scenario identity/description, resolved behavior steps, tables, docstrings, outline template, and examples. Normalized empty optional fields are omitted; non-empty content is retained.

The exact serialized text supplies `content_hash`. A persistent vector is reusable only when endpoint identity and this hash match. Paths, tags, scores, classifications, and Fingerprint evidence remain outside model text; there is no separate serializer-version field.

### Consequences

- Serializer changes re-embed affected entries and invalidate prior semantic evidence through content hashes.
- Complete passages can exceed the 512-token limit and therefore require [ADR-060](ADR-060-actual-tokenizer-caps.md).
- Report endpoint evidence is a separate metadata representation and can be richer than model text.

### Confirmation

Serialization tests cover feature/rule context, backgrounds, roles, tables, docstrings, outlines, examples, empty-field omission, deterministic text, hash reuse, old-projection invalidation, and deletion cleanup.

## Pros and Cons of the Options

### One complete serialization

- Good, because model inputs and semantic meaning share one boundary.
- Bad, because passages are longer and need explicit token safety.

### Short projection

- Good, because input is smaller.
- Bad, because meaningful context can be omitted and stale vectors can survive.

### Complete report evidence only

- Good, because reports become richer without changing model cost.
- Bad, because the model scores a different representation from the reviewed evidence.

## More Information

The omission refinement is [ADR-062](ADR-062-omit-empty-optional-fields.md); model input mechanics are in [chapter 8.5g](../08-crosscutting-concepts.md#85g-shared-complete-semantic-serialization-and-token-budget-guard).
