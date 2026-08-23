# Proposal: release-0-3-0

## Problem / Opportunity

Projects created with Givn 0.2.0 are not given a complete, reviewable path to
the current Givn behavior. The current stack changes the authoring rules,
review gates, semantic evidence workflow, generated agent guidance, and Arc42
architecture records. Updating only the configuration marker leaves a project
partly on the old contract and forces each team to reconstruct the migration
from a large repository diff.

An upgrade also needs a trustworthy Git boundary. If it silently mixes generated
Givn files with a project's uncommitted work, the resulting migration cannot be
reviewed or safely reproduced. If it commits the whole worktree, it can publish
files that Givn does not own.

## Proposed Solution

Release Givn as version 0.3.0 with a changelog and a migration guide for every
project whose `givn/config.yaml` identifies version 0.2.0. The guide is written
as an executable prompt sequence for an LLM working inside the project. It
directs the LLM to inspect the real project, update only applicable surfaces,
resolve newly exposed specification and architecture findings, and verify the
result before completion.

When a project runs `givn upgrade`, Givn refreshes its managed scaffold and
regenerates its managed agent integrations. If the project is under Git, this
machine-safe portion is committed separately and only known Givn-owned paths
are staged. For a release with no project migration, this is the complete
operation and no maintenance change is created. For the 0.2.0-to-0.3.0
compatibility boundary, a visible maintenance change named
`migrate-0-2-0-to-0-3-0` is then created with the upgrade commit recorded. The
change contains a detailed proposal and design, deliberately skips a product
feature specification, and leaves design review, task derivation,
implementation, review, and archive to the project LLM and its normal Givn
gates.

The operation is idempotent. Existing user changes are never staged or
overwritten, an existing active or archived migration is never duplicated, and
the migration plan never claims that human or LLM review has already passed.

## Out of Scope

- Automatically changing a project's domain specifications, step definitions,
  source code, tests, ADR decisions, Arc42 facts, or user-authored documents.
- Automatically refreshing ejected templates, instructions, skills, or commands
  that are project-owned overrides.
- Adding a synthetic migration feature to the project's permanent
  `givn/specs/` tree.
- Automatically committing unrelated files or using `git add -A`.
- Supporting arbitrary unknown version jumps in this release. The documented
  migration route is exactly 0.2.0 to 0.3.0.
- Deciding from SemVer alone that a future release needs or does not need a
  migration. Each release must declare its migration policy.
- Publishing release binaries or package-manager integrations that are not
  already part of this repository.

## Open Questions

- A project without Git cannot receive the requested automatic commit. The
  upgrade must report this as an actionable error rather than pretending that
  the migration commit exists.
- Projects with a dirty worktree must be refused before any upgrade write, so
  the user can create a clean migration boundary first.
