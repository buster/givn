# Design: arc42-review-alignment

## Overview

Extend the completion-review guidance and review template with one Arc42
conformance section. The section is always present in the shared core assets so
projects with local overrides have a stable contract; it is explicitly marked
`N/A when the arc42 addon is disabled`. When the addon is enabled, the section
is a mandatory human review of the change-level Arc42 assessment, durable
chapter content, `design.md`, `tasks.md`, and the completed source tree.

This change does not infer architecture from source code. It makes the
cross-reference and its evidence a required review activity, which is the
appropriate seam for facts that require architectural judgment.

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | latest stable | Existing project toolchain; no new runtime dependency |
| cucumber-rs (`cucumber`) | 0.23.0 | Existing Gherkin runner; verified in `Cargo.lock` |
| Markdown assets | N/A | Existing cascade/eject asset mechanism |

No new dependency is required. The implementation is Markdown asset content
plus capability-specific Cucumber steps that read real CLI outputs and the
generated review guidance.

## Step Definitions

| Capability | Step definition file |
|---|---|
| arc42-review-alignment | `tests/steps/arc42_review_alignment_steps.rs` |

The generic CLI invocation and stdout assertions remain in
`tests/steps/mod.rs`; the capability file contains only the capability-specific
assertion for the ejected review template.

The effective test-runner configuration is read from `givn/config.yaml` and,
for freshly initialised projects, the user override is stored in
`givn/commands.yaml`:

```sh
cargo test --test features_runner
```

The change-scoped command used during implementation is:

```sh
GIVN_FEATURES=givn/changes/arc42-review-alignment/specs cargo test --test features_runner
```

Single-scenario command:

```sh
GIVN_FEATURES=givn/changes/arc42-review-alignment/specs cargo test --test features_runner -- --name "<scenario title>"
```

## Internal Primitives

None. The change adds no production structs, functions, or data structures.
The review table is deliberately human-readable rather than an inferred or
persisted architecture model.

## Strict Mode

The existing `tests/features_runner.rs` builder calls
`fail_on_skipped()`. New steps begin as `unimplemented!("TODO")` during RED;
undefined or pending steps therefore produce a non-zero runner result rather
than a false pass.

The e2e command configured in this project is:

```sh
cargo test --test features_runner -- --tags @e2e
```

It uses the same strict Cucumber runner with the `@e2e` tag filter.

## Local Runnability & Digital Twins

This is a CLI-only documentation capability. No application server, database,
queue, browser, or external service is started. The local run command is the
change-scoped Cucumber command shown above; it invokes the compiled `givn`
binary in a temporary project root.

There are no external dependencies and therefore no digital twins or isolated
network services. The anticipated interface obstacle is an ejected template
being inspected from the wrong project root; the step uses the same temporary
root as the real `givn eject` subprocess and reads the exact path reported by
the command.

## E2E Smoke Test Infrastructure

- **Interface type:** CLI.
- **Driver:** `tests/common::run_givn` starts the real `givn` subprocess.
- **E2E command:** `cargo test --test features_runner -- --tags @e2e`.
- **Infrastructure:** a temporary initialized project per scenario; no server,
  database, browser, or digital twin.
- **Assertions:** stdout, exit status, and the contents of the file written by
  the real `givn eject review:template` command. Repository inspection is
  additional evidence, not a replacement for the CLI result.

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and `givn` subprocess | `cargo test --test features_runner` | Existing project coverage configuration | Existing `coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml` paths | Existing archive coverage gate | Successful `givn instructions` and `givn eject` CLI invocations |

No new production process is introduced. Coverage uses the repository's
existing commands and collision-safe output paths.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| inspect the completion-review instructions for a change | Arc42-enabled review instructions require conformance evidence | CLI | `run_givn`: invoke `givn instructions review --change ...`, assert stdout and exit status |
| eject the completion-review template for customisation | The review template includes the Arc42 conformance block | CLI | `run_givn`: invoke `givn eject review:template`, then inspect the ejected file named by the CLI contract |
| synchronize generated review guidance for agent targets | Generated review guidance stays aligned after skill sync | CLI | `run_givn`: invoke `givn skills sync`, then inspect the generated review skill and slash-command files |

The disabled-addon scenario is a configuration variant of the first
interaction, not a second e2e action.

## Architecture Impact

The following embedded assets are updated in parallel so every delivery surface
can carry the same rule. Running `givn skills sync` materialises the changed
skill and command assets into all configured agent targets:

- `assets/templates/review.md` — adds the fill-in conformance table and sign-off
  items.
- `assets/instructions/review.md` — adds the review procedure and mismatch
  remediation rule.
- `assets/skills/givn-review/SKILL.md.tmpl` — gives agents the same mandatory
  procedure.
- `assets/commands/givn-review.md` — keeps the slash-command orchestration
  aligned with the instruction and template.

The durable Arc42 documentation updated for this process rule is:

- `docs/arc42/04-solution-strategy.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/09-architecture-decisions.md`
- `docs/arc42/10-quality-requirements.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`

The six chapter edits are factual documentation of this change, not a change
to the Arc42 chapter structure or scaffold format.

No artifact graph, archive gate, durable chapter, or Arc42 chapter-selection
rule changes.

## Data Model Changes

None. No YAML schema, persisted state, or command-line interface changes.

## Open Technical Questions

None. Human architectural comparison is intentional; automatic source-code
inference would create false confidence and is out of scope.
