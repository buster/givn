# Design: release-0-9-9

## Overview

Release 0.9.9 is a release-owned maintenance change. `specs` is skipped
(`.givn-skip`), no product behavior changes, and the deliverables are the
version bump, the config marker, and the changelog entry naming the two
archived changes.

## Visual Design Contract

- Interface: none
- Viewports: not applicable
- Design system: not applicable
- Persona: developer--primary-user

## Use-Case Traceability

No use-case delta is authored. The released behavior is owned by the archived
`bound-review-context` and `default-all-addons` changes and their permanent
specifications.

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock`; no new dependency |

## Decisions

No proposal question is open. The release carries no behavior decision.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| N/A (specs skipped) | N/A | N/A |

Test runner command:

```text
./run-tests.sh --non-e2e
```

Single-scenario run command: N/A; no scenario is added.

## Internal Primitives

None. The change touches versioned data and documentation only.

## Strict Mode

Inherited unchanged: `tests/features_runner.rs` calls `.fail_on_skipped()`.

## Local Runnability & Digital Twins

Local run command:

```text
cargo run -- instructions
```

No external dependency, database, container, or digital twin is involved.

## E2E Smoke Test Infrastructure

No E2E scenario is added or changed. The existing corpus runs unchanged.

## Architecture Impact

| File | Change |
|---|---|
| `Cargo.toml` | `version = "0.9.9"` |
| `Cargo.lock` | workspace package `givn` version 0.9.9 |
| `givn/config.yaml` | `givn_config_version: "0.9.9"` |
| `CHANGELOG.md` | new `## [0.9.9] - 2026-10-29` section naming both archived changes |

No production Rust code changes. The migration catalog is unchanged; the 0.9.0
bundle covers later 0.9.x patch releases, so 0.9.2 to 0.9.9 is a migration-free
patch route.

## Data Model Changes

No schema change. The config version marker advances from 0.9.2 to 0.9.9.

## Coverage process boundaries

No new process. The existing coverage scripts and paths are unchanged.

## Open Technical Questions

None.
