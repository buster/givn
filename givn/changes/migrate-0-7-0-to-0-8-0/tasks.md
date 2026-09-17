# Tasks: migrate-0-7-0-to-0-8-0

Release-owned maintenance change: `specs` is skipped, no product behavior
changes. The deliverable is the 0.8.0 version bump plus the
`migrate-0-7-0-to-0-8-0` catalog bundle for the use-case, persona, and ADR
model hardening. No Persona is confirmed. Existing guarantees stay in force:
managed upgrades preserve user-owned files, never derive migration decisions
from Rust conditionals, and the maintenance LLM owns project-specific
decisions.

## Setup

- [x] No new runner or step definitions: `specs` is skipped and the existing
      suite (`./run-tests.sh`, `.fail_on_skipped()` strict mode) is inherited
      unchanged. Design `## Step Definitions` declares N/A.
- [x] Version surface confirmed: `env!("CARGO_PKG_VERSION")` feeds
      `givn init`, `{{GIVN_VERSION}}`, and the `givn upgrade` target version,
      so the Cargo bump is the single version source.

## Release deliverables

### Version bump to 0.8.0

- [x] `Cargo.toml`, `Cargo.lock`, and `givn/config.yaml` at 0.8.0;
      `CHANGELOG.md` folds `## [Unreleased]` into `## [0.8.0] - 2026-09-17`
      with the multi-SCM work and the model-hardening entries. Evidence:
      ```
      cargo check --locked
      Finished `dev` profile [unoptimized + debuginfo] target(s)
      givn --version
      givn 0.8.0
      ```
- [x] COMMIT: `9c8694bf` -- `feat(migrate-0-7-0-to-0-8-0): release 0.8.0 with the model-hardening migration bundle`
- [x] Formatting and lint on the 0.8.0 rule changes: `cargo fmt` and
      `cargo clippy --workspace --all-targets --locked -- -D warnings` clean.
      COMMIT: `4b7373f5` -- `style: apply rustfmt and clippy to the 0.8.0 rule changes`

### Catalog entry and route test

- [x] `assets/migrations/catalog.yaml` registers `migrate-0-7-0-to-0-8-0`
      (source 0.7.0, target 0.8.0, order 60, both asset paths).
- [x] `src/cli/migration.rs` test asserts
      `migration_path("0.7.0", env!("CARGO_PKG_VERSION"))` returns exactly
      `migrate-0-7-0-to-0-8-0`. Evidence:
      ```
      cargo test --locked --lib migration
      test result: ok. 11 passed; 0 failed
      ```
- [x] COMMIT: `ee39051f` -- `feat(migrate-0-7-0-to-0-8-0): catalog the 0.7.0 to 0.8.0 route and ship the bundle`

### Consumer bundle

- [x] `assets/migrations/0.7.0-to-0.8.0/proposal.md`: problem, solution,
      out-of-scope, completion boundary.
- [x] `assets/migrations/0.7.0-to-0.8.0/design.md`: the five ordered phases
      (use-case model with the inlined rules and R1-R10, personas, ADRs,
      smaller adoptions, overrides), the enforcement-boundary paragraph, the
      placeholder use (`{{SOURCE_VERSION}}`, `{{TARGET_VERSION}}`,
      `{{UPGRADE_COMMIT}}`), and the per-phase validation steps. No
      `{{MIGRATION_GUIDE}}`.
- [x] COMMIT: `ee39051f` -- `feat(migrate-0-7-0-to-0-8-0): catalog the 0.7.0 to 0.8.0 route and ship the bundle`

### Documentation

- [x] `docs/migrations/0.7.0-to-0.8.0.md` is the human-facing guide:
      the 0.8.0 contract in project-visible terms, the migration contract
      (clean checkout, marker, aggregate change), and the phase-by-phase
      prompt pack.
- [x] `docs/migrations/README.md` names the current bundle instead of the
      stale 0.3.0-to-0.4.0 claim.
- [x] COMMIT: `7b227a3a` -- `docs(migrate-0-7-0-to-0-8-0): publish the 0.7.0 to 0.8.0 migration guide`

## Verification

- [x] `cargo check --locked` passes and `givn --version` prints 0.8.0.
- [x] Scratch-project smoke test: `givn init --no-addons`, marker rewritten
      to 0.7.0, committed, `givn upgrade` → exit 0. Evidence:
      ```
      managed upgrade commit (A): e2a1c56cb5a359c8415ade5082a9e0dd0f72c697
      aggregate migration commit (B): 3d3959750c4f463779cf4e76e1c9401e9196caa2
      migration change: givn/changes/migrate-0-7-0-to-0-8-0
      migration phases: migrate-0-7-0-to-0-8-0

      ls givn/changes/migrate-0-7-0-to-0-8-0/ → design.md migration.yaml proposal.md
      design.md: 6 phase headings, 0 unsubstituted "{{"
      migration.yaml: sequence 1, id migrate-0-7-0-to-0-8-0
      second `givn upgrade`:
      Migration change 'migrate-0-7-0-to-0-8-0' already exists; project is already prepared for migration.
      ```
- [x] Full suite (`./run-tests.sh`) and e2e suite (`./run-tests.sh --e2e`)
      → zero exit. Evidence:
      ```
      Summary [31.682s] 539 tests run: 539 passed, 0 skipped
      Summary [5.657s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] Archive: the maintenance change archives with `specs` skipped; the
      verification scopes run against the permanent corpus.

## Post-merge maintainer action (not this change)

- [ ] Annotated `v0.8.0` tag after merge, per the 0.6.0/0.7.0 precedent.
