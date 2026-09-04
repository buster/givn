# Group: authoring

## Actor

A developer starting and driving a givn change from proposal to
implementation.

## Goal

Create, name, and prepare changes correctly — from `givn init` of the
project through `givn new`, artifact instructions, and the extras that
come with installation.

## Main flow

1. Initialise the project and pick agent targets (`givn init`).
2. Start a change and write its artifacts (`givn new`, `givn instructions`).
3. Install optional extras where the workflow needs them.

## Interactions

- Consumer selects agent targets during interactive init
- Consumer runs init non-interactively and gets opencode skills
- Consumer re-syncs skills after init and the selection is honoured
- Canonical E2E policy is emitted by specs instructions
- Generated guidance points to the canonical E2E policy

## Includes

- support (instruction-integrity, config-overlay)

## Extends

- none

## Out of scope

- Archiving and merging (archival).
- Status reporting (status).

## Diagram

```mermaid
flowchart LR
  Actor((Developer)) --> UC1[[Start a change]]
  Actor --> UC2[[Request instructions]]
  Actor --> UC3[[Install extras]]
```
