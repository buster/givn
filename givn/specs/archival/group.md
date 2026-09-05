# Group: archival

## Actor

A developer archiving a completed, reviewed change.

## Goal

Merge deltas into the permanent corpus, run the post-merge gates, and
move the change into the archive — atomically, with rollback on failure.

## Main flow

1. Run the archive gate hooks (verify, verify-e2e).
2. Merge (or retire) delta features into `givn/specs/`.
3. Run post-merge verify and coverage gates; roll back on failure.
4. Move the change directory to `givn/archive/`.

## Interactions

- Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner
- Consumer archives a change with coverage enabled and the coverage summary appears in README
- Consumer archives a modified scenario and the permanent spec keeps the docstring
- Consumer archives a new capability and the merged spec lands in its declared group
- Consumer archives a new capability and the merged spec lands in its declared group

## Includes

- verification (the gates run before any mutation)
- support (coverage-script-workflow for coverage merge commands)

## Extends

- none

## Out of scope

- Reviewing the change (check review, not archive).
- Capability routing decisions (proposal-time).

## Diagram

```mermaid
flowchart LR
  Actor((Developer)) --> UC1[[Gate change]]
  UC1 --> UC2[[Merge deltas]]
  UC2 --> UC3[[Move to archive]]
```
