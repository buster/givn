# Group: spec-discovery

## Actor

An authoring agent looking for existing scenarios before writing new ones.

## Goal

Make the permanent corpus discoverable deterministically — duplicates,
progress reporting, and strict-model overrides — with no embedding model
in the loop.

## Main flow

1. Search for existing scenarios that cover the intended behaviour.
2. Report duplicates and corpus progress (`givn spec duplicates`,
   progress output).
3. Apply release-strict overrides where configured.

## Interactions

- audit the permanent corpus for exact and structural duplicates
- search the permanent corpus lexically with `givn spec find`
- inspect the corpus overview with `givn spec tree`
- route a change proposal against the permanent corpus
- verify the JSON duplicate audit channel stays clean of progress wording
- keep the deterministic audit free of model wording

## Includes

- support (spec-explore-change, spec-explore-permanent)

## Extends

- none

## Out of scope

- Embedding-based search ranking (retired).
- Merging deltas (archival).

## Diagram

```mermaid
flowchart LR
  Actor((Agent)) --> UC1[[Find duplicates]]
  Actor --> UC2[[Report progress]]
```
