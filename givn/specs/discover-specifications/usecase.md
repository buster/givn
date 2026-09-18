# Use case: discover-specifications

## Level

!

## Actors

- Authoring agent

## Goal

Find existing behavior deterministically before authoring or reviewing a specification.

## Trigger

The agent investigates a proposed behavior or the permanent corpus.

## Preconditions

- The active specification corpus is readable.

## Main flow

1. The agent searches or inspects the corpus.
2. The system reports exact and structural overlap.
3. The system preserves deterministic evidence for review.

## Extensions

- 1: model evidence is unavailable → the deterministic checks remain useful; protects the minimal guarantee.

## Rules

- Discovery commands do not modify permanent specifications.
- JSON output remains machine-readable.

## Examples

- An agent audits duplicate scenarios without downloading a model.

## Minimal guarantee

The corpus remains unchanged by discovery.

## Success guarantee

The agent receives stable, actionable candidates and findings.

## Personas

- agent--technical-role

## Capabilities

- keep-audit-output-clean
- spec-duplicates

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| keep-audit-output-clean | request JSON duplicate audit | The JSON duplicate audit is valid JSON without progress wording |
| spec-duplicates | audit duplicates in one feature | Exact duplicates in one feature are reported together |
| spec-duplicates | audit duplicates across features | Exact duplicates across features are reported together |

## Includes

- fragment: support

## Extends

- none

## Out of scope

Changing the behavior of a capability discovered in the corpus.

## Diagram

```mermaid
flowchart LR
  Agent((Agent)) --> Search[[Search corpus]]
  Search --> Evidence[[Report deterministic evidence]]
```

<!-- givn:base-sha256:c03449eed80b6f3143a0b5b1a5d352009680da200f1c0bf5ad71737b9b22a118 -->
