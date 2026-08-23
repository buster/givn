# Changelog

All notable changes to Givn are documented here. Version 0.2.0 refers to the
master branch that is the source baseline for this release. Version 0.3.0
contains the complete stacked change set that follows that baseline.

## [0.3.0] - 2026-08-23

### Added

- Added deterministic overlap analysis to `givn lint`, including duplicate
  title errors, structural shape/subset findings, long-scenario warnings,
  machine-readable JSON output, review dispositions, and net-delta receipts.
- Added optional local semantic retrieval with `givn spec index`, `search`,
  `explore`, `review`, and `duplicates` surfaces. Permanent index entries are
  reused only when their complete serialized content still matches.
- Added complete Gherkin semantic serialization, project-relative identities,
  active-change exploration, change-local semantic reports, projected
  remediation verification, and deterministic evidence hashes.
- Added optional BGE and NLI evidence paths with independent pools, explicit
  recommended/filtered/unscored/unavailable states, actual-tokenizer limits,
  and visible token-cap evidence. These paths never assign Givn classifications
  or edit feature files automatically.
- Added canonical black-box-first and E2E interaction guidance. Generated
  instructions now delegate normative interaction policy to one resolved source
  instead of maintaining competing copies in every skill.
- Added repository-wide acceptance fixtures and rollback coverage for review,
  archive, semantic evidence, and generated project ownership boundaries.
- Added strict ADR qualification guidance, canonical artifact routing, and a
  consolidated Arc42 ADR register that preserves independent architecture
  history without retaining non-ADR records.
- Added the LLM migration playbook at
  [`docs/migrations/0.2.0-to-0.3.0.md`](docs/migrations/0.2.0-to-0.3.0.md).

### Changed

- `givn check review --change <id>` now runs the structural overlap gate and,
  in retrieval-enabled builds, the blocking semantic review gate.
- `givn archive` rechecks the blocking semantic gate immediately before
  modifying permanent specifications and refreshes the permanent retrieval
  index after a successful archive.
- Retrieval remains feature-gated. A feature-free binary reports explicit
  retrieval unavailability instead of silently passing a semantic gate.
- Generated review, design, task, and skill guidance now requires concrete
  evidence for strict runners, real-interface E2E tests, overlap decisions,
  semantic remediation, and architecture conformance.
- Arc42 documentation is consolidated into durable chapter files and a
  one-file-per-decision ADR register. `ADR-023` is refined by the 0.3.0
  migration protocol.
- The package and CLI version are now `0.3.0`.

### Migration

Projects with `givn/config.yaml` at version `0.2.0` receive an automatic,
reviewable preparation when they run the 0.3.0 `givn upgrade` command in a
clean Git worktree. Givn creates a managed upgrade commit, then a separate
maintenance migration change that records the first commit and contains the
LLM prompts for project-specific work. The migration guide is the entry point:

```text
docs/migrations/0.2.0-to-0.3.0.md
```

The automatic preparation never changes domain specifications, source code,
ejected facets, durable Arc42 facts, or user-authored files. It refuses dirty
or non-Git projects before writing and never stages the complete worktree.

### Verification Boundary

The release includes the retrieval and second-stage evidence implementation,
but proposed `ADR-063` and the active `semantic-reranker-budget` change remain
explicitly incomplete until their own verification and review gates pass.
Release distribution binaries, Homebrew, and shell-installer channels remain
planned work.

## [0.2.0]

The master-branch baseline for the 0.3.0 migration. See the repository history
for the complete 0.2.0 feature set.
