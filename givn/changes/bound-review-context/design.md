# Design: bound-review-context

## Overview

Add a review-context record to the change workflow. `proposal.md`,
`design-review.md`, and `review.md` record `Review kind`, `Author`, and
`Reviewer context`. A new `review_context` gate validates the record during
`givn check review` and again during archive. Guidance assets stop prescribing
subagent orchestration; `assets/instructions/review.md` becomes the single
owner of the one-context review policy.

The gate is deliberately narrow. It reads the recorded kind; it never infers
independence from the presence or absence of markers. A legacy change with no
recorded kind keeps its previous gate result.

## Visual Design Contract

- Interface: none
- Viewports: not applicable
- Design system: not applicable
- Persona: agent--technical-role

The change adds line-oriented CLI diagnostics and workflow guidance. It adds
no rendered screen and no terminal UI state.

## Use-Case Traceability

Owning use case: `verify-change` (`givn/specs/verify-change/usecase.md`).

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| A review PASS requires the existing review markers | Both markers stay required; the new gate runs alongside them | `givn-workflow-enforcement`, existing review-check E2E scenario |
| The developer receives a complete verification result | The gate reports every review-context defect in one run, not the first | `givn-workflow-enforcement` review-kind scenarios |
| An invalid change cannot pass the verification gate | A proposal recording `independent` cannot be satisfied by a `self-review` record | `givn-workflow-enforcement` mismatch scenarios |
| The outline gate keeps its reviewer identity check | `OUTLINE: AUTHOR` / `OUTLINE: REVIEWER` are unchanged; the new record is separate | `outline-review-gate` scenarios |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock`; no new dependency |
| cucumber-rs | existing workspace pin | `Cargo.lock`; `.fail_on_skipped()` remains the strict-mode mechanism |

## Decisions

- Decides D1: the subagent-vocabulary acceptance check covers guidance assets
  (`assets/instructions`, `assets/commands`, `assets/skills`,
  `assets/templates`), not the development report or harness reporter
  integrations — realized in the asset-scan scenario and the
  `review_context` guidance check.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `givn-workflow-enforcement` | `tests/steps/workflow_enforcement_steps.rs` | same file |

Test runner command (from `givn/commands.yaml`):

```text
./run-tests.sh --non-e2e
```

Single-scenario run command:

```text
GIVN_FEATURES=givn/changes/bound-review-context/specs cargo test --locked --test features_runner -- --name '^<scenario title>$'
```

## Internal Primitives

| Primitive | Type | Purpose |
|---|---|---|
| `ReviewKind` | `enum` | `SelfReview` or `Independent`, parsed from the change record |
| `ReviewRecord` | `struct` | Optional kind plus `Author` and `Reviewer context` values from one artifact |
| `review_context::parse` | `fn` | Read and validate one artifact's review record, ignoring fenced examples |
| `review_context::check` | `fn` | Compare the proposal, design-review, and review records and return one deterministic failure |
| `guidance_assets` | `const` | The four guidance asset trees the vocabulary scan targets |

## Strict Mode

`tests/features_runner.rs` already calls `.fail_on_skipped()` on the Cucumber
builder. Undefined or pending steps fail the runner. New step bodies start as
`unimplemented!("TODO")` during RED and are replaced before GREEN; no step body
is left empty.

## Local Runnability & Digital Twins

Local run command:

```text
cargo run -- instructions
```

No external dependency, database, container, or digital twin is involved. The
CLI runs against the local filesystem.

## E2E Smoke Test Infrastructure

- **Interface type:** CLI — a real subprocess invocation is the public
  consumer boundary.
- **E2E runner command:** `./run-tests.sh --e2e` (`verify.e2e_command`), which
  filters `@e2e`.
- **E2E step location:** `tests/steps/workflow_enforcement_e2e_steps.rs`
  (existing file for the capability).
- **Local test infrastructure:** the existing `features_runner` spawns the
  compiled `givn` binary; no server, browser, or container.
- **E2E framework choice:** cucumber-rs with the project's `GivnWorld`; it
  invokes the real CLI.
- **E2E strict-mode proof:** `.fail_on_skipped()` in
  `tests/features_runner.rs`; the existing task-list setup proof is inherited.
- **Interaction Coverage Matrix:** the new behavior runs through the existing
  `check review` action already covered by the capability's E2E scenario
  "Consumer runs review and receives every use-case schema problem in one
  pass". No new inventory entry or E2E scenario is added; the new gate
  scenarios are regular scenarios asserting distinct invariants.

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run the review check on a change with use-case schema errors | Consumer runs review and receives every use-case schema problem in one pass | CLI | spawn the `givn` binary with `check review --change <id>` and read the process output |

## Architecture Impact

New module:

```text
src/review_context.rs
  ReviewKind, ReviewRecord, parse, check
```

Modified modules:

| File | Change |
|---|---|
| `src/lib.rs` | Export `review_context` |
| `src/integrity_hook.rs` | Call `review_context::check` during the review integrity check |
| `src/cli/archive.rs` | Call `review_context::check` in the static review gates before mutation |
| `assets/templates/proposal.md` | Add the `Review kind` record with the self-review default |
| `assets/templates/design-review.md` | Add kind, author, and reviewer-context fields |
| `assets/templates/review.md` | Add kind, author, and reviewer-context fields |
| `assets/instructions/review.md` | Own the one-context policy |
| `assets/instructions/proposal.md` | Remove architect-subagent orchestration and fresh-context design-review sequencing |
| `assets/instructions/design-review.md` | Remove mandatory fresh-context grilling and hardening subagents |
| `assets/instructions/characterize.md` | Use the current-context review policy |
| `assets/instructions/ideation.md` | Replace the fresh-context quality review with the current-context policy |
| `assets/instructions/reverse-engineer.md` | Remove fresh-delegated-context instructions |
| `assets/instructions/tasks.md` | Remove coding-subagent delegation and self-report language |
| `assets/commands/*.md`, `assets/skills/*/SKILL.md.tmpl` | Remove fresh-context agent boilerplate; commands load the review instruction |
| `assets/instructions/dialog.md` | Keep business delegation; remove execution-topology prescriptions |

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| `ReviewContextCheck` | verify-change | `givn check review`, `givn archive` | `src/review_context.rs` | a second review artifact format or a non-review gate needs the same parser |

The module is internal; it imports no CLI or SCM type and takes the change
directory as input. The composition root is `cli::check` and `cli::archive`.

## Data Model Changes

No persistent data or schema. The change adds two record formats:

```text
Review kind: self-review | independent
Author: <id>
Reviewer context: <id>
```

The proposal records the requested kind. `design-review.md` and `review.md`
record the performed kind. A proposal-recording `independent` requires an
independent review record with a different reviewer context. The outline
gate's existing `OUTLINE: AUTHOR` / `OUTLINE: REVIEWER` identity check is
unchanged.

## Coverage process boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| `features_runner` | `./run-tests.sh` | existing coverage build | `coverage/non-e2e-cobertura.xml` | `./merge-coverages.sh` | `review_context::check` rejection path |

The existing coverage scripts and paths are unchanged.

## Open Technical Questions

None.
