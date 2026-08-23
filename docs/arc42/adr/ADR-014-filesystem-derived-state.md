---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - lifecycle state design record
informed:
  - contributors and users
---

# Derive lifecycle state from the filesystem

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - adding a separate state database would change recovery, version control, and every lifecycle command.
- [x] **Shapes quality attributes** - filesystem state is inspectable, git-friendly, and avoids state drift.
- [x] **Structural** - artifact paths, markers, and the manifest graph define the state machine boundary.
- [x] **Must be shared** - commands and agents must interpret the same files and markers.

## Context and Problem Statement

Lifecycle state can be represented by a database or by the artifacts a change contains. A separate state file can disagree with manually edited or version-controlled artifacts.

## Decision Drivers

- no state drift
- inspectability in git
- recovery after interruption
- compatibility with configurable artifact graphs

## Considered Options

- Derive state from generated files and completion markers.
- Maintain a separate state file.
- Store state only in a remote service.

## Decision Outcome

Chosen option: **Derive state from the filesystem and effective manifest**. `givn status` computes artifact status, task checkbox progress, sign-off markers, skip markers, and archive presence without a separate state file.

### Consequences

- Manual edits are visible to the next status calculation.
- Ambiguous or partial files remain `Partial` rather than being repaired automatically.
- The configured graph, not a hardcoded list, controls which artifacts are considered.
- Hook execution is a separate gate concern refined by [ADR-028](ADR-028-hooks-and-mandatory-checks.md).

### Confirmation

State tests cover partial tasks, sign-off markers, skipped artifacts, manifest ordering, active/archive lookup, and JSON status output.

## Pros and Cons of the Options

### Filesystem-derived state

- Good, because the repository itself is the state record.
- Bad, because malformed or manually incomplete documents require conservative interpretation.

### Separate state file

- Good, because state can be compact and explicit.
- Bad, because it can drift from the artifacts and adds another file to merge.

### Remote state service

- Good, because state can be centralized.
- Bad, because it violates the local, offline project boundary.

## More Information

See [chapter 8.6](../08-crosscutting-concepts.md#86-filesystem-derived-state).
