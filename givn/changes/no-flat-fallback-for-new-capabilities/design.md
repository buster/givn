# Design: no-flat-fallback-for-new-capabilities

## Technical Approach

Replace the fallback arm of the merge-target decision with a hard error. In
`src/cli/archive.rs` `process_delta_file`, when the capability exists nowhere
in the permanent corpus and the proposal records no `NEW in <group>` row for
it, the archive is blocked:

```text
archive blocked: capability '<cap>' exists nowhere in the permanent corpus
and its proposal records no 'NEW in <group>' routing decision
```

The error surfaces before any file is written; the archive aborts with the
change still active. The existing-grouped arm and the retirement paths are
untouched.

## Step Definitions

All steps reuse existing bindings:

| Step | Defined in |
|---|---|
| `a givn project with verify.e2e_command unconfigured in givn/config.yaml` | `tests/steps/enforce_post_merge_e2e_gate_steps.rs` |
| `a completed change "..." whose proposal records no routing decision and whose delta lives under specs/.../` | `tests/steps/grouped_merge_steps.rs` (reused from predecessor) |
| `the project declares a complete run declaration` | `tests/steps/run_declaration_fixtures_support.rs` |
| `the user invokes \`givn archive --change <id>\`` | `tests/steps/mod.rs` |
| `the command exits non-zero` / `stderr contains "..."` / `the directory "..." does not exist` | `tests/steps/mod.rs` |
| `the change directory still exists under givn/changes/` | `tests/steps/workflow_enforcement_steps.rs` |

No new step file.

## Runner and Strict Mode

Existing contract (`./run-tests.sh`, `--e2e`, `.fail_on_skipped()`).
Single-scenario command:

```text
GIVN_FEATURES=givn/changes/no-flat-fallback-for-new-capabilities/specs cargo test --test features_runner -- --name '^Archive is blocked when a new capability has no declared routing decision$'
```

## Local Runnability and E2E

CLI subprocess in a temp project; no server, browser, database, or digital
twins. RED anchor: with the fallback still present the archive succeeds and
the `exits non-zero` assertion fails.

## Interaction Coverage Matrix

No new inventory entry: the blocked archive is a variant of the existing
archival action ("Consumer archives a new capability...") — the grouped
`@e2e` scenario remains the canonical e2e evidence; the blocked outcome is a
regular scenario. The delta removes the superseded flat-layout scenario and
adds the blocked one; the supersession is disposed in review.

## Coverage

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build | separate non-E2E/E2E Cobertura reports | `./merge-coverages.sh` | `cli::archive::process_delta_file` (blocked arm) |

## Version Freshness

No new dependency or tool; existing Rust toolchain and cucumber runner
unchanged.