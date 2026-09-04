# Group: status

## Actor

A developer inspecting the current state of changes and artifacts.

## Goal

Answer "where am I?" — what changes are active, which artifacts are done,
and what the next step is — without reading files by hand.

## Main flow

1. Inspect change and artifact state (`givn status`).
2. Report task progress and pending work.

## Interactions

- show change/artifact status tree-wide or for one change
- render task progress with checked/unchecked boxes
- validate change id input when a specific change is requested

## Includes

- support (config-overlay)

## Extends

- none

## Out of scope

- Mutating any state; status is read-only.

## Diagram

```mermaid
flowchart LR
  Actor((Developer)) --> UC1[[Inspect status]]
  UC1 --> UC2[[Report progress]]
```
