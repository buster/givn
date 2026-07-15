# Design: integrity-hook-and-archive-gate

## Overview

Add three enforcement mechanisms to givn's workflow:

1. A built-in `integrity` hook that checks tasks.md for non-placeholder commit hashes, design.md Step Definitions for existing non-empty files, and arc42 chapter files for evidence of editing — runnable via `givn check review`.
2. A post-archive verify gate that re-runs verify.command against the merged permanent specs before finalising archive, with a full backup and no auto-restore.
3. A `## Internal Primitives` section in the design.md template and a corresponding sign-off item in the review.md template.

## Technology Decisions

- **Language**: Rust (givn's existing language).
- **Test runner**: `cargo test` (givn's existing command). The `whatever` feature runner executes `.feature` files as an integration test.
- **Strict mode**: cucumber-rs `.fail_on_skipped()` — already present in givn's feature runner (`.fail_on_skipped()` is wired in the test code).
- **Gherkin library**: `gherkin` crate + cucumber-rs (givn's existing stack).
- **Config format**: YAML through serde (givn's existing config stack).

## Step Definitions

| Capability | Step definition file |
|---|---|
| givn-workflow-enforcement | `tests/steps/workflow_enforcement_steps.rs` |

The step definitions execute `cargo run -- <args>` against the built givn binary, creating temporary directories with fixture file structures, running the givn CLI, and asserting on exit code, stdout, stderr, and filesystem state.

### Test runner command

`cargo test --test features_runner -- --test-threads=1`

### Single-scenario run command

`cargo test --test features_runner <title> -- --exact`

### Strict mode

cucumber-rs `.fail_on_skipped()` is already configured in the existing feature runner. No additional mechanism needed.

## Internal Primitives

| Primitive | Type | Purpose |
|---|---|---|
| Project manifest | `Manifest` (embedded in givn binary) | Holds artifact definitions, verify command, addon states. The `integrity` hook reads manifest.addons.arc42 to decide whether to run arc42 checks. |
| Embedded asset store | `Assets` (from `rust-embed`) | Resolves template bytes for arc42 chapter comparison. Used directly by the integrity hook. |
| GIVN_FEATURES env var | String | Environment variable set by the hook runner to scope features path. Not accessed by step definitions directly. |

These are internal to the integrity hook's implementation and are not accessed by any scenario step definition.

## Strict Mode (mandatory)

cucumber-rs `.fail_on_skipped()` — already present in the existing features runner (`tests/features_runner.rs`). The step definitions will never leave a step body empty; pending steps use `unimplemented!()`.

### Not-implemented stub

```rust
unimplemented!("TODO")
```

## Local Runnability & Digital Twins

This is a CLI-only tool with no network dependencies. The givn binary is built with `cargo build` and invoked via `cargo run -- <args>`. No external services, no database, no docker.

### Local run command

```
cargo run -- <args>
```

### Isolated network

No external dependencies. Tests use temporary directories (via `tempfile::tempdir()`) for fixture scaffolding.

### Digital twins

None. givn has no third-party external dependencies.

### Anticipated interface obstacles and their fix

None. The interface is the CLI (subprocess invocation), which is straightforward to drive from step definitions.

## E2E Smoke Test Infrastructure

### Interface type

CLI-only. The real interface is the `givn` binary invoked as a subprocess.

### E2E runner command

No e2e runner needed. All scenarios in this change's spec exercise the CLI interface directly and are tagged as non-e2e (executed via `verify.command`, not `verify.e2e_command`).

The only scenario that involves `givn archive` — the post-archive gate — is non-e2e: it exercises the CLI and asserts on exit code + filesystem state, which is the same mechanism as every other givn scenario. Running it in-process (via the features runner) is the real interface.

### Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| User runs `givn check review` — integrity findings for missing commit hashes | (non-e2e) | CLI | Subprocess + filesystem assertions |
| User runs `givn check review` — integrity findings for missing step-def files | (non-e2e) | CLI | Subprocess + filesystem assertions |
| User runs `givn check review` — arc42 unedited chapter detection | (non-e2e) | CLI | Subprocess + filesystem assertions |
| User runs `givn archive` — verify gate passes | (non-e2e) | CLI | Subprocess + filesystem assertions |
| User runs `givn archive` — verify gate blocked by failing merged specs | (non-e2e) | CLI | Subprocess + filesystem assertions |
| User writes design.md with Internal Primitives section | (non-e2e) | CLI | Template output assertion |

No `@e2e` scenarios. The CLI is the real interface and every scenario drives it via subprocess invocation — the same mechanism an actual user would use. Adding a browser-based `@e2e` test for a CLI tool would be theater.

## Architecture Impact

### New files

- `src/integrity_hook.rs` — the `run_integrity_hook` function implementing all three checks (hash presence, promised-files-exist, arc42 conformance).

### Modified files

- `src/hooks.rs` — add `"integrity"` match arm in `run_artifact_hooks` dispatching to `run_integrity_hook`.
- `src/cli/archive.rs` — add snapshot + post-merge verify gate before the file move.
- `assets/default-manifest.yaml` — add `"integrity"` to `review.post`.
- `assets/templates/tasks.md` — change COMMIT box to inline format.
- `assets/templates/design.md` — add `## Internal Primitives` section.
- `assets/templates/review.md` — add sign-off item for internal primitives + arc42 slug-mapping instruction.

### Existing modules affected

- `src/hooks.rs` — new built-in, reuses `run_shell_command` for the post-archive verify gate.
- `src/gherkin/merge.rs` — no changes needed; `merge_file` already writes atomically. The archive gate wraps around the existing merge loop.

## Data Model Changes

None. The manifest structure (`default-manifest.yaml`) adds no new fields — the `integrity` hook is a built-in name resolved by `hooks.rs`, not a configurable command string. The addons state (including `arc42`) is already in the manifest.

## Open Technical Questions

None. All decisions were resolved during the grill process.
