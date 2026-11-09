# Design: rewrite-communication-policy

## Overview

Replace the managed `communication_policy` block with an 11-rule, two-section
text: five tone rules that carry the existing presentation obligations, and six
documentation rules that name formats. The block drops per-document budgets,
documentation file names, and dialog-example restatement.

The asset `assets/addons/communication-policy/policy.md` is the single source.
`givn init`, `givn skills sync`, and `givn addons enable communication_policy`
write it between the `communication-policy` markers. This repository's
`AGENTS.md` carries the rendered copy, so both stay byte-identical.

## Visual Design Contract

- Interface: none
- Viewports: not applicable
- Design system: not applicable
- Personas: developer--primary-user, agent--technical-role

The change alters generated Markdown text. It adds no rendered screen.

## Use-Case Traceability

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| Communication rules govern concrete presentation and documentation formats, never repository layout | One bounded section per concern: `Tone` and `Documentation` | `enable-optional-addons`, new scenario |
| An enabled addon's guidance is written once and refreshed on sync | The asset stays the only source; `AGENTS.md` is its rendered copy | existing `Consumer enables the outline addon and it is listed as enabled`; rendering is unchanged |
| Generated guidance remains technology-agnostic | The block names formats, not repository files or budgets | absence assertions in the new scenario |
| The dialog obligation is independent of the addon | No dialog rule is copied into the block | existing `the mandatory dialog policy remains independent of the communication addon` |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock`; no new dependency |
| cucumber-rs | existing workspace pin | `.fail_on_skipped()` remains the strict-mode mechanism |

## Decisions

- Decides D1: the shipped block carries no README line budget — realized by
  omitting it from `assets/addons/communication-policy/policy.md` and asserting
  the absence in `tests/steps/default_all_addons_steps.rs`. The measurement is
  recorded in `proposal.md`.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `enable-optional-addons` | `tests/steps/default_all_addons_steps.rs` | same file |

Test runner command:

```text
./run-tests.sh --non-e2e
```

Single-scenario run command:

```text
GIVN_FEATURES=givn/changes/rewrite-communication-policy/specs cargo test --locked --test features_runner -- --name '^The communication block binds documentation to named artifacts$'
```

## Strict Mode

`tests/features_runner.rs` calls `.fail_on_skipped()`. Each new step asserts on
the rendered block and returns no early pass on missing text.

## E2E Smoke Test Infrastructure

- **Interface type:** CLI — generated Markdown read by the developer and the authoring agent.
- **E2E runner command:** `./run-tests.sh --e2e` (unchanged).
- **E2E step location:** `tests/steps/initialize_a_project_e2e_steps.rs` (existing file).
- **Local test infrastructure:** the existing `features_runner` spawns the compiled `givn` binary; no server, browser, or container.
- **E2E framework choice:** cucumber-rs with `GivnWorld`; it invokes the real CLI.
- **E2E strict-mode proof:** `.fail_on_skipped()` in `tests/features_runner.rs`.
- **Interaction Coverage Matrix:** the new scenario is a variant of the existing enabling action; addon enablement normalizes to one inventory entry, so no entry and no new `@e2e` scenario are added.

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| enable an addon (all addon enablement normalizes here) | Consumer enables the outline addon and it is listed as enabled | CLI | spawn `givn addons enable outline` |

## Architecture Impact

| File | Change |
|---|---|
| `assets/addons/communication-policy/policy.md` | Replace the block with 11 rules in two sections |
| `AGENTS.md` | Render the same block, byte-identical |
| `src/cli/addons.rs` | Addon description names a presentation and documentation block |
| `tests/steps/default_all_addons_steps.rs` | New block assertions; title assertion updated |
| `givn/specs/initialize-project/usecase.md` | Rule wording covers documentation formats and rejects repository layout |
| `docs/arc42/02-architecture-constraints.md` | Addon scope wording |
| `docs/arc42/03-context-and-scope.md` | Fenced-block wording |
| `docs/arc42/06-runtime-view.md` | Sync-step wording |
| `docs/arc42/08-crosscutting-concepts.md` | Guidance-ownership wording |
| `docs/glossary.md` | Communication policy definition |
| `docs/migrations/README.md` | Verification-step wording |

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| `CommunicationPolicyBlock` | initialize-project | `givn init`, `givn skills sync`, `givn addons enable` | `assets/addons/communication-policy/policy.md` rendered by `cli::addons` | a second policy block with its own marker set or per-project variants |

## Data Model Changes

None. The block is embedded text; no schema or persisted state changes.

## Coverage process boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| `features_runner` | `./run-tests.sh` | existing coverage build | `coverage/non-e2e-cobertura.xml` | `./merge-coverages.sh` | new step assertions execute against the rendered block |

## Open Technical Questions

None.
