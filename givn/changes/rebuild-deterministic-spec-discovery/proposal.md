# Proposal: rebuild-deterministic-spec-discovery

## Problem / Opportunity

The four `spec-*` capabilities that describe discovery subcommands
(`spec-duplicates`, `spec-explore-change`, `spec-explore-permanent`,
`spec-progress-output`) were written for the embedding era. After the
deterministic rebuild (Phases 4.1–4.5 of the retirement plan) their
scenarios assert behaviour that no longer exists: `givn spec duplicates`
is now exact/structural only, explore/search remain opt-in behind the
`spec-retrieval` feature, and model-download progress wording is gone
from deterministic commands. 33 of the 41 scenarios in the group fail
against the current binary.

This change is the precondition for dropping the Cargo feature flags
from the default build (retirement plan task 1.2): the four
capabilities' specs must describe only the deterministic behaviour that
remains.

## Proposed Solution

Rewrite all four capabilities:

- **spec-duplicates**: the deterministic audit — exact duplicates,
  shared assertion shapes, assertion subsets, JSON output, malformed
  inputs, missing-corpus behaviour, archive exclusion, determinism,
  non-mutation, and model-wording hygiene.
- **spec-explore-permanent**: deterministic discovery — `givn spec find`
  (AST-based lexical search), `givn spec tree` (per-group counts and
  narrative), non-mutation, no phantom hits from docstrings.
- **spec-explore-change**: routing — `givn spec route --change` ranked
  candidates with evidence, EXTEND/NEW recommendations, exactly-one-input
  validation, non-mutation.
- **spec-progress-output**: clean output channels — the JSON audit is
  valid JSON free of progress/model wording; repeated audits are stable.

All new scenarios drive the compiled CLI against fixture projects with
the grouped corpus layout. The delta removes the 73 superseded scenarios
and adds 25 deterministic replacements. The `spec-discovery` group.md
interaction inventory is reconciled to the new e2e set.

## Out of Scope

- `givn spec explore` / `spec search` / `spec review` / `spec index`
  themselves: they remain opt-in behind `spec-retrieval` with their
  existing unavailable-diagnostics; a later change may retire them fully.
- Dropping the Cargo feature flags (task 1.2, immediately after this).

## Open Questions

None.
