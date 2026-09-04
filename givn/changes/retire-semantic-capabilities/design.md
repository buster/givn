# Design: retire-semantic-capabilities

## Overview

No new architecture. This change only removes capability directories from
the permanent spec corpus via the capability-retirement mechanism already
shipped in the binary (`@givn.retired` delta op, tombstone in
`givn/specs/.retired.yaml`).

## Technology decisions

None. No new components, no new step definitions, no new dependencies.
Every delta in this change is a zero-scenario `@givn.retired` feature
file; `givn archive` deletes the matching capability directory and records
a tombstone entry.

## Step Definitions

None. No new step definitions: the retirement deltas in this change have
zero scenarios, so there is nothing to implement. Each task in tasks.md is
a pure deletion verified by re-running the full suite and confirming the
capability directory is gone after archive.

## Architecture Impact

- `givn/specs/` shrinks by 15 capability directories.
- `givn/specs/.retired.yaml` is created (tombstone register, covered by
  archive backup/rollback).
- Nothing else changes. The step definitions for the retired capabilities
  still compile at this point; dropping the Cargo feature flags is
  explicitly a later change (blocked on the deterministic `spec`
  subcommand rebuild).
