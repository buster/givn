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

Write the frame protocol to the topic state. Confirm the Frame with the
user: it defines the goal, scope, and stakeholders everything else derives
from. Later phase boundaries report what was recorded and proceed; they ask
only when a new fork is unresolved.

## Personas

For every stakeholder type, create one or more personas — concrete persona
segments with interests of their own. An avid reader and a collector are two
personas of the same stakeholder type.

Reuse comes first: inventory `givn/personas/` and every sibling topic's
`personas/` into a session-state table against the topic's stakeholder types,
then match by ID or record a one-sentence new-segment justification naming
the distinguishing criterion. A new persona exists only when it differs from
an existing one in at least one of — the criteria are disjunctive, not
conjunctive:

- **authority boundary** — what it may do or decide;
- **evaluation capability** — what it can judge;
- **own vocabulary** — terms it uses or refuses.

The three adapter-implementer-shaped files collapse to one persona with three stance blocks: they share the authority (ship an adapter against a port) and the evaluation (the conformance suite), so they are one persona segment, not three. Topic-specific demands go into a dated `## Topic stances` appendix,
never into Role, Interests, or Goals.

Persona files use the schema in `assets/templates/persona.md`: a stable core
(role, authority boundary, evaluation capability, own vocabulary, success
criterion) and an append-only running record of dated entries per review or
topic stance. A reused persona's topic-local file contains a header line plus
new record entries only — never a biography copy:

```text
Reuse: <persona-id>, base <short-hash or date>; this file adds record entries only
New: <persona-id> — <distinguishing criterion>
```

- Create the persona files after Frame; the user confirms every persona
  before it is used. Never invent a persona the user has not confirmed.
- Personas review every artifact that affects their stakeholder type: a
  light reaction to the event storming, value ranking during Use Case
  Definition, critique of their use cases during Use Case Elaboration.
- Cap: at most three persona segments per stakeholder type, about ten in
  total.
- Findings are advisory; the user arbitrates.
- At handoff, persona promotion appends the validated persona's record
  entries into the project-level persona collection with a user-confirmed
  merged diff; the topic file stays as historical record. Repeated promotion
  of the same persona segment merges; never auto-promote.

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
- `changes/<change-id>.md` — one change seed per deliverable, written at handoff.

Every `questions.md` entry uses this stable schema:

```markdown
## Q<N> — <short title in domain language>

- Status: open | closed <date> (<decider>)
- Blocks: <the decision or work item this question blocks, one line>
- Context: <why the question exists — the problem, the constraints, and
  what changes depending on the answer, 2–5 sentences>
- Question: <full standalone question>
- Options:
  1. **<option name>** — <what it means>
     In practice: <concrete example of what the user sees or the system does>
     Gain: <what improves>
     Cost: <what gets harder or is given up>
- Resolution: **<chosen option>** because <reason>. Rejected: <option>
  because <reason>; <option> because <reason>.
```

A closed question keeps its full text, options, gains, costs, examples, and
rejected alternatives. An outcome-only resolution is not a valid record.

Every `domain-terms.md` entry uses this stable schema:

```markdown
| Term | Status | Definition | Contrast | First use | Collision check |
|---|---|---|---|---|---|
| <Term> | candidate | <one sentence> | <X, not Y> | <file:line> | <commands run, result> |
```

Status is `candidate`, `confirmed`, or `rejected`:

- `candidate` — the definition and first use are sufficient.
- `confirmed` — requires a genus-and-differentia definition, a contrast pair,
  and one concrete domain sentence plus a recorded collision check.
  The full term introduction test lives in `givn-domain-modeling`.
- `rejected` — keeps the definition and the rejection reason; never deleted.

Escape `|` as `\|` inside the Collision check cell and prefer hit-count
commands without pipelines, so the recorded command stays inside its column.

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
- Propose only `confirmed` terms for the durable term list; report
  `rejected` terms with their reason. The user decides each one.
- Offer persona promotion; the user decides per persona file.
- Cut change seeds by deliverable guarantee, using the seed schema below. A
  seed is one change whose guarantee is true end to end and can ship alone;
  it may cover several use cases or fragments, and one use case may yield several seeds. Never cut a seed whose intermediate state would make a confirmed rule untrue — a setting no command honors, a decision no artifact applies. Record each seed's dependencies and a suggested start order. Each seed is input to `/givn-propose`, not a proposal or a change.
- Map each elaborated `!` use case to its own permanent leaf document — never to a capability inside another goal.
- Before publishing, show the use-case-to-permanent mapping, Persona
  promotion choices, and change seeds. Handoff occurs only after the user
  confirms those records; never auto-promote or auto-start.
- Handoff never starts a change — the user runs the normal proposal flow per
  seed when ready.

Each `changes/<change-id>.md` seed uses this stable schema:

```markdown
# Change seed: <change-id>

Status: ready

- Use-case IDs: <stable use-case-id>, ...
- Topic: <topic-id>
- Personas: <persona-id>, or none — <reason>
- Suggested start: <order position and dependencies>
- Handoff: <exact handoff status>

## Accepted decisions

### <decision title>

- Context: <what forced the decision>
- Options:
  1. **<option name>** — <what it means>
     In practice: <concrete example of what the user sees or the system does>
     Gain: <what improves>
     Cost: <what gets harder or is given up>
- Chosen: **<option name>** because <reason>. Rejected: <option> because <reason>.

## Open questions

### <short title>

<full standalone question: what it blocks, the context, the options with
meaning, gain, cost, and a concrete example each>
```

Every seed's first line after the title is its status: `Status: ready`,
`Status: proposed by <change-id>`, `Status: implemented in <change-id>`,
`Status: superseded by <reference>`, or `Status: withdrawn`. Handoff writes
`ready`; proposal planning stamps `proposed by`; archiving stamps
`implemented in` or `withdrawn`. A superseding decision edits only the status
line; the seed's original text stays.

## One question at a time

Follow the canonical dialog policy (`givn instructions dialog`) for every
live question. A bare option list is a defect, not a style choice: name
the decision it blocks, the context that forces it, and every option's
meaning, concrete example, gain, and cost. Ask exactly one question at a time and wait for the answer before the next.
Ask only what the current step cannot answer from confirmed decisions; a
direction already decided is never asked again. A review finding that follows
mechanically from a confirmed decision is applied and reported; only a
finding that changes a confirmed direction returns to the human. A phase report states what was recorded and invites correction; it is a question only when a new fork is unresolved. Decisions that share one direction are one question; independent decisions stay separate.

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
  meaning) go to the domain terms file. Before a term reaches `confirmed`,
  run and record the search-before-coin procedure:
  1. Grep the durable glossary (`docs/arc42/12-glossary.md`) for the term and
     its synonyms.
  2. Grep every sibling topic's `domain-terms.md` and the repository for the
     term.
  3. Record the commands run and the hit counts in the `Collision check`
     column. "No collision" without recorded commands is not a record.
- Unknowns go to the questions log; do not debate opinions.

## Use Case Definition

Cut the event chain into use cases.

> A use case is a contract between one primary actor and the system: the
> actor's single goal, stated as observable outcome, with the success and
> minimal guarantees the system makes while pursuing it. A use case stays
> true if the system is rebuilt with different technology.

A sequence of events that culminates in value is a use case. Size each one
with these falsifiable rules; the durable reference with the worked example is
`docs/use-case-model.md`:

1. **One goal, one primary actor.** The goal is one active-verb phrase naming
   the primary actor's outcome. Falsification: an "and" joining two verb
   phrases in Goal, or two distinct outcomes in the success guarantee, means
   split. Closure test: the actor could stop afterward and the system is left
   consistent.
2. **Goal-shaped, never component-shaped.** No use-case or capability name may
   name a technical component, mechanism, or channel (server, backend, store,
   service, panel, browser, CLI). Placement test for every capability: which
   actor goal does completing this advance? Channel is never an answer; a
   browser or CLI variant of an action lives under the action's goal.
3. **Level discipline.** `!` owns capabilities directly. `+` owns zero direct
   capabilities and only child `!` via `Includes: usecase:`; its coverage is
   the union of its children. The promotion rule: a leaf whose flow names
   three or more distinct actor goals, or whose success guarantee is a
   conjunction, must be promoted to `+` and split. `-` stays ideation-only.
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
   actors use "X for Y" notation (`Agent for the Requester`); a bare system
   actor's behavior is a subfunction.
8. **Relationship semantics.** `Includes` means unconditional reuse (summary
   to child, leaf to fragment substrate). `Extends` means optional insertion
   under a stated condition recorded in the extending document; never targets
   fragments. The graph stays acyclic.

Name the boundary each use case lives in; term collisions mark boundaries.
Tag each use case with the affected stakeholders — their personas will review
it.

## Use Case Elaboration

Elaborate one defined use case at a time.

- First the rules: acceptance rules as short statements.
- Then the examples: concrete, with real values; unclear outcomes are parked,
  not invented.
- Then the contract: trigger, preconditions, main flow, extensions, minimal
  and success guarantees.
- Only now shape scenario skeletons: preconditions become Given, trigger and
  actor action become When, guarantees become Then.

Authoring requirements for the contract:

- Actors: the first actor is the primary actor; mark additional actors
  `Supporting:`. Goal and Trigger name the primary actor.
- Main flow: numbered steps, each pairing actor intention with system
  responsibility (`N. Actor <intent>; System <responsibility>`); numbers let
  extensions reference them.
- Extensions: `- <step>: <condition> → <handling>; protects <guarantee>`. An
  extension that adds a new outcome is never an extension.
- Guarantees: Minimal is true even on failure; Success is the goal's
  observable outcome and nothing else.
- Out of scope: names excluded actor goals (with owning use-case IDs when they
  exist), not excluded mechanisms.
- Open prerequisites: a rule or guarantee that depends on an unresolved
  question carries `(open: Q<N>)` after the statement, naming the question in
  `questions.md`. A qualified statement must not be presented as unconditional.
  The qualifier stays until the question closes; closing the question removes the qualifier in the same edit.

## Quality reviews

A fresh-context subagent runs a quality review at each phase boundary; one
reviewer per boundary, findings advisory, the user arbitrates:

- Frame: research protocol completeness, stakeholder list with interests.
- Event Storming: symmetry of cause and effect, missing event partners, and a
  recorded collision check for every `confirmed` term entry.
- Use Case Definition: sizing, boundaries, value criterion.
- Use Case Elaboration: rules complete, examples concrete, contract
  unambiguous.

## What you must NOT do

- Never write givn change artifacts — the outcome is topic state and change
  seeds, produced later.
- Never skip the user's confirmation on anything recorded in their name.
- Never answer a question the project already answered in its specs or docs.
