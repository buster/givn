---
status: accepted
date: 2026-08-16
decision-makers:
  - givn maintainers
consulted:
  - change-aware exploration scenarios and design
informed:
  - authors consuming source-labeled exploration output
---

# Add active change scenarios to Explore request-locally

## Qualification

- [x] **Hard to reverse** - persisting delta scenarios would contaminate the permanent catalogue and change later search results.
- [x] **Shapes quality attributes** - request-local work preserves deterministic state and avoids abandoned-change leakage.
- [x] **Structural** - it defines the boundary between permanent index entries and transient current-change candidates.
- [x] **Must be shared** - authors must distinguish permanent evidence from active delta evidence.

## Context and Problem Statement

An author working in a change needs to compare a draft with scenarios already written in that change. Persisting those deltas as index entries would make incomplete work look permanent.

## Decision Drivers

- include active Added and Modified scenarios
- exclude Removed scenarios
- preserve permanent-only persistence
- filter source and projected-target self-identities
- label results in text and JSON

## Considered Options

- Persist active deltas in the permanent index.
- Keep Explore permanently scoped and defer current-change comparison.
- Merge active deltas into a request-local corpus and discard their vectors after rendering.

## Decision Outcome

Chosen option: **Use a request-local current-change corpus for `givn spec explore --change <id>`**. The command validates and parses the change delta before refreshing the permanent index, embeds active Added/Modified scenarios in memory, excludes Removed scenarios, and labels results with `scope` and `change`.

### Consequences

- The permanent index remains the only persisted catalogue; `index.entries` excludes deltas.
- A Modified scenario has both delta-source and projected-target identities; either can be excluded as the query identity. Added scenarios use their source identity.
- BGE and NLI consume the surface's current E5 result pool independently; they do not persist delta evidence.

### Confirmation

Change-aware Explore scenarios assert source labeling, added/modified filtering, removed exclusion, file/stdin parity, rejection before refresh, and permanent-index non-persistence.

## Pros and Cons of the Options

### Request-local corpus

- Good, because active work is visible without changing permanent state.
- Bad, because each request pays transient embedding work.

### Persist deltas

- Good, because later requests could reuse vectors.
- Bad, because abandoned or rejected changes contaminate the catalogue.

### Permanent-only Explore

- Good, because state remains simple.
- Bad, because repetition inside one active change is invisible until review.

## More Information

See [ADR-043](ADR-043-permanent-explore.md) and [ADR-045](ADR-045-project-relative-identity.md).
