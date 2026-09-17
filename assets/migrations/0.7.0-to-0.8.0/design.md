# Migration Design: {{SOURCE_VERSION}} to {{TARGET_VERSION}}

Self-contained procedure for the project LLM. You have no access to the Givn
source repository; everything required is in this document. The managed
refresh and the `givn_config_version` bump are already done by `givn upgrade`
(Commit {{UPGRADE_COMMIT}}). This maintenance change carries the
project-specific work.

Work the phases in order. Within each phase, one artifact per edit, and run
the affected checks after each edit. Stop and ask the human when a decision is
not derivable from the project's own artifacts.

## Preflight

1. Confirm `givn/config.yaml` declares `givn_config_version:
   "{{TARGET_VERSION}}"` and this change exists at
   `givn/changes/migrate-0-7-0-to-0-8-0/`.
2. Confirm the checkout is clean and committed before the first edit.
3. Inventory the corpus: `givn spec tree`, the permanent `givn/specs/`
   directories, `givn/personas/`, every `givn/ideation/*/personas/`, and
   `docs/arc42/adr/`.
4. Record the inventory (file, planned disposition) in this change's
   `migration.yaml` or a notes section of `design.md`, so the migration is
   auditable.

## Enforcement boundary

Only two of the rules below are machine-checked:

- `givn lint` reports a dangling `## Personas` reference as an advisory
  `[PERS ]` finding.
- `givn check review` resolves a declared visual interface's `PERSONAS:`
  value against `givn/personas/`.

The use-case scoping rules, the persona identity criteria, and the ADR
falsification tests are authoring and review guidance. A green suite does not
by itself prove this migration; the checks are the recorded dispositions and
the human-confirmed diffs.

## Phase 1 — Use-case model

### The definition

> A use case is a contract between one primary actor and the system: the
> actor's single goal, stated as observable outcome, with the success and
> minimal guarantees the system makes while pursuing it. A use case stays
> true if the system is rebuilt with different technology.

### The eight scoping rules

1. **One goal, one primary actor.** The goal is one active-verb phrase naming
   the primary actor's outcome. An "and" joining two verb phrases in Goal, or
   two distinct outcomes in the success guarantee, means split. Closure test:
   the actor could stop afterward and the system is left consistent.
2. **Goal-shaped, never component-shaped.** No use-case or capability name
   may name a technical component, mechanism, or channel (server, backend,
   store, service, panel, browser, CLI). Placement test for every capability:
   which actor goal does completing this advance? Channel is never an answer;
   a browser or CLI variant of an action lives under the action's goal.
3. **Level discipline.** `!` owns capabilities directly. `+` owns zero direct
   capabilities and only child `!` via `Includes: usecase:`; its coverage is
   the union of its children. A leaf whose flow names three or more distinct
   actor goals, or whose success guarantee is a conjunction, is promoted to
   `+` and split. `-` stays ideation-only.
4. **Mutation containment.** The guarantees declare the case as observing
   (read-only) or changing. An observing case owns zero mutating
   interactions; a capability whose scenarios assert a state change belongs
   under a goal whose success guarantee names that change.
5. **Extension discipline.** Every extension names the main-flow step it
   branches from and the guarantee it protects or the recovery it performs.
   An extension introducing a new actor goal, a new success outcome, or a new
   capability is a new use case or a capability under another goal — never an
   extension.
6. **Precondition and flow consistency.** Preconditions hold before the
   trigger and are never established by the main flow. The success guarantee
   is established by the flow, never assumed. The trigger is an actor action
   or event, never a state or desire.
7. **Implementation independence.** Goal, Trigger, Main flow, and Guarantees
   must remain true if the system were rebuilt in another language and
   framework. Mechanism vocabulary (branches, compare-and-swap, loopback,
   tokens, manifests, signals) never appears in those sections. Non-human
   actors use "X for Y" notation; a bare system actor's behavior is a
   subfunction.
8. **Relationship semantics.** `Includes` means unconditional reuse (summary
   to child, leaf to fragment substrate). `Extends` means optional insertion
   under a stated condition recorded in the extending document; never targets
   fragments.

### The ten review rejection criteria

| # | Criterion | Proving section |
|---|---|---|
| R1 | Goal conjunction | `## Goal` joins two verb phrases, or `## Success guarantee` names two outcomes |
| R2 | Missing primary actor | `## Actors` has no identifiable primary, or `## Trigger` does not name them |
| R3 | Placement failure | `## Capabilities` contains a capability that fails the placement test |
| R4 | Mutation under an observing goal | A capability's scenarios assert a state change while the guarantees declare observation |
| R5 | Extension without a step reference | `## Extensions` omits its main-flow step, or adds a new success outcome |
| R6 | Inconsistent preconditions or flow | A precondition is established by the flow; the success guarantee is assumed; the trigger is a state or desire |
| R7 | Mechanism vocabulary | `## Goal`, `## Trigger`, or the guarantees name a mechanism |
| R8 | Component- or channel-shaped name | A use-case or capability name names a technical component or channel |
| R9 | Oversized leaf | A leaf's `## Main flow` names three or more actor goals |
| R10 | Relationship misuse | `Extends` without a stated condition; an end-user goal placed as a fragment or vice versa |

The same criteria ship in `givn instructions review`; the generated ideation
guidance carries the definition and rules. Use them as the live references.

### Procedure

1. For each permanent use-case document, list the failing criteria. Example:
   a leaf whose `## Goal` is "Manage the run and publish results" fails R1
   (two verb phrases) and R9 if its flow names three goals.
2. Repair one capability at a time:
   - **Split a conjunction goal** into one goal per outcome, each with its own
     primary actor and its own leaf document.
   - **Promote an oversized leaf** to `+` and move each capability under the
     child goal it advances; the summary owns no direct capability.
   - **Re-home a misplaced capability** under the goal it advances; a browser
     or CLI variant moves under the action's goal, not under a channel name.
   - **Move a mutation** under a changing goal whose success guarantee names
     the change.
   - **Rewrite extensions** to `- <step>: <condition> → <handling>; protects
     <guarantee>`, or move the new outcome into its own use case.
3. After each move, run the affected `.feature` files and `givn lint`; a
   moved capability keeps its scenarios unchanged.
4. Record each split, promotion, and re-homing in this change's `design.md`
   with the before/after use-case IDs.

## Phase 2 — Personas

### Identity criteria

A new persona exists only when it differs from an existing one in at least
one of — the criteria are disjunctive, not conjunctive:

- **authority boundary** — what it may do or decide;
- **evaluation capability** — what it can judge;
- **own vocabulary** — terms it uses or refuses.

Topic-specific demands go into a dated topic-stances appendix, never into the
stable core. One role sliced by which mechanism it touches (for example an
adapter implementer by port) is one persona with stance blocks when the
authority and evaluation match.

### File schema

```markdown
# Persona: <name>

## Stable core

- Role: <one line>
- Authority boundary: <what it may do or decide>
- Evaluation capability: <what it can judge>
- Own vocabulary: <terms it uses or refuses>
- Success criterion: <what success means for it>

## Running record

Dated entries per review or topic stance; append-only, never rewriting
earlier entries.

- <YYYY-MM-DD> — <entry>

## Topic stances

Topic-specific demands, dated, never in the stable core.

- <YYYY-MM-DD> — <topic>: <stance>
```

A reused persona's topic-local file contains a header line plus new record
entries only — never a biography copy:

```text
Reuse: <persona-id>, base <short-hash or date>; this file adds record entries only
New: <persona-id> — <distinguishing criterion>
```

### Procedure

1. Inventory `givn/personas/` and every sibling topic's `personas/` against
   the project's stakeholder types.
2. Collapse segments that share all three criteria into one project persona;
   keep their differences as dated topic stances.
3. Migrate each project persona file to the schema above; move topic-flavored
   interests, goals, and pain points into the stances appendix.
4. Convert each reused topic copy to the `Reuse:` header with record entries
   only; never leave a second biography copy.
5. At handoff, append-merge record entries into the project persona with a
   user-confirmed diff; the topic file stays as historical record.
6. Resolve every `## Personas` entry against `givn/personas/<id>.md` or a
   topic's `personas/<id>.md`; `givn lint` reports the rest as `[PERS ]`
   advisories. A visual change's `PERSONAS:` value must resolve in
   `givn check review`; a reasoned `none` is valid only for a change without
   a user-visible surface.

Preserved behaviors: the user confirms every persona before use; personas
review every artifact that affects their stakeholder type; at most three
segments per stakeholder type and about ten in total; findings are advisory
with user arbitration; promotion never happens automatically.

## Phase 3 — ADRs

### Falsification tests

A candidate fails the qualification gate when any of these holds, no matter
how the five dimensions were argued:

1. **Single-change reversibility.** Fully implementable and reversible within
   one change, touching one component. File count or interface breadth does
   not change this. Non-example: a project identity key split is a data-model
   decision owned by the design and the glossary.
2. **No multi-change constraint.** Does not constrain at least two future
   changes or two components. Non-example: a terminal state value plus its
   presentation filtering is feature behavior owned by the capability spec
   and design.
3. **Product-behavior exclusion.** Observable workflow behavior ownable by a
   use case plus Gherkin (who authorizes what, in which order, with which
   state values) fails `lower_level_artifact` unless it also moves a
   component, authority, or deployment boundary stated in the verdict.
   Non-examples: an assumption-and-go-ahead gate and a human evaluation gate
   are workflow behavior; a change-request entity with crash-safety
   requirements is a domain entity.

The tests classify by kind, not count; there is no numeric cap.

### Procedure

1. For each active record in `docs/arc42/adr/`, re-run the five mandatory
   dimensions plus the three falsification tests and the cheaper-home
   challenge: name the specific canonical artifact (feature specification,
   design, Arc42 chapter, process documentation, project documentation, or
   code) that would own the rationale instead.
2. A qualified record stays. A record that fails is routed once:
   - move the rationale to the chosen canonical home;
   - remove the record from the active register (`docs/arc42/09-...`) and
     `docs/arc42/adr/README.md`;
   - record the historical-scope note — record ID, failing test, destination
     — in this migration change's `design.md`. Do **not** copy the record
     into `docs/arc42/adr/archive/`: a record that fails the gate is not a
     historical ADR.
3. Replace a stale project-local ADR template: delete
   `docs/arc42/adr/adr-template.md`, run `givn addons enable arc42` to
   re-scaffold missing files, then confirm the file no longer contains
   `ADR-QUALIFICATION: PASS` or `qualification: QUALIFIED`.
4. Record the structured verdict (qualification, the five gates,
   supporting-only shared evidence, routing, and the route target) in the
   change assessment (`arc42.md`) before creating or editing any MADR.
5. New decisions follow the same gate: the design's Technology Decisions
   justification stays in `design.md` unless the falsification tests pass.

## Phase 4 — Smaller adoptions

1. **Dialog policy.** Run `givn instructions dialog` and follow it in every
   interactive step: named artifacts with identifiers in parentheses, the
   four-part option treatment (meaning, gain, cost, concrete example),
   cross-reference glosses, standalone answerability, rejected alternatives,
   one question at a time.
2. **Change seeds.** Give every seed a status line after its title:
   `ready`, `proposed by <change-id>`, `implemented in <change-id>`,
   `superseded by <reference>`, or `withdrawn`. Classify existing seeds
   honestly — a consumed seed stamped `ready` misleads the next planner;
   `givn lint` reports a missing or contradicted status as a `[SEED ]`
   advisory. Add the content schema (use-case ID, topic, Personas, suggested
   start, accepted decisions with option treatment, full open questions).
3. **Term records.** Migrate `domain-terms.md` entries to
   term/status/definition/contrast/first-use/collision-check with
   `candidate`, `confirmed`, and `rejected` statuses. A `confirmed` term
   keeps its status only with the pasted search commands and hit counts in
   the collision-check cell ("no collision" without commands is not a
   record); rejected terms keep their definition and reason and are never
   deleted.
4. **Guarantee qualifiers and decision ledger.** A rule or guarantee that
   depends on an unresolved question carries `(open: Q<N>)`; closing the
   question removes the qualifier in the same edit. Proposal questions carry
   `D<n>` IDs with the option treatment and a disposition (answered with its
   source, carried with an owner and the deciding artifact, or out of scope
   with a reason); the design records `Decides D<n>: <decision> — <where it
   is realized>` for every ID.
5. **Severity and routing output.** Re-read `givn lint`: every finding now
   carries `advisory`, `finding`, or `blocking`, and advisory findings do not
   change the exit code. `givn spec route` reports explicit confidence
   (`signal` or `no signal`); with no signal the proposal omits the
   recommendation and records the decision with its rationale.

## Phase 5 — Ejected overrides

1. Inventory overrides: uncommented settings in `givn/config.yaml`, files
   under `givn/artifacts/`, and project skill overrides under
   `givn/skills/`.
2. For each, compare against the {{TARGET_VERSION}} embedded default
   (`givn eject` can materialize a fresh copy for comparison) and re-apply
   only genuine project-specific edits on top.
3. Never hand-patch the old rule text back into a stale copy. Managed
   generated files (skills, commands, AGENTS.md blocks) were already
   refreshed by `givn upgrade`; only ejected facets are project-owned.

## Validation and completion

1. `givn lint` and `givn check review` pass; the `[PERS ]` advisories are
   resolved or explicitly recorded as forward references.
2. The project's own suite passes (`verify.command`).
3. The next archive runs cleanly under the {{TARGET_VERSION}} rules.
4. The recorded inventory has a disposition for every use case, persona, and
   ADR, and the migration change archives through the normal flow.
