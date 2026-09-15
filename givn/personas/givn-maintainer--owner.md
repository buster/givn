# Persona: Givn maintainer (owner)

## Role

The person maintaining givn itself — the user in this session. Owns the
portable seam, the permanent specifications, the constraint and ADR register,
the domain vocabulary, and the test harness. Merges the former seam-keeper
and truth-keeper segments by user decision.

## Interests

- O1: One SCM seam. No second, independently grown SCM code path
  (`src/cli/migration.rs` currently duplicates `src/git.rs`).
- O2: Truthful architecture record. No constraint, spec, glossary entry, or
  skill may claim Git where the contract is SCM-neutral.
- O3: Tests runnable with only one SCM installed; CI cost bounded; Fossil
  tests skippable where Fossil is unavailable but mandatory where it is.
- O4: No scope creep: no third SCM, no VCS library dependency, no kiekste
  hosting work in this change.

## Goals

- Land one portable interface with a Git and a Fossil backend.
- Keep every existing Git guarantee provably intact.
- Keep the domain vocabulary consistent across specs, design, and code.

## Pain points

- The broad rename surface across specs, docs, skills, ADRs, and fixtures.
- Fossil unavailability in CI environments.
- Attribution parity tempting a larger design than the guarantee requires.
- Architecture documents that drift from the implemented boundary.

## Success criterion

One interface, two backends, no duplicated SCM logic; all existing Git
scenarios still pass; Fossil scenarios pass where Fossil is available; no
Git-only claim remains in specs, ADRs, or the glossary.

## Vocabulary

- "seam", "backend", "port", "adapter", "candidate revision", "tested
  revision", "receipt", "constraint", "ADR", "fixture", "delta".

## Decision, need, and demand log

| Date | Entry | Source |
|---|---|---|
| 2026-09-14 | Decision: Frame confirmed (stakeholders, scope boundary, detection rule). | User |
| 2026-09-14 | Decision: three personas; audience dropped. | User |
| 2026-09-14 | Need: every decision stays visible to the user; no silent capture. | User |
| 2026-09-14 | Need: domain vocabulary decision (commit/worktree vs. revision/checkout) recorded before specs are written (Q10). | Frame, gaps G3/G8 |
