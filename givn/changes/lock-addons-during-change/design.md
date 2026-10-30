# Design: lock-addons-during-change

## Overview

The change adds a change-local add-on lock and three enforcement points:

1. `givn new` records the effective add-on values.
2. `givn addons enable|disable` refuses while a locked change is active.
3. `givn check review` and `givn archive` fail on drift.

A forced init that would change add-ons during a locked change is refused before writing. An interactive operator override with a reason updates the recorded values and appends an override entry.

## Visual Design Contract

- Interface: terminal
- Viewports: not applicable
- Design system: not applicable
- Persona: developer--primary-user, agent--technical-role, givn-maintainer--owner

The visible surface is CLI text. The override scenario drives a real PTY. No browser screen is rendered.

## Use-Case Traceability

Owning use case: `initialize-project`; supporting owner: `verify-change`.

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| Initialization records policy | `givn new` writes `config-lock.yaml` | `enable-optional-addons`, `A new change records the operator add-on configuration` |
| Add-on mutation is operator-owned | `givn addons` guard refuses during a locked change | `enable-optional-addons`, `An active change refuses add-on mutation` |
| A missing dependency is an environment failure | Refusal names isolation/reporting guidance | same scenario |
| Operator intent is recorded | `--operator --reason` on a TTY appends an override | `enable-optional-addons`, `An operator override on a real terminal is recorded` |
| Review detects drift | `check review` compares the lock | `givn-workflow-enforcement`, `Review rejects add-on configuration drift` |
| Archive detects drift | archive static gate calls the same check | code path covered by the same check; review scenario is the executable evidence |
| Forced init cannot silently weaken policy | `init --force` guard checks the proposed add-on set | `enable-optional-addons`, forced-init scenarios |
| Existing changes remain usable | missing lock is skipped | unit test `a_lock_is_optional_for_old_changes` |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock` |
| serde_yaml_ng | existing dependency | Lock file serialization |
| util-linux `script` | existing acceptance prerequisite | Real PTY for the operator override test |

No new dependency.

## Decisions

- Decides D1: automatic lock for every new change — `givn new` writes the lock.
- Decides D2: `givn/changes/<id>/config-lock.yaml` with `version: 1`, `addons`, and `overrides`.
- Decides D3: `--operator --reason` requires a TTY and records an override entry.
- Decides D4: missing lock is skipped; deliberate deletion is a documented residual limit.
- Decides D5: `init --force` refuses only when the proposed add-on set differs from the recorded one.

## Architecture Impact

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Add-on lock | initialize-project, verify-change | `givn new`, `givn addons`, `givn init --force`, review, archive | `src/config/lock.rs` | The lock covers non-addon configuration or requires cryptographic protection |
| Add-on mutation refusal | initialize-project | `givn addons enable|disable` | `src/cli/addons.rs` guard | A second mutation command appears |
| Review/archive drift gate | verify-change | `givn check review`, `givn archive` | `src/cli/check.rs`, `src/cli/archive.rs` | A non-static verification path needs the check |

### Module changes

- `src/config/lock.rs`: lock format, effective values, proposed init values, differences, read/write, override recording.
- `src/cli/new.rs`: write the lock after creating the change directory.
- `src/cli/addons.rs`: guard enable/disable, TTY/reason override, record override after mutation.
- `src/cli/init.rs`: refuse a forced init that would change the add-on set.
- `src/cli/check.rs`: add the drift check to review failures.
- `src/cli/archive.rs`: add the drift check to static review gates.
- `src/main.rs`: add `--operator`, `--reason`, and `--change` to the add-on commands.
- `tests/common/mod.rs`: generic PTY command helper.
- `tests/steps/default_all_addons_steps.rs`: lock assertions and the PTY override step.

No lifecycle artifact, manifest, or JSON schema changes.

## Data Model Changes

`config-lock.yaml` version 1:

```yaml
version: 1
addons:
  arc42: true
  coverage: true
  ...
overrides:
  - addon: coverage
    from: true
    to: false
    reason: dependency unavailable
    timestamp: "..."
```

The file is change-local and archived with the change. It is not a lifecycle artifact.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `enable-optional-addons` | `tests/steps/default_all_addons_steps.rs` | same file |
| `givn-workflow-enforcement` | `tests/steps/workflow_enforcement_steps.rs` | same file |

Test runner command: `./run-tests.sh --non-e2e`.

Strict mode is unchanged. The override scenario uses the existing `script` PTY helper.

## Interaction Coverage Matrix

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `enable-optional-addons` — enable/disable an add-on | Existing row | Existing add-on E2E | Real `givn addons` subprocess; lock scenarios are regular variants |
| `givn-workflow-enforcement` — request review checks | Existing row | Existing review E2E | Real `givn check review` subprocess |

No new E2E action is added.

## External Boundary Cases

| Boundary | Adversarial case | Handling |
|---|---|---|
| Active change | add-on command without override | refuse before writing |
| Operator override | non-TTY, empty reason, several locked changes | refuse with the specific missing condition |
| Direct edit | `coverage: true -> false` | review/archive fail with addon and values |
| Forced init | `--no-addons` during a locked change | refuse before writes |
| Forced init | preserving add-ons | allowed |
| Old change | no lock file | skipped |
| Lock deletion | file removed | documented residual limit; no marker in 0.9.16 |

## Local Runnability

No server or external service. The PTY test requires util-linux `script`, already an acceptance prerequisite.

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Feature runner | `./run-tests.sh --non-e2e` | `features_runner` and `givn` | `target/llvm-cov-target` | `merge-coverages.sh` | lock guard and check paths |
| PTY command | `script -qec` inside the lock scenario | same `givn` binary | same profile set | same merge | override recording |

Measured population: production source under `src/`.

## Out-of-Scope Technical Decisions

- No cryptographic lock or signed config.
- No lock-deletion marker.
- No configuration-change lifecycle.
- No dependency installation.
