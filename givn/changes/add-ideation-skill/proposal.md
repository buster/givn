# Proposal: add-ideation-skill

## Problem / Opportunity

Givn's guided workflow starts at the proposal: every skill from `/givn-propose`
onward assumes the idea is already understood well enough to commit to one
change. Before that point there is only `/givn-explore` — a thinking stance
with no structure, no artifacts, and no memory. It cannot:

- carry a domain exploration across sessions,
- work a domain out with the user (events, actors, rules, boundaries),
- represent the people the system is supposed to serve,
- produce anything that later changes can build on.

A user facing a new domain or an unexplored area of an existing system has no
path from "vague idea" to "a set of change-ready use cases". Multi-session
exploration state does not exist, stakeholder perspectives are captured
nowhere, and the gap between exploring and proposing is crossed by hand.

## Proposed Solution

Givn ships a new facilitation skill, **givn-ideation**, as an always-on
alternative to `givn-explore`:

- **Explore** answers "do we want this, and which approach?" — no artifacts.
- **Ideation** answers "what does the solution look like, domain-wise?" —
  and ends with change-ready use cases.

The generated explore guidance points to ideation when the user wants
artifacts and multiple changes.

The ideation guidance instructs an agent to run a phase-driven conversation
with the user, **one question at a time**, resuming from persisted state
when a session continues:

1. **Frame** — inventory what is known: for an existing system, the project
   state, permanent specs, and documentation; for a new domain, the problem,
   a vision, and stakeholders with their interests.
2. **Event Storming** — model the domain as a chain of past-tense domain
   events with actors, commands, and policies; unknowns are recorded as open
   questions instead of debated.
3. **Use Case Definition** — define the use cases: sequences of events
   that culminate in value become named use cases, each sized (user-goal use
   cases are change-sized, summaries group them, subfunctions are steps
   inside a scenario) and bounded.
4. **Use Case Elaboration** — elaborate each defined use case: acceptance
   rules, concrete examples, and a use-case contract (trigger,
   preconditions, main flow, extensions, guarantees) shaped so scenarios
   follow directly; rules and examples are collected before any scenario
   wording is written.

All ideation work persists per topic in the project state, so an exploration
survives sessions and is auditable: session state, frame protocol, event
storming protocol, use case definitions, glossary candidates, open questions,
elaborated use cases, and one change seed per use case.

**Stakeholder personas.** For every stakeholder type the guidance creates one
or more persona files — concrete embodiments with distinct interests (an
avid reader and a collector are two personas of the same stakeholder type).
Each persona file carries a running record of decisions, needs, and open
demands, updated after every review the persona takes part in. Personas join
every review of an artifact that affects them: they react to the event
storming, rank use cases by value, and critique the use cases that touch
them. The user confirms every persona and every update; all findings are
advisory.

**Quality reviews.** A fresh-context reviewer applies a rubric at each phase
boundary (frame, event storming, use case definition, use case elaboration);
findings are advisory and the user arbitrates.

**Handoff.** The guidance publishes a readable topic document under the
project docs, proposes glossary merges for crystallized terms, offers to
promote validated personas into a project-level persona collection that
outlives the topic, and writes one change seed per use case with a suggested
start order. Handoff never starts a change by itself — the user runs the
normal proposal flow per use case.

The skill is delivered through the existing skill synchronization: every
configured agent target and slash-command entry point receives it by
default, independent of addons.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. Editing this table IS the
> declaration — `givn check review` compares what you write here against
> the delta you actually author, not against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| ideation-skill | EXTEND spec-explore-change (score 8.00) | `NEW in support` | Route matched on boilerplate `givn spec route` / `givn check review` mentions in the template text, not on behavior. The touched surface is skills-sync guidance content — precedent capabilities `skill-rewrites` and `domain-skill-trigger` live in `support`. `spec-explore-change` owns `givn spec explore` candidate discovery, which this change does not modify. |

## Out of Scope

- No new CLI command — ideation is agent guidance, not a binary surface.
- No changes to the propose/spec/design/design-review/tasks/implement/review
  workflow skills.
- Persona consumption by later workflow phases (review, propose) — future
  changes, after ideation proves the concept.
- Automatic change creation from ideation output — handoff writes change
  seeds only.
- No changes to lint, gates, archive, or instructions routing — ideation
  owns no lifecycle artifact.
- No machine enforcement of the ideation contract beyond what skill
  synchronization already provides; facilitation quality itself is not
  spec-able.

## Open Questions

- None. All decisions were settled during exploration; the exact guidance
  contract phrases asserted by the specs crystallize in the spec phase.