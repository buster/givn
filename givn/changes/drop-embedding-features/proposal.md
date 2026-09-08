# Proposal: drop-embedding-features

## Use-Case Context

- Use-case ID: none (removal of a retired engine; contracts touched live in
  `maintain-project` and `discover-specifications`)
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

givn no longer has a semantic search engine. The capability corpus that
specified it was retired by `retire-semantic-capabilities` (15 capabilities
tombstoned, including the engine itself), and the remaining permanent specs
pin deterministic-only behaviour ("the semantic gate is offline", "stdout
stays free of embedding and download wording").

The build contract, however, still pretends the engine exists:

- The product still declares opt-in build variants for the engine. The
  documented install "with all features including semantic retrieval" does
  not match what an install produces.
- The test suite still partitions every run into engine build variants and
  simulated-model environments, although no permanent capability describes
  that engine any more. The coverage replay executes roughly ten groups
  whose scenarios cannot exist.
- The one remaining engine capability (`release-strict-model-overrides`)
  specifies installed-build strictness for an engine that is retired, and
  its strictness gate currently executes zero checks while still paying a
  full release build.
- The engine's model dependencies are still declared and compiled for the
  feature-selected builds, lengthening every build and CI run for behaviour
  no user can reach.

## Proposed Solution

givn is a single, feature-free binary:

- A standard build and an install produce one binary; there are no build
  variants to select and no model dependencies to declare. The feature-free
  build contract in the maintenance capability now asserts that no opt-in
  features exist at all.
- The engine-backed discovery commands (`spec index`, `spec search`,
  `spec explore`) are gone. The deterministic discovery commands
  (`spec find`, `spec tree`, `spec duplicates`, `spec progress`,
  `spec coverage`) keep their current contracts with unchanged scenarios.
- The installed-build strictness capability is retired through the
  tombstone primitive, recording the engine's removal.
- The suite and the coverage replay run as single feature-free passes: no
  build-variant groups, no simulated-model environments, no strictness
  gate, and no coverage groups whose scenarios cannot exist.
- Generated guidance and skill text never references model-backed
  commands.

## Capability Routing

`givn spec route` recommends `EXTEND givn-workflow-enforcement` (7.00) on a
generic "both invoke `givn check`" match. The change does not extend that
capability; decisions below deviate with rationale.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| release-0-3-0 | — (not ranked) | `EXTEND release-0-3-0` | Owns the pinned build contract; its default-build scenario updates from "four features declared opt-in" to "no opt-in features declared". |
| adr-qualification-gate | — (not ranked) | `NEW in fragments` | Support-fragment capability; its migration-history scenario is modified to the post-engine register counts and archived-record checks. |
| instruction-integrity | — (not ranked) | `NEW in fragments` | Support-fragment capability; its permanent scenario is modified to pin the absence of engine-backed retrieval wording. |
| release-strict-model-overrides (discover-specifications) | — (not ranked) | `RETIRE release-strict-model-overrides` via `@givn.retired` | Engine retired; installed-build strictness has no subject. Precedent: `retire-semantic-capabilities`. |
| givn-workflow-enforcement (verify-change) | `EXTEND` (7.00) | Not extended | Generic keyword match on `givn check`; no observable workflow-enforcement behaviour changes. |

## Out of Scope

- The deterministic discovery, duplicate-audit, progress, and coverage
  tools and their permanent scenarios — their contracts do not change.
- The retire changes already archived (`retire-semantic-capabilities`,
  `retire-semantic-review-guidance`, `retire-semantic-retrieval-vocabulary`)
  — their tombstones stand.
- The `nextest` runner branch — separate workstream, rebased after this
  change lands.
- Reintroducing any model-backed search.

## Open Questions

1. Do any engine-backed entry points other than `spec index`, `spec search`,
   `spec explore` remain referenced by generated guidance, templates, or
   hooks? (Survey says no; verify during design.)
2. Glossary: are the engine terms (E5 candidate, Semantic report, Retrieval
   feature, …) deleted or marked retired in `docs/arc42/12-glossary.md`?
3. Does the strictness gate disappear together with the engine, or does its
   name remain for a future non-engine strictness proof?
4. Sequence: land this change on `main` first, then rebase the `nextest`
   branch onto the collapsed (single-pass) scripts?
