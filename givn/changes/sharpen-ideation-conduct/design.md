# Design: sharpen-ideation-conduct

## Overview

A guidance-only change: it rewrites parts of the generated `givn-ideation`
skill template and its slash-command guardrails, and amends the two durable
records that describe ideation conduct (arc42 8.19 change seeds and the
glossary row "Change seed"). No runtime behavior, no new command, no parser.
The executable spec asserts the rendered guidance text through the existing
generic content-assertion steps.

## Decisions

- Decides D1: replace the per-use-case seed rule with the
  deliverable-guarantee cut and the no-lying-intermediate-state test —
  realized in the skill template's Handoff section and the command
  guardrails.
- Decides D2: the Frame is confirmed once; later phase boundaries report what
  was recorded and proceed, asking only when a new fork is unresolved —
  realized in the skill template's Frame section and the command guardrails.

## Exact file changes

| Path | Change |
|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Rewrite the "One question at a time" section (live-question standard, no re-asking, one question per direction); amend the Frame paragraph (confirm once, later boundaries report); replace the Handoff seed bullet with the deliverable cut; rename the topic-state seed entry to `changes/<change-id>.md`; generalize the seed schema heading and `Use-case ID` field to a change identity with plural use-case IDs. |
| `assets/commands/givn-ideation.md` | Guardrails: the question line gains the full-treatment requirement and the no-re-asking rule; the final batch line becomes the seed-cut guardrail; one line records the Frame-once rule. |
| `docs/arc42/08-crosscutting-concepts.md` | Section 8.19: the seed is cut by deliverable guarantee, covers one or more use cases, and its intermediate state never makes a confirmed rule untrue; file naming uses the change identity; schema field becomes plural use-case IDs. |
| `docs/arc42/12-glossary.md` | Row "Change seed": per-deliverable proposal seed, not per-use-case; may cover several use cases and never creates a state where a confirmed rule is untrue. |

New wording, for the implementer:

Skill template, question section:

```text
## One question at a time

Follow the canonical dialog policy (`givn instructions dialog`) for every
live question. A bare option list is a defect, not a style choice: name the
decision it blocks, the context that forces it, and every option's meaning,
concrete example, gain, and cost.

Ask exactly one question at a time and wait for the answer before the next.
Ask only what the current step cannot answer from confirmed decisions; a
direction already decided is never asked again. A review finding that follows
mechanically from a confirmed decision is applied and reported; only a
finding that changes a confirmed direction returns to the human. A phase report states what was recorded and invites correction; it is a question only when a new fork is unresolved. Decisions that share one direction are one question; independent decisions stay separate.

If the outcome of a step is unclear, you do not have an example — you have a
question. Record it in the questions log and move on; never debate opinions
in place of a missing answer.
```

Skill template, Frame paragraph replacement:

```text
Write the frame protocol to the topic state. Confirm the Frame with the
user: it defines the goal, scope, and stakeholders everything else derives
from. Later phase boundaries report what was recorded and proceed; they ask
only when a new fork is unresolved.
```

Skill template, Handoff bullet replacement:

```text
- Cut change seeds by deliverable guarantee, using the seed schema below. A
  seed is one change whose guarantee is true end to end and can ship alone;
  it may cover several use cases or fragments, and one use case may yield several seeds. Never cut a seed whose intermediate state would make a confirmed rule untrue — a setting no command honors, a decision no artifact applies. Record each seed's dependencies and a suggested start order. Each seed is input to `/givn-propose`, not a proposal or a change.
```

Command guardrails, exact wording:

```text
- Every question carries the full option treatment; a direction already decided is never asked again.
- Cut seeds by deliverable guarantee; a confirmed rule must stay true.
- The Frame is confirmed once; later phase boundaries report and proceed.
```

## Step definitions

No new steps. The rendered-guidance assertions bind to the existing generic
steps in `tests/steps/prose_consolidation_steps.rs`:
`the generated givn-<skill> guidance contains "<text>"` and
`the generated givn-<skill> command contains "<text>"`.

## Runner and strict mode

- Runner: `./run-tests.sh` (`cargo nextest run`, the `tests/features_runner.rs`
  binary executes the `.feature` files).
- Strict mode: `.fail_on_skipped()` in `tests/features_runner.rs`; undefined
  or skipped steps fail.
- Single-scenario command for RED/GREEN evidence:
  `cargo test --test features_runner -- --name "<scenario title>"`.
- E2E: not extended; the capability's existing `@e2e` scenario still proves
  the `givn skills sync` interaction, and content assertions are variants,
  not new inventory entries.

## Use-case traceability

| Contract element | Technical decision |
|---|---|
| Fragment `support` / capability `generate-ideation-guidance` | Rewrite the two generated artifacts the capability owns; no capability split. |
| Guarantee: guidance carries the conduct rules | Delta scenarios assert the rendered phrases after `givn skills sync`. |
| Rule: the template is the single source | Both agent targets render the same template; the command guardrails repeat the conduct rules. |
| Confirmed Personas | `developer--primary-user` (answers questions), `agent--technical-role` (asks and cuts), `givn-maintainer--owner` (corpus). CLI text output is user-visible, so Personas are named. |

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| generate-ideation-guidance: synchronize skills (fragment; no inventory) | A consumer synchronizes skills and every agent target receives the ideation facilitation skill | CLI | Real `givn skills sync` subprocess; content asserted on the rendered files |

## Version freshness

No language, framework, library, or image version is introduced; the change
edits embedded text assets consumed by the existing Rust binary.

## Local runnability & digital twins

No external or third-party dependency is touched. Local run and test command:
`./run-tests.sh`; E2E smoke via the configured `verify.e2e_command`. No
digital twin is required.

## Coverage process boundaries

N/A — the change adds no instrumented process and no measurement command.

## Arc42 impact

Chapter 8.19 and the glossary row "Change seed" are amended as listed above;
no ADR is required (the change applies existing decisions about generated
guidance and the seed lifecycle). The `arc42.md` change artifact records
this assessment.
