# The use-case model

This is the durable reference for how givn's use cases are cut, written, and
reviewed. The generated ideation guidance carries the authoring rules; the
review instruction carries the rejection criteria; this document holds both
plus the worked example.

## Definition

> A use case is a contract between one primary actor and the system: the
> actor's single goal, stated as observable outcome, with the success and
> minimal guarantees the system makes while pursuing it. A use case stays
> true if the system is rebuilt with different technology.

## Scoping rules

Each rule is falsifiable. The falsification test is what a reviewer checks.

### 1. One goal, one primary actor

The goal is one active-verb phrase naming the primary actor's outcome.
Falsification: an "and" joining two verb phrases in Goal, or two distinct
outcomes in the success guarantee, means split. Closure test: the actor could
stop afterward and the system is left consistent.

### 2. Goal-shaped, never component-shaped

No use-case or capability name may name a technical component, mechanism, or
channel (server, backend, store, service, panel, browser, CLI). Placement test
for every capability: which actor goal does completing this advance? Channel is
never an answer; a browser or CLI variant of an action lives under the action's
goal.

### 3. Level discipline

`!` owns capabilities directly. `+` owns zero direct capabilities and only
child `!` via `Includes: usecase:`; its coverage is the union of its children.
Promotion rule: a leaf whose flow names three or more distinct actor goals, or
whose success guarantee is a conjunction, must be promoted to `+` and split.
`-` stays ideation-only.

### 4. Mutation containment

The guarantees declare the case as observing (read-only) or changing. An
observing case owns zero mutating interactions; a capability whose scenarios
assert a state change belongs under a goal whose success guarantee names that
change.

### 5. Extension discipline

Every extension names the main-flow step it branches from and the guarantee it
protects or the recovery it performs. An extension introducing a new actor
goal, a new success outcome, or a new capability is a new use case or a
capability under another goal — never an extension.

### 6. Precondition and flow consistency

Preconditions hold before the trigger and are never established by the main
flow. The success guarantee is established by the flow, never assumed. The
trigger is an actor action or event, never a state or desire.

### 7. Implementation independence

Goal, Trigger, Main flow, and Guarantees must remain true if the system were
rebuilt in another language and framework. Mechanism vocabulary (branches,
compare-and-swap, loopback, tokens, manifests, signals) never appears in those
sections. Non-human actors use "X for Y" notation (`Agent for the Requester`);
a bare system actor's behavior is a subfunction.

### 8. Relationship semantics

`Includes` means unconditional reuse (summary to child, leaf to fragment
substrate). `Extends` means optional insertion under a stated condition
recorded in the extending document; never targets fragments. The graph stays
acyclic (already enforced).

## Authoring requirements

- Actors: the first bullet is the primary actor; others are marked
  `Supporting:`. Goal and Trigger name the primary actor.
- Main flow: numbered steps, each pairing actor intention with system
  responsibility (`N. Actor <intent>; System <responsibility>`). Numbers
  exist so extensions can reference them.
- Extensions: `- <step>: <condition> → <handling>; protects <guarantee>`.
  Unreferenced extensions are rejected.
- Guarantees: Minimal is true even on failure; Success is the goal's
  observable outcome and nothing else.
- Out of scope: names excluded actor goals (with owning use-case IDs when they
  exist), not excluded mechanisms.

## Review rejection criteria

A review fails when any of these holds. Each criterion names the observable
document section that proves it.

| # | Criterion | Proving section |
|---|---|---|
| R1 | Goal conjunction | `## Goal` joins two verb phrases, or `## Success guarantee` names two outcomes |
| R2 | Missing primary actor | `## Actors` has no identifiable primary, or `## Trigger` does not name them |
| R3 | Placement failure | `## Capabilities` contains a capability that fails the placement test for this goal |
| R4 | Mutation under an observing goal | A capability's scenarios assert a state change while the guarantees declare observation |
| R5 | Extension without a step reference | `## Extensions` omits its main-flow step, or adds a new success outcome |
| R6 | Inconsistent preconditions or flow | A precondition is established by the flow; the success guarantee is assumed; the trigger is a state or desire |
| R7 | Mechanism vocabulary | `## Goal`, `## Trigger`, or the guarantees name a mechanism |
| R8 | Component- or channel-shaped name | A use-case or capability name names a technical component or channel |
| R9 | Oversized leaf | A leaf's `## Main flow` names three or more actor goals |
| R10 | Relationship misuse | `Extends` without a stated condition; an end-user goal placed as a fragment or vice versa |

## Worked example

The example comes from a real project's corpus. It shows both failure shapes
and their repairs.

### An observing mega-leaf

The `follow-the-run` use case promised observation, yet its rules said
"Observation does not answer Questions or create Change Requests" while it
owned `rename a project`, `submit a driving action`, token auth, SIGTERM
handling, and the PWA manifest. That shape fails R4 (mutation under an observing goal) and R8 (component- or channel-shaped name).

The repair splits it into three leaves and one substrate fragment:

- `inspect-project-state` — the observing goal.
- `follow-run-events` — the observing goal for the event stream.
- `manage-project-identity` — the changing goal that owns the rename
  mutation.
- `web-interface-substrate` — a fragment for the reusable web-server
  mechanics, included by the goals that need it; it is not a user goal.

### A conjunction-goal leaf

The `run-a-project` use case was 211 lines with 18 capabilities, 23
unnumbered extensions, and a goal that joined four independently valid goals.
That shape fails R1 (goal conjunction) and R9 (oversized leaf).

The repair promotes it to a summary (`+`) and splits it into the goals
ideation had already named: the leaf's capabilities are re-homed one at a time
under the goal each one advances, and the summary's coverage becomes the union
of its children. Incremental repair is the expected path; a big-bang rewrite
is not required.

### Why the rules exist

The same project had already defined six properly-sized goals during ideation.
All six shipped as capabilities absorbed into the mega-leaf, because nothing
at authoring or review time named the absorption as a defect. The rules and
criteria above are that missing signal.
