# Tasks: migrate-0-7-0-to-0-8-0

Release-owned maintenance change: `specs` is skipped, no product behavior
changes. The deliverable is the 0.8.0 version bump plus the
`migrate-0-7-0-to-0-8-0` catalog bundle for the use-case, persona, and ADR
model hardening. No Persona is confirmed. Existing guarantees stay in force:
managed upgrades preserve user-owned files, never derive migration decisions
from Rust conditionals, and the maintenance LLM owns project-specific
decisions.

## Setup

- [ ] No new runner or step definitions: `specs` is skipped and the existing
      suite (`./run-tests.sh`, `.fail_on_skipped()` strict mode) is inherited
      unchanged. Design `## Step Definitions` declares N/A.
- [ ] Version surface confirmed: `env!("CARGO_PKG_VERSION")` feeds
      `givn init`, `{{GIVN_VERSION}}`, and the `givn upgrade` target version,
      so the Cargo bump is the single version source.

## Release deliverables

### Version bump to 0.8.0

- [ ] `Cargo.toml`, `Cargo.lock`, and `givn/config.yaml` at 0.8.0;
      `CHANGELOG.md` folds `## [Unreleased]` into `## [0.8.0] - <date>` with
      the multi-SCM work and the model-hardening entries. Evidence:
      `cargo check --locked` passes and `givn --version` reports 0.8.0.
- [ ] COMMIT: `<hash>` -- `feat(migrate-0-7-0-to-0-8-0): release 0.8.0 with the model-hardening migration bundle`

### Catalog entry and route test

- [ ] `assets/migrations/catalog.yaml` registers `migrate-0-7-0-to-0-8-0`
      (source 0.7.0, target 0.8.0, order 60, both asset paths).
- [ ] `src/cli/migration.rs` test asserts
      `migration_path("0.7.0", env!("CARGO_PKG_VERSION"))` returns exactly
      `migrate-0-7-0-to-0-8-0`. Evidence: `cargo test --locked --lib migration`
      passes.
- [ ] COMMIT: `<hash>` -- `feat(migrate-0-7-0-to-0-8-0): catalog the 0.7.0 to 0.8.0 migration route`

### Consumer bundle

- [ ] `assets/migrations/0.7.0-to-0.8.0/proposal.md`: problem, solution,
      out-of-scope, completion boundary.
- [ ] `assets/migrations/0.7.0-to-0.8.0/design.md`: the five ordered phases
      (use-case model with the inlined rules and R1-R10, personas, ADRs,
      smaller adoptions, overrides), the enforcement-boundary paragraph, the
      placeholder use (`{{SOURCE_VERSION}}`, `{{TARGET_VERSION}}`,
      `{{UPGRADE_COMMIT}}`), and the per-phase validation steps. No
      `{{MIGRATION_GUIDE}}`.
- [ ] COMMIT: `<hash>` -- `feat(migrate-0-7-0-to-0-8-0): ship the model-hardening migration bundle`

### Documentation

- [ ] `docs/migrations/0.7.0-to-0.8.0.md` is the human-facing guide:
      the 0.8.0 contract in project-visible terms, the migration contract
      (clean checkout, marker, aggregate change), and the phase-by-phase
      prompt pack.
- [ ] `docs/migrations/README.md` names the current bundle instead of the
      stale 0.3.0-to-0.4.0 claim.
- [ ] COMMIT: `<hash>` -- `docs(migrate-0-7-0-to-0-8-0): publish the 0.7.0 to 0.8.0 migration guide`

## Verification

- [ ] `cargo check --locked` passes and `givn --version` prints 0.8.0.
- [ ] Scratch-project smoke test: `givn init --no-addons`, rewrite
      `givn_config_version` to 0.7.0, commit, run `givn upgrade`; assert
      `givn/changes/migrate-0-7-0-to-0-8-0/{migration.yaml,proposal.md,design.md}`
      exist, the aggregate design contains the five phase headings and no
      unsubstituted `{{`, `migration.yaml` names the bundle id, a second
      `givn upgrade` reports the change already exists. Paste the output.
- [ ] Full suite (`./run-tests.sh`) and e2e suite (`./run-tests.sh --e2e`)
      → zero exit. Evidence pasted.
- [ ] Archive: the maintenance change archives with `specs` skipped; the
      verification scopes run against the permanent corpus.

## Post-merge maintainer action (not this change)

- [ ] Annotated `v0.8.0` tag after merge, per the 0.6.0/0.7.0 precedent.
