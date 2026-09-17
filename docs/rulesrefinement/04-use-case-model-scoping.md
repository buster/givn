# 04 — use-case-model-scoping

Status: ready for `/givn-propose`
Source: use-case scoping (T3 option A)
Depends on: 01 (dialog policy)

## Problem

givn's entire formal use-case definition is one sentence in a historical
release plan. The cutting guidance in the ideation skill is four lines. There
is no oversize signal, and the only size signal that exists — the
`singleton_usecase` warning — punishes small leaves, so the tool
statistically rewards absorbing capabilities into mega-leaves. Level
discipline is dead: the parser permits `+` with direct capabilities, and no
`+` document exists in givn or kiekste. Routing groups by CLI command surface
and lexical overlap, which offers `NEW in <giant-leaf>` as the path of least
resistance. The review gate has no rejection criteria for scope.

The result in kiekste: `run-a-project` is 211 lines with 18 capabilities, 23
unnumbered extensions, and a goal that is a conjunction of four goals;
`follow-the-run` promises observation while owning a rename mutation and the
entire web server.

## Evidence

kiekste side:

- `givn/specs/run-a-project/usecase.md`: goal text joins four independently
  valid goals; four actors with no primary; extensions add new success
  outcomes instead of protecting the main one; `repository-backend` is
  infrastructure mechanism filed under a user goal; the precondition is
  contradicted by step 1.
- `givn/specs/follow-the-run/usecase.md`: rule "Observation does not answer
  Questions or create Change Requests" while owning `rename a project`,
  `submit a driving action`, token auth, SIGTERM handling, and the PWA
  manifest.
- `givn/ideation/run-authority/use-case-definitions.md` defined six
  properly-sized goals; all shipped as capabilities absorbed into
  `run-a-project`.
- Provenance: the migration commit moved `group.md` 1:1; the release plan
  forbade a mechanical rename but supplied no cut rules.

givn side:

- `docs/usecase-migration-release-plan.md` line 85: the only definition.
- `assets/skills/givn-ideation/SKILL.md.tmpl` lines 169–178: four lines of
  cutting guidance.
- `src/overlap/analysis.rs` `singleton_usecases` (line 508) warns on
  1-capability leaves; no oversize counterpart.
- `src/spec/route.rs`: affinity from command surface and content-word
  overlap; `src/spec/routing_table.rs` checks only that a decision was
  recorded.
- `assets/instructions/review.md` lines 202–205: one prose paragraph with no
  rejection criteria.

## Decided scope

T3 option A only: canonical definition, scoping rules, authoring
requirements, and review rejection criteria in templates, skills, and
instructions. No parser changes, no router changes, no archive-time
capability moves.

## Required changes

### 1. Canonical definition

Encode this definition in the ideation skill and the review instruction, and
publish it as a durable document (see 5):

> A use case is a contract between one primary actor and the system: the
> actor's single goal, stated as observable outcome, with the success and
> minimal guarantees the system makes while pursuing it. A use case stays
> true if the system is rebuilt with different technology.

### 2. Scoping rules

Each rule is falsifiable and belongs in the ideation skill (Use Case
Definition and Use Case Elaboration) and the review rejection criteria (4).

1. **One goal, one primary actor.** The goal is one active-verb phrase
   naming the primary actor's outcome. Falsification: an "and" joining two
   verb phrases in Goal, or two distinct outcomes in the success guarantee,
   means split. Closure test: the actor could stop afterward and the system
   is left consistent.
2. **Goal-shaped, never component-shaped.** No use-case or capability name
   may name a technical component, mechanism, or channel (server, backend,
   store, service, panel, browser, CLI). Placement test for every
   capability: which actor goal does completing this advance? Channel is
   never an answer; a browser or CLI variant of an action lives under the
   action's goal.
3. **Level discipline.** `!` owns capabilities directly. `+` owns zero
   direct capabilities and only child `!` via `Includes: usecase:`; its
   coverage is the union of its children. Promotion rule: a leaf whose flow
   names three or more distinct actor goals, or whose success guarantee is a
   conjunction, must be promoted to `+` and split. `-` stays ideation-only.
4. **Mutation containment.** The guarantees declare the case as observing
   (read-only) or changing. An observing case owns zero mutating
   interactions; a capability whose scenarios assert a state change belongs
   under a goal whose success guarantee names that change.
5. **Extension discipline.** Every extension names the main-flow step it
   branches from and the guarantee it protects or the recovery it performs.
   An extension introducing a new actor goal, a new success outcome, or a
   new capability is a new use case or a capability under another goal —
   never an extension.
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
8. **Relationship semantics.** `Includes` means unconditional reuse
   (summary to child, leaf to fragment substrate). `Extends` means optional
   insertion under a stated condition recorded in the extending document;
   never targets fragments. The graph stays acyclic (already enforced).

### 3. Authoring requirements

- Actors: the first bullet is the primary actor; others are marked
  `Supporting:`. Goal and Trigger name the primary actor.
- Main flow: numbered steps, each pairing actor intention with system
  responsibility (`N. Actor <intent>; System <responsibility>`). Numbers
  exist so extensions can reference them.
- Extensions: `- <step>: <condition> → <handling>; protects <guarantee>`.
  Unreferenced extensions are rejected.
- Guarantees: Minimal is true even on failure; Success is the goal's
  observable outcome and nothing else.
- Out of scope: names excluded actor goals (with owning use-case IDs when
  they exist), not excluded mechanisms.

### 4. Review rejection criteria

Add to `assets/instructions/review.md` (Use-case and Persona conformance
section) and to the design-review use-case branch as a checklist. A review
fails when any holds:

- R1: Goal conjunction or multi-outcome success guarantee.
- R2: No identifiable primary actor, or the trigger does not name them.
- R3: A capability fails the placement test for this goal.
- R4: A mutating scenario under an observing goal.
- R5: An extension without a step reference, or an extension adding a
  success outcome.
- R6: A precondition established by the flow; a success assumed rather than
  established; a trigger that is a state or desire.
- R7: Mechanism vocabulary in Goal, Trigger, or Guarantees.
- R8: A component- or channel-shaped capability or use-case name.
- R9: A leaf with three or more actor goals in its flow.
- R10: `Extends` without a stated condition; an end-user goal placed as a
  fragment or vice versa.

### 5. Durable document

Create `docs/use-case-model.md` with the definition, the eight rules, the
authoring requirements, the rejection criteria, and the worked before/after
example from kiekste (the `follow-the-run` split into
`inspect-project-state`, `follow-run-events`, `manage-project-identity`, and
a `web-interface-substrate` fragment; `run-a-project` promoted to `+` and
split into the goals ideation already named). Reference it from
`docs/workflow.md` §3 and `docs/reference.md`.

### 6. Generated template guidance

`src/gherkin/usecase_md.rs::usecase_md_template()` embeds guidance into every
generated `usecase.md`: numbered main-flow example, the extension format, the
primary-actor convention, and a one-line promotion note. Verify the parser
tolerates `Primary:`/`Supporting:` prefixes in Actors and step-referenced
extensions; adjust parsing tolerance only if existing tests fail. Do not add
new warnings.

### 7. Handoff mapping

Ideation Handoff maps each elaborated `!` to a permanent leaf document, never
to a capability inside another goal. Add this to the Handoff checklist in the
ideation skill.

## Files to change

| Path | Change |
|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Definition, rules 1–8, authoring requirements, handoff mapping |
| `assets/instructions/review.md` | Rejection criteria R1–R10 |
| `assets/instructions/design-review.md` | Reference the criteria in the use-case branch |
| `assets/instructions/specs.md` | Placement test for `NEW in <leaf>`; capability naming |
| `assets/skills/givn-spec/SKILL.md.tmpl` | Placement test reference |
| `src/gherkin/usecase_md.rs` | Template guidance only |
| `docs/use-case-model.md` | New durable document |
| `docs/workflow.md` | Reference from §3 |
| `docs/reference.md` | Reference from the skills section |

## Executable-spec impact

- `givn/specs/make-usecases-canonical/canonical-usecase-contract.feature`:
  extend with scenarios for the definition, the promotion rule, the
  placement test, and the rejection criteria presence.
- `givn/specs/fragments/ideation-skill.feature`: assert the generated
  guidance contains "primary actor", the promotion rule, and the extension
  format.
- `givn/specs/fragments/instruction-integrity.feature`: assert the review
  instruction carries the rejection criteria.
- `givn/specs/fragments/skill-rewrites.feature`: update phrase assertions
  touched by the rewrites.
- If the parser tolerance changes, add a scenario to the use-case document
  parsing spec.

## Acceptance criteria

- [ ] The definition and the eight rules appear in the generated ideation
      guidance; the rejection criteria appear in the review instruction.
- [ ] `docs/use-case-model.md` exists and is referenced from the workflow
      and reference docs.
- [ ] Generated `usecase.md` files contain the authoring guidance.
- [ ] A `follow-the-run`-shaped case fails R4 and R8 on review.
- [ ] A `run-a-project`-shaped case fails R1 and R9 and the review demands
      promotion and split.
- [ ] No new parser warning is introduced; `givn lint` behavior for existing
      corpora is unchanged.

## Out of scope

- T3 option B: parser oversize warnings, unreferenced-extension warnings,
  `+`-with-capabilities errors, demoting the `singleton_usecase` warning.
- T3 option C: router anti-absorption, `SPLIT`/`PROMOTE` decision verbs,
  archive-time capability moves.
- Re-cutting kiekste's corpus. That is a kiekste follow-up, enabled by these
  rules, not part of the givn change.

## Risks and notes

- Prose-only enforcement: absorption can continue where reviewers are lax.
  The structural fix (router and parser signals) is deliberately deferred;
  record this as accepted risk in the design.
- The rejection criteria must not become a rubber stamp. Tie each to an
  observable artifact section so a reviewer can check mechanically.
- `run-a-project`-style splits are large; the rules enable incremental repair
  (re-homing capabilities one at a time), so do not demand a big-bang rewrite
  in the guidance.
