# Proposal: migrate-0-6-0-to-0-7-0

## Use-Case Context

- Use-case ID: none (release-owned maintenance change; `specs` is skipped)
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

Publishing archives forward-only with a verification receipt is a breaking
change for existing projects:

- `givn archive` now requires a Git worktree whose root is the project root
  and a committed project state.
- Every verification scope must write a machine-readable result
  (`GIVN_RESULT_FILE`) with reconciling scenario counts; a missing, failed,
  skipped, zero, or non-reconciling result blocks proof.
- `givn check review` no longer executes the test suite; the archive
  verification receipt is the execution evidence.

`givn upgrade` refreshes managed config and guidance automatically, but the
runner scripts are project-owned: without a release migration a 0.6.0 project
upgrades to the new binary with no route or instructions to update them, and
its next archive fails closed. The binary version and the project's
`givn_config_version` marker also still claim 0.6.0.

## Proposed Solution

Release 0.7.0 with a release-owned migration bundle:

- Bump the package version (`Cargo.toml`, `Cargo.lock`), the project version
  marker (`givn/config.yaml`), and the changelog.
- Register `migrate-0-6-0-to-0-7-0` (order 50, source 0.6.0, target 0.7.0) in
  `assets/migrations/catalog.yaml`.
- Add the self-contained bundle
  `assets/migrations/0.6.0-to-0.7.0/{proposal.md,design.md}`: inventory the
  project's `verify.command`/`verify.e2e_command`, make each scope emit the
  fail-closed runner result without fabricating counts, make the project a Git
  worktree at its root with a committed state, update the review expectation
  to the receipt model, and verify through the next archive.
- Document the release and the consumer action in `CHANGELOG.md` and the
  migration documentation.

The machine-safe preparation (managed config/guidance refresh and the
`givn_config_version` bump) stays with `givn upgrade` and Commit A; the bundle
carries only project-specific decisions.

## Out of Scope

- Rewriting project behavior, specifications, or test scenarios.
- Automating project-owned runner scripts from Givn (the maintenance LLM
  updates them and a human confirms the result contract).
- Backfilling verification receipts for legacy archives; they remain
  historical and are never re-published.

## Open Questions

- None.
