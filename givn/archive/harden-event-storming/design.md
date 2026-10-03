# Design: harden-event-storming

## Overview

A guidance-only change. It rewrites the Event Storming section of the
generated `givn-ideation` skill template (canvas with distinguishable kinds,
aggregate legality, non-aggregate naming), its topic-state line, and the
phase's quality-review lens; adds one guardrail line to the generated
`/givn-ideation` command; and amends the two durable human-facing records
(`docs/workflow.md`, the arc42 glossary row "Event Storming"). No runtime
behavior, no new command, no parser, no lint. The executable spec asserts the
rendered guidance text through the existing generic content-assertion steps.

## Decisions

- Decides D1: the rule is a coverage rule — every event, command, aggregate,
  and actor appears in one or more mermaid diagrams; the causal reading
  (command between actor and aggregate; aggregate emits events) is stated in
  prose but the arrangement stays the author's choice — realized in the skill
  template's canvas bullets.
- Decides D2: guidance only, no mechanical check — realized by leaving
  `src/cli/lint.rs` untouched (accepted risk R-46 keeps ideation state as
  convention exercised at the phase boundary).
- Decides D3: the kinds are distinguishable — per-node kind markers or a
  legend — realized in the canvas bullet and asserted by the delta.

## Exact file changes

| Path | Change |
|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Event Storming section: the per-event record gains its aggregate; three new bullets carry the canvas, aggregate legality, and non-aggregate-kinds rules; the topic-state line for `eventstorming.md` names aggregates and the rendered canvas; the Quality reviews Event Storming bullet becomes the canvas lens. |
| `assets/commands/givn-ideation.md` | Guardrails: one line requires the rendered canvas and a declared consistency boundary per aggregate. |
| `docs/workflow.md` | The `event storming` row of the phase table names aggregates, the rendered canvas, and legal aggregates. |
| `docs/arc42/12-glossary.md` | Row "Event Storming": the definition includes the rendered canvas and aggregate legality. |

New wording, for the implementer:

Skill template, Event Storming bullets (the two bullets after the intro line
are replaced by these six):

```text
- One event per line, past tense, in the domain's own language.
- For each event: who triggers it (actor), with which command, on which
  aggregate (the consistency boundary the command changes), which policy
  reacts, what the actor reads (read model).
- Draw the canvas: one or more mermaid diagrams in `eventstorming.md` that
  show every event, command, aggregate, and actor, with the kinds
  distinguishable (per-node kind markers or a legend). A command sits between
  the actor that issues it and the aggregate it changes; the aggregate emits
  its events.
- Aggregates are given, not minted. An aggregate is a consistency boundary:
  a domain-noun root that receives commands and emits events under one
  invariant. Reuse the aggregates the project already declares
{{#if addons.arc42}}
  (the arc42 glossary's "Aggregates / boundaries"); a new aggregate is
{{else}}
  (the project's durable documentation); a new aggregate is
{{/if}}
  introduced as a domain term — its root noun, invariant, commands, and
  events — and confirmed before the canvas relies on it.
- A contract, a declaration, a configuration, a cursor position, a grant, a
  report, a request record, or a hosted service is a record, a specification,
  a configuration, or an external system — never an aggregate. Name the kind
  where the canvas shows it.
- A text chain without a canvas is an incomplete event storming: the picture
  and the lines must cover the same elements, and the canvas must render.
```

The aggregate bullet's glossary reference is a Handlebars conditional,
mirroring the term-collision bullet's existing `{{#if addons.arc42}}` block:
the arc42 branch renders the proposal's wording verbatim, and a project
without the addon points at its own durable documentation instead of a file
that does not exist. The asserted phrase "Reuse the aggregates the project
already declares" sits outside the branch and is identical in both renders.

The review lens says "a declared consistency boundary" rather than the
proposal's "declared glossary aggregate": the always-on guidance must not
name addon-owned words — a project with the addons disabled fails the
`match-guidance-to-enabled-addons` scan on "glossary" (case-insensitive) —
and "consistency boundary" is the term the aggregate bullet defines.

Skill template, topic-state line replacement:

```text
- `eventstorming.md` — the event chain with actors, commands, aggregates,
  policies, read models, drawn as a rendered mermaid canvas whose kinds are
  distinguishable.
```

Skill template, Quality reviews Event Storming bullet replacement:

```text
- Event Storming: symmetry of cause and effect, missing event partners, a
  canvas that shows every event, command, aggregate, and actor with its kinds
  distinguishable, every aggregate a declared consistency boundary or a
  confirmed domain term (a record, specification, configuration, or service
  drawn as an aggregate is a defect), the lines and the picture naming the
  same aggregates, a canvas that renders, and a recorded collision check for
  every `confirmed` term entry.
```

Command guardrails, exact wording (appended to the Guardrails list):

```text
- The event storming ships a rendered mermaid canvas naming every event,
  command, aggregate, and actor — and every aggregate is a declared
  consistency boundary, never a record or a specification.
```

`docs/workflow.md`, the `event storming` row of the phase table:

```text
| event storming | domain events in past tense, reverse narrative ("which events must happen for this?"), commands/actors/aggregates/policies, a rendered mermaid canvas showing every event, command, aggregate, and actor with legal aggregates (declared boundaries, never records or specifications), hotspots parked | `eventstorming.md`, `domain-terms.md`, `questions.md` |
```

Durable-document verification: the `docs/workflow.md` row and the
`docs/arc42/12-glossary.md` row are review-verified against the exact texts
here, matching the `change-release-note` precedent — no step reads either
file, and no assertion is added for them.

`docs/arc42/12-glossary.md`, row "Event Storming":

```text
| **Event Storming** | The ideation phase that models the domain as a chain of past-tense domain events with actors, commands, aggregates, policies, and read models, drawn as a rendered mermaid canvas whose aggregates are declared consistency boundaries (never records, specifications, configurations, or services). Anti-term: Storm (rejected as ambiguous). |
```

## Step definitions

No new steps. All four delta scenarios bind to the existing generic steps in
`tests/steps/prose_consolidation_steps.rs`:
`the generated givn-<skill> guidance contains "<text>"` and
`the generated givn-<skill> command contains "<text>"`.

## Runner and strict mode

- Runner: `./run-tests.sh` (`cargo nextest run`); the `tests/features_runner.rs`
  binary executes the `.feature` files.
- Strict mode: `.fail_on_skipped()` in `tests/features_runner.rs`; undefined
  or skipped steps fail.
- Single-scenario command (the unit id is the path relative to the
  `GIVN_FEATURES` root):
  `GIVN_FEATURES=givn/changes/harden-event-storming/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::<scenario title>"`.
- E2E: not extended; the capability's existing `@e2e` scenario still proves the
  `givn skills sync` interaction, and content assertions are variants, not new
  inventory entries.

## Use-case traceability

| Contract element | Technical decision |
|---|---|
| Fragment `support` / capability `generate-ideation-guidance` | Rewrite the generated artifacts the capability owns; no capability split. |
| Guarantee: guidance carries the canvas, kind, and legality rules | Delta scenarios assert the rendered phrases after `givn skills sync`. |
| Rule: the template is the single source | Both agent targets render the same template; the command guardrails repeat the rule in one line. |
| Confirmed Personas | `developer--primary-user` (reads the guidance and reviews the canvas), `agent--technical-role` (runs the phase and draws the canvas), `givn-maintainer--owner` (corpus). The capability has CLI text output, so Personas are named. |

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| generate-ideation-guidance: synchronize skills (fragment; no inventory) | A consumer synchronizes skills and every agent target receives the ideation facilitation skill | CLI | Real `givn skills sync` subprocess; content asserted on the rendered files |

## Version freshness

No language, framework, library, or image version is introduced; the change
edits embedded text assets consumed by the existing Rust binary.

## Local runnability & digital twins

No external or third-party dependency is touched. Local run and test command:
`./run-tests.sh`; E2E smoke via the configured `verify.e2e_command`. No digital
twin is required.

## Coverage process boundaries

N/A — the change adds no instrumented process and no measurement command.

## Arc42 impact

Chapter 12 (glossary, row "Event Storming") is amended as listed above; no ADR
is required (the change applies existing decisions about generated guidance).
The `arc42.md` change artifact records this assessment.

## Internal Primitives

None — the change introduces no data structure, abstraction, or helper in
production code. It edits embedded text assets and durable documentation.
