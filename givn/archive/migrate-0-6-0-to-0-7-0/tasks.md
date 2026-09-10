# Tasks: migrate-0-6-0-to-0-7-0

Release-owned maintenance change: `specs` is skipped, no product behavior
changes. The deliverable is the 0.7.0 version bump plus the
`migrate-0-6-0-to-0-7-0` catalog bundle for the archive-proof contract. No
Persona is confirmed. Existing guarantees stay in force: managed upgrades
preserve user-owned files, never derive migration decisions from Rust
conditionals, and the maintenance LLM owns project-specific decisions.

## Setup

- [x] No new runner or step definitions: `specs` is skipped and the existing
      suite (`./run-tests.sh`, `.fail_on_skipped()` strict mode) is inherited
      unchanged. Design `## Step Definitions` declares N/A.
- [x] Version surface confirmed: `env!("CARGO_PKG_VERSION")` feeds
      `givn init`, `{{GIVN_VERSION}}`, and the `givn upgrade` target version,
      so the Cargo bump is the single version source.

## Release deliverables

### Version bump to 0.7.0

- [x] `Cargo.toml` and `Cargo.lock` at 0.7.0; `givn/config.yaml` marker
      0.7.0; `CHANGELOG.md` 0.7.0 section. Evidence: `cargo check --locked`
      passes and `givn --version` reports 0.7.0.
- [x] COMMIT: `53454c9e` -- `feat(migrate-0-6-0-to-0-7-0): release 0.7.0 with the archive-proof migration bundle`

### Catalog entry

- [x] `assets/migrations/catalog.yaml` registers `migrate-0-6-0-to-0-7-0`
      (source 0.6.0, target 0.7.0, order 50, both asset paths). Evidence:
      scratch-project upgrade resolved the route.

### Consumer bundle

- [x] `assets/migrations/0.6.0-to-0.7.0/proposal.md`: problem, solution,
      out-of-scope, completion boundary.
- [x] `assets/migrations/0.6.0-to-0.7.0/design.md`: preflight, scope
      inventory, the `GIVN_RESULT_FILE` contract with per-runner count
      sources, command routing, Git publication boundary, validation,
      blocking checks, forward-repair guidance, and the retained evidence
      file. Rules mirror `proof::RunnerResult::read`.
- [x] COMMIT: `53454c9e` -- `feat(migrate-0-6-0-to-0-7-0): release 0.7.0 with the archive-proof migration bundle`

### Documentation

- [x] `CHANGELOG.md` names the bundle; `docs/migrations/README.md` names the
      current bundle instead of the stale 0.3.0-to-0.4.0 claim; the glossary
      symbol points at `proof::RunnerResult::read`.
- [x] COMMIT: `31719f87` -- `fix(migrate-0-6-0-to-0-7-0): harden bundle guidance and aggregate titles`

### Aggregate renderer fix

- [x] `write_aggregate_migration_files` titles the generated proposal/design
      with the final change id instead of the `.tmp-<pid>` staging name;
      regression test `aggregate_titles_use_the_change_id_not_the_staging_dir`
      passes.
- [x] COMMIT: `31719f87` -- `fix(migrate-0-6-0-to-0-7-0): harden bundle guidance and aggregate titles`

## Verification

- [x] Route smoke test: a scratch 0.6.0 project ran `givn upgrade` with the
      0.7.0 binary and produced `givn/changes/migrate-0-6-0-to-0-7-0/` with
      Commit A/B, the aggregate plan containing the `GIVN_RESULT_FILE`
      procedure, and the 0.7.0 marker; a second run reported the change
      already exists.
- [x] `cargo test --locked --lib` includes the aggregate-title regression;
      `cargo test --locked --lib migration::tests::aggregate_titles` passes.
- [x] `cargo check --locked` passes after the version bump.
- [x] `givn lint --change migrate-0-6-0-to-0-7-0` is clean (no deltas).
- [x] Full suite `./run-tests.sh`: 415/415; E2E `./run-tests.sh --e2e`:
      79/79 (evidence in review.md).
- [x] `givn archive --change migrate-0-6-0-to-0-7-0` proves the maintenance
      change and records the receipt.
