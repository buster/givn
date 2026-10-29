# Design: default-all-addons

## Overview

Make every registered add-on enabled by default. The embedded manifest, the
serde defaults, and `givn init` agree on the same posture. An explicit `false`
is the only opt-out, and `givn init --no-addons` writes that false for every
known add-on so the choice is visible in `givn/config.yaml`. A partial
`addons:` block changes only the keys it names.

## Visual Design Contract

- Interface: none
- Viewports: not applicable
- Design system: not applicable
- Persona: developer--primary-user

The change adds line-oriented CLI output and config text. It adds no rendered
screen.

## Use-Case Traceability

Owning use cases: `initialize-project` (capability `enable-optional-addons`)
and `assess-code-structure` (capability `outline-report`).

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| A fresh project receives the default add-ons | Every `AddonSpec.default_enabled` is true and `givn init` enables each one | `enable-optional-addons` scenarios |
| A project can opt out | `givn init --no-addons` writes explicit false values; `givn addons disable` remains per-add-on | `enable-optional-addons`, `outline-report` scenarios |
| A partial addons block changes only named keys | `Addons` serde fields default to true; the overlay keeps the parsed block | `a partial addons block keeps unnamed addons enabled` |
| The outline report is available by default | `addons.outline` follows the enabled default; the report gate is unchanged | `The outline experiment is enabled by default` |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock`; no new dependency |
| cucumber-rs | existing workspace pin | `.fail_on_skipped()` remains the strict-mode mechanism |

## Decisions

- Decides D1: a missing key in an explicit `addons:` block means enabled —
  realized by `enabled_by_default()` on every serde field and by the explicit
  false values written by `givn init --no-addons`.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `enable-optional-addons` | `tests/steps/default_all_addons_steps.rs` | same file |
| `outline-report` | `tests/steps/outline_report_steps.rs` | same file |
| `initialize-project` | `tests/steps/mod.rs` | same file |

Test runner command:

```text
./run-tests.sh --non-e2e
```

Single-scenario run command:

```text
GIVN_FEATURES=givn/changes/default-all-addons/specs cargo test --locked --test features_runner -- --name '^<scenario title>$'
```

## Internal Primitives

| Primitive | Type | Purpose |
|---|---|---|
| `enabled_by_default` | `fn` | Serde default for every `Addons` field, returning `true` |
| `impl Default for Addons` | `impl` | In-memory default with every flag true |
| `AddonSpec.default_enabled` | field | Registry default; all entries true |
| `addons::disable_all` | `fn` | Write an explicit false for every known add-on during `givn init --no-addons` |
| `addons::set_addon` | `fn` | Write one add-on key into a project's config |

## Strict Mode

`tests/features_runner.rs` already calls `.fail_on_skipped()`. New step bodies
use `unimplemented!("TODO")` during RED and are replaced before GREEN.

## Local Runnability & Digital Twins

Local run command:

```text
cargo run -- instructions
```

No external dependency, database, container, or digital twin is involved.

## E2E Smoke Test Infrastructure

- **Interface type:** CLI — a real subprocess invocation.
- **E2E runner command:** `./run-tests.sh --e2e`.
- **E2E step location:** `tests/steps/initialize_a_project_e2e_steps.rs` and
  `tests/steps/outline_report_e2e_steps.rs` (existing files).
- **Local test infrastructure:** the existing `features_runner` spawns the
  compiled `givn` binary; no server, browser, or container.
- **E2E framework choice:** cucumber-rs with `GivnWorld`; it invokes the real
  CLI.
- **E2E strict-mode proof:** `.fail_on_skipped()` in `tests/features_runner.rs`.
- **Interaction Coverage Matrix:** the default add-on state is an invariant of
  the existing init action, already covered by "Consumer runs init
  non-interactively and gets opencode skills". The per-add-on opt-out is
  covered by the existing "Consumer enables the outline addon and it is listed
  as enabled" and the new explicit-disable scenario. No new inventory entry is
  added.

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| initialize with an explicit noninteractive target selection | Consumer runs init non-interactively and gets opencode skills | CLI | spawn `givn init --agents opencode` |
| enable the outline addon | Consumer enables the outline addon and it is listed as enabled | CLI | spawn `givn addons enable outline` and `givn addons list` |

## Architecture Impact

| File | Change |
|---|---|
| `src/config/manifest.rs` | Add `enabled_by_default`, apply it to every `Addons` field, implement `Default for Addons` as all true |
| `assets/default-manifest.yaml` | All add-ons true |
| `src/cli/addons.rs` | All registry entries default true; `run_list` uses the effective default; add `disable_all` and refactor `set_addon` to take a project root |
| `src/cli/init.rs` | `--no-addons` writes explicit false values; the summary no longer lists default-off add-ons |
| `assets/config.yaml.tmpl` | Document the all-enabled default and the explicit opt-out |
| `assets/instructions/architecture-principles.md` | Replace "opt-in" with enabled-by-default wording |
| `docs/arc42/05-building-block-view.md`, `08-crosscutting-concepts.md` | Update the default add-on facts |
| `docs/arc42/12-glossary.md` | Define the add-on default posture |

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| `AddonDefaults` | initialize-project | `givn init`, `givn addons list`, `givn addons enable/disable` | `config::manifest::Addons`, `cli::addons` | a second default policy (for example per-agent defaults) needs its own source |

## Data Model Changes

No persistent data or schema. `givn/config.yaml` gains explicit `false` lines
when a project opts out at init time. A missing key means enabled; an explicit
`false` means disabled.

## Coverage process boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| `features_runner` | `./run-tests.sh` | existing coverage build | `coverage/non-e2e-cobertura.xml` | `./merge-coverages.sh` | `Addons::default` and `disable_all` paths |

The existing coverage scripts and paths are unchanged.

## Open Technical Questions

None.
