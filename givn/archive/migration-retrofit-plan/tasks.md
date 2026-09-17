# Tasks: migration-retrofit-plan

Maintenance change amending the 0.8.0 migration bundle, its guide, the
aggregate-plan wrapper, and the durable chapter sentences. `specs` is
skipped; no version bump. The settled execution model is recorded in the
change design.

## Setup

- [x] No new runner or step definitions: `specs` is skipped and the existing
      suite is inherited unchanged. Design `## Step Definitions` declares
      N/A.
- [x] The seven operator decisions are recorded in the proposal and the
      design; no decision is re-opened.

## Deliverables

### Bundle and guide

- [x] `assets/migrations/0.7.0-to-0.8.0/proposal.md` reframed to the
      inventory-and-plan model with the seven rules and the two-level
      completion boundary.
- [x] `assets/migrations/0.7.0-to-0.8.0/design.md` gains `## Execution model`
      and `## Inventory and dispositions` (pinned columns, id format,
      in-flight definition); the phases become inventory passes with a
      per-phase execution-model line; the ADR verdict/repair split, the
      proposed-ADR owner rule, the term-evidence exception, and the
      validation checks are recorded.
- [x] `docs/migrations/0.7.0-to-0.8.0.md` states the seven rules and rewrites
      Prompts 1-6 to inventory and name follow-ups, with the two-level
      completion boundary.
- [x] COMMIT: `77c92103` -- `feat(migration-retrofit-plan): amend the 0.8.0 bundle and guide to the inventory-and-follow-up model`

### Wrapper and contract docs

- [x] `src/cli/migration.rs` wrapper strings defer to each bundle's own
      completion rules instead of asserting in-change completion.
- [x] `docs/migrations/README.md` and the `CHANGELOG.md` 0.8.0 entry describe
      the amended model.
- [x] COMMIT: `cd4e82f3` -- `docs(migration-retrofit-plan): align the wrapper and contract documentation`

### Durable chapters

- [x] `docs/arc42/02-architecture-constraints.md`,
      `docs/arc42/06-runtime-view.md`, and
      `docs/arc42/07-deployment-view.md` reword the old-model sentences.
- [x] COMMIT: `a29b196a` -- `docs(migration-retrofit-plan): plan proposal, design, design-review, tasks, and arc42 impact` (the chapter edits landed with the planning commit)

## Verification

- [x] `cargo check --locked`, `cargo fmt`, and
      `cargo clippy --workspace --all-targets --locked -- -D warnings` clean.
      Evidence:
      ```
      Finished `dev` profile [unoptimized + debuginfo] target(s)
      ```
- [x] `cargo test --locked --lib migration` passes (the wrapper change does
      not break the catalog or route tests). Evidence:
      ```
      test result: ok. 11 passed; 0 failed
      ```
- [x] Aggregate smoke test on a scratch 0.7.0 project: the generated
      maintenance change design contains the amended sections and wrapper
      sentence, with no unsubstituted `{{`. Evidence:
      ```
      ## Execution model: 1
      ## Inventory and dispositions: 1
      follow-up change: 7
      ## Phase 1 — Use-case model: 1
      unsubstituted: 0
      Work the following phases strictly in sequence. Follow each bundle's own completion rules; a bundle may direct that the repairs run as named follow-up changes. The complete sequence is also recorded in `migration.yaml`.
      ```
- [x] Full suite (`./run-tests.sh`) and e2e suite (`./run-tests.sh --e2e`)
      → zero exit. Evidence:
      ```
      Summary [31.897s] 539 tests run: 539 passed, 0 skipped
      Summary [5.575s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] Archive: the maintenance change archives with `specs` skipped.

## Post-merge maintainer action (not part of this change)

The `v0.8.0` tag remains a post-merge maintainer action; this amendment is
part of the untagged 0.8.0 release.
