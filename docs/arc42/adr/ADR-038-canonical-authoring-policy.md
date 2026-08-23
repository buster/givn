---
status: accepted
date: 2026-08-14
decision-makers:
  - givn maintainers
consulted:
  - prose-consolidation and instruction-integrity scenarios
informed:
  - agents and contributors authoring interactions
---

# Keep one normative interaction-authoring policy

## Qualification

- [x] **Hard to reverse** - contradictory generated guidance systematically produces incompatible specifications and test evidence.
- [x] **Shapes quality attributes** - one policy improves consistency, testability, and maintainability.
- [x] **Structural** - the resolved `specs` instruction is the policy boundary used by other artifact guidance.
- [x] **Must be shared** - every authoring entry point must delegate to the same rule set.

## Context and Problem Statement

Interaction inventories and E2E rules were repeated across instructions, skills, commands, and templates. Small wording changes caused agents to choose different interfaces or duplicate coverage.

## Decision Drivers

- one discoverable authority
- project override compatibility
- thin derived guidance
- language and runner neutrality

## Considered Options

- Duplicate the policy in every asset.
- Put the complete policy in the resolved `specs` instruction and reference it elsewhere.
- Add a runtime include/preprocessor.

## Decision Outcome

Chosen option: **Make the resolved `specs` instruction normative**. Other instructions, skills, commands, and templates contain only artifact-specific choreography and derived checks; project overrides replace the effective policy through the normal cascade.

### Consequences

- Agents must read the resolved specs instruction before authoring or reviewing interactions.
- Generated guidance can become stale until `givn skills sync` runs.
- Policy parity is checked rather than assumed.

### Confirmation

Prose-consolidation and instruction-integrity scenarios assert canonical policy content, override resolution, generated target parity, and absence of contradictory legacy wording.

## Pros and Cons of the Options

### Canonical specs instruction

- Good, because policy has one authority without a runtime include mechanism.
- Bad, because derived entry points are not self-contained.

### Duplicated policy

- Good, because each asset can stand alone.
- Bad, because wording drifts and contradictions are hard to detect.

### Runtime include

- Good, because shared text could be mechanically inserted.
- Bad, because it adds a new renderer and override protocol for a documentation problem.

## More Information

See [chapter 8.3a](../08-crosscutting-concepts.md#83a-normative-authoring-guidance) and [ADR-048](ADR-048-instruction-json-contract.md).
