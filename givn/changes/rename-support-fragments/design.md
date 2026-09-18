# Design: rename-support-fragments

## Overview

Rename fourteen fragment capabilities and their files, update the fragment
narrative, and repair one test path. No behavior changes; `specs` is
skipped.

## Exact file changes

| Path | Change |
|---|---|
| `givn/specs/fragments/<old>.feature` (14) | Move to `<new>.feature`; title updated |
| `givn/specs/fragments/fragment.md` | Capability list carries the new names; `customize-the-lifecycle-graph` is owned |
| `tests/steps/adr_qualification_gate_steps.rs` | The register-history step names `find-existing-scenarios.feature` |

## Verification

- `givn lint` exits 0; no old capability name remains in `givn/specs/`.
- `cargo fmt` and `cargo clippy --workspace --all-targets --locked -- -D
  warnings` clean.
- The suite and e2e pass with the renamed scenarios.

## Step Definitions

No new steps; one hard-coded fixture path in an existing step is updated.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — specification reorganization and one test path.

## Internal Primitives

None.

## Use-case traceability

- Use-case ID: the `support` fragment
- Affected capabilities: the fourteen renamed ones
- Confirmed Personas: none (no user-visible surface)

## Open Technical Questions

None.
