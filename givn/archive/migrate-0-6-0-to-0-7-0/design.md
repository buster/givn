# Design: migrate-0-6-0-to-0-7-0

## Overview

Release 0.7.0 of the archive-proof contract plus the release-owned migration
bundle that carries a 0.6.0 project to it. This is a maintenance change:
`specs` is skipped (`.givn-skip`), no product behavior changes, and the
deliverables are versioned assets and documentation.

## Exact file changes

| File | Change |
|---|---|
| `Cargo.toml` | `version = "0.7.0"` |
| `Cargo.lock` | workspace package version 0.7.0 |
| `givn/config.yaml` | `givn_config_version: "0.7.0"` |
| `assets/migrations/catalog.yaml` | `migrate-0-6-0-to-0-7-0`, source `0.6.0`, target `0.7.0`, order 50, asset paths |
| `assets/migrations/0.6.0-to-0.7.0/proposal.md` | consumer-facing migration proposal (new) |
| `assets/migrations/0.6.0-to-0.7.0/design.md` | self-contained consumer migration procedure (new) |
| `CHANGELOG.md` | 0.7.0 section with the bundle and the breaking archive contract |
| `src/cli/migration.rs` | pass the final change id into the aggregate renderer so generated proposal/design titles no longer leak the staging directory name |

No Rust code changes. The catalog is data; the route resolves through the
existing `src/cli/migration.rs` logic, so no version-specific conditional is
added (the bundle contract forbids that).

## Bundle contract

The consumer bundle is the phase consumed by `givn upgrade`'s aggregate
maintenance change. It must be self-contained: an LLM in the consumer project
has no access to this repository.

- `proposal.md`: problem, solution, out of scope, and completion boundary.
- `design.md`: preflight (version/commit/Git boundary/commands), scope
  inventory, the `GIVN_RESULT_FILE` contract and per-runner count sources, the
  command routing rule, Git readiness, validation, blocking checks, and the
  single retained evidence file.
- Placeholders used: `{{SOURCE_VERSION}}`, `{{TARGET_VERSION}}`,
  `{{UPGRADE_COMMIT}}` (substituted by the existing upgrade renderer).
- The contract mirrors `proof::RunnerResult::read` exactly: scope
  `regular`/`e2e`, integer counts, `total > 0`, `failed == 0`, `skipped == 0`,
  `passed + failed + skipped == total`, fail-closed on missing or malformed
  results.
- Counts must come from the runner; the bundle explicitly forbids hardcoded
  or exit-code-inferred counts, and tells the LLM to stop and report when the
  runner has no machine-readable output.

## Version bump mechanics

`env!("CARGO_PKG_VERSION")` is the single source for `givn init`, the
`{{GIVN_VERSION}}` config placeholder, and the `givn upgrade` target version.
Bumping the package version therefore makes new projects write the 0.7.0
marker and makes 0.6.0 projects resolve the new catalog edge. The repository's
own `givn/config.yaml` marker is bumped explicitly with the release, matching
the 0.6.0 release change (`ca3825e2`).

## Verification

- Catalog validity: `givn upgrade` resolved the route in a scratch project at
  marker 0.6.0 and produced `givn/changes/migrate-0-6-0-to-0-7-0/` whose
  design contains the `GIVN_RESULT_FILE` procedure; Commit A/B hashes were
  recorded. A second `givn upgrade` reported the change already exists.
- The bundle assets are embedded; `cargo check --locked` validates the build
  and the lock. A scratch 0.6.0 project's aggregate proposal/design titles use
  the final `migrate-0-6-0-to-0-7-0` id, not a `.tmp-<pid>` staging name.
- The repository suite (`./run-tests.sh`) and the E2E scope
  (`./run-tests.sh --e2e`) run against the unchanged permanent corpus; the
  existing upgrade scenarios tolerate catalog additions by construction
  (`tests/steps/release_0_3_0_e2e_steps.rs`).
- Archive: the maintenance change archives with `specs` skipped; its
  verification scopes run against the permanent corpus.

## Step Definitions

N/A — no step definitions for this change.

## Strict Mode (mandatory)

Inherited unchanged: `GivnWorld::cucumber().fail_on_skipped()` in
`tests/features_runner.rs`. No new steps are added.

## Local Runnability & Digital Twins (mandatory)

N/A — no new runnable surface, no external dependency, no digital twin. The
verification path is the existing CLI plus the repository suite.

## Internal Primitives

None — no production structs, functions, or enums are introduced.

## Use-case traceability

| Proposal claim | Verification |
|---|---|
| Version and marker bumped to 0.7.0 | `Cargo.toml`, `Cargo.lock`, `givn/config.yaml`; `givn --version` |
| Catalog routes 0.6.0 → 0.7.0 | scratch-project `givn upgrade` smoke test; catalog entry |
| Bundle carries the consumer procedure | `assets/migrations/0.6.0-to-0.7.0/{proposal,design}.md`; aggregate design contains it |
| No behavior/spec change | `specs` skipped; suite green on the unchanged corpus |

## Open Technical Questions

None.
