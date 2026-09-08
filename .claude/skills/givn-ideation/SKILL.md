---
name: givn-ideation
description: Work a vague idea into change-ready use cases through a structured, state-persisting domain exploration
---

# givn-ideation

Work a vague idea into change-ready use cases. This is the structured,
artifact-producing alternative to `givn-explore`: explore answers "do we want
this, and which approach?" without artifacts; ideation answers "what does the
solution look like, domain-wise?" and ends with change-ready use cases, one
per future givn change.

## When to use

| | givn-explore | givn-ideation |
|---|---|---|
| Question | "Do we want this? Which option?" | "How does the solution look, domain-wise?" |
| Structure | none, emergent | phases, notation, reviews |
| Output | none, only insight | topic state, use cases, change seeds |
| Duration | one conversation | several sessions |
| Domain load | low | high (DDD, event storming) |

## Context

- Existing project: run `givn status` and `givn spec tree` first; read the
  README, the durable architecture chapters, and the durable term list
  for known terms before asking anything the project already answered.
- New domain: start from the problem, not the solution.
- Read the topic state before every session; write to it after every turn.
  All ideation work persists there and survives sessions.

## Frame

Inventory what is known before exploring what is not.

Existing project:

1. Run `givn status` and `givn spec tree`; read the README and the
   durable architecture chapters relevant to the domain.
2. Record the research protocol: what exists, what the permanent specs
   already own, where the gaps are.
3. Identify stakeholders — the parties with an interest in the outcome,
   including those who never touch the system (owners, regulators).

New domain:

1. Name the problem in one sentence. If the user named a solution first,
   ask: "if that is the answer, what was the question?"
2. Establish a vision: who the system serves and what changes for them.
3. Establish stakeholders with their interests; every later artifact is
   checked against this list.

Write the frame protocol to the topic state. Report it to the user and let
them confirm it before any phase starts.

## Personas

For every stakeholder type, create one or more personas — concrete persona
segments with interests of their own. An avid reader and a collector are two
personas of the same stakeholder type.

- Create the persona files after Frame; the user confirms every persona
  before it is used. Never invent a persona the user has not confirmed.
- Each persona file carries: role, interests, goals, pain points, success
  criterion, its own vocabulary, and a running record of decisions, needs,
  and open demands — updated after every review the persona takes part in.
- Personas review every artifact that affects their stakeholder type: a
  light reaction to the event storming, value ranking during Use Case
  Definition, critique of their use cases during Use Case Elaboration.
- Cap: at most three persona segments per stakeholder type, about ten in
  total.
- Findings are advisory; the user arbitrates.
- At handoff, persona promotion moves validated personas into the
  project-level persona collection where later workflow phases can consult
  them. Repeated promotion of the same persona segment merges.

The phase ladder: Frame → Event Storming → Use Case Definition → Use Case
Elaboration → Handoff. Run them in order; never skip ahead.

## Topic state

All ideation work lives under `givn/ideation/<topic-id>/`:

- `session.md` — resume state: current phase, progress, the next question.
  Read it first in every session; update it after every turn.
- `frame.md` — the Frame protocol.
- `eventstorming.md` — the event chain with actors, commands, policies, read
  models.
- `use-case-definitions.md` — the sized, bounded use case list.
- `use-cases/<uc-id>.md` — one elaborated use case per file.
- `personas/<type>--<segment>.md` — one persona per file.
- `questions.md` — the open question log: every parked unknown. Never debate
  an unknown in place of a missing answer.
- `domain-terms.md` — domain terms as they crystallize.
- `changes/<uc-id>.md` — one change seed per use case, written at handoff.

Use-case drafts use the same identity and contract vocabulary as permanent
specifications. Each `use-cases/<uc-id>.md` uses this stable schema:

```markdown
# Use case: <use-case-id>
## Level
!
## Actors
## Goal
## Trigger
## Preconditions
## Main flow
## Extensions
## Rules
## Examples
## Minimal guarantee
## Success guarantee
## Personas
## Capabilities
## Interactions
## Includes
## Extends
## Out of scope
## Diagram
```

At ideation time `Capabilities` and `Interactions` may be `- none`; Handoff
maps the same use-case ID into the normal proposal/spec flow. A summary use
case uses `+` and owns child use-case relationships. A `-` subfunction remains
inside a flow and does not become a permanent use-case directory.

## Handoff

The last phase produces the readable record:

- Publish the topic document to `docs/ideation/<topic-id>.md` — written for
  humans, readable start to finish.
- Propose term merges for crystallized terms into the durable term
  list; the user decides each one.
- Offer persona promotion; the user decides per persona file.
- Write one change seed per use case with a suggested start order.
- Each change seed records the stable use-case ID, topic source, confirmed
  Persona IDs, accepted decisions, open questions, and exact handoff status.
  It is input to `/givn-propose`, not a proposal and not a change.
- Before publishing, show the use-case-to-permanent mapping, Persona
  promotion choices, and change seeds. Handoff occurs only after the user
  confirms those records; never auto-promote or auto-start.
- Handoff never starts a change — the user runs the normal proposal flow per
  use case when ready.

## One question at a time

Ask exactly one question at a time and wait for the answer before the next.
If the outcome of a step is unclear, you do not have an example — you have a
question. Record it in the questions log and move on; never debate opinions
in place of a missing answer.

## Event Storming

Model the domain as a chain of past-tense domain events.

- One event per line, past tense, in the domain's own language.
- For each event: who triggers it (actor), with which command, which policy
  reacts, what the actor reads (read model).
- Reverse narrative: "which events must have happened to make this one
  possible?" — walk backwards until the chain starts at the domain's
  beginning.
- Term collisions (same word, different meaning; different words, same
  meaning) go to the domain terms file.
- Unknowns go to the questions log; do not debate opinions.

## Use Case Definition

Cut the event chain into use cases.

- A sequence of events that culminates in value is a use case.
- Size each one: a user goal (`!`) is change-sized; a summary (`+`) groups
  several; a subfunction (`-`) is a step inside a scenario, not a use case.
- Name the boundary each use case lives in; term collisions mark boundaries.
- Tag each use case with the affected stakeholders — their personas will
  review it.

## Use Case Elaboration

Elaborate one defined use case at a time.

- First the rules: acceptance rules as short statements.
- Then the examples: concrete, with real values; unclear outcomes are parked,
  not invented.
- Then the contract: trigger, preconditions, main flow, extensions, minimal
  and success guarantees.
- Only now shape scenario skeletons: preconditions become Given, trigger and
  actor action become When, guarantees become Then.

## Quality reviews

A fresh-context subagent runs a quality review at each phase boundary; one
reviewer per boundary, findings advisory, the user arbitrates:

- Frame: research protocol completeness, stakeholder list with interests.
- Event Storming: symmetry of cause and effect, missing event partners, term
  collisions.
- Use Case Definition: sizing, boundaries, value criterion.
- Use Case Elaboration: rules complete, examples concrete, contract
  unambiguous.

## What you must NOT do

- Never write givn change artifacts — the outcome is topic state and change
  seeds, produced later.
- Never skip the user's confirmation on anything recorded in their name.
- Never answer a question the project already answered in its specs or docs.
