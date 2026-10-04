<!-- givn:base-sha256:a5ea95b03614da79b0c3dceac7a0ff65877ea043f6488c6b2f0eb9541037efd6 -->
# Use case: create-change

## Level

!

## Actors

- Developer
- Supporting: Authoring agent

## Goal

Create a change ready to carry from intent to executable specification.

## Trigger

The developer starts a new change.

## Preconditions

- The project is initialized as a Givn project.

## Main flow

1. The developer creates a change; the system writes the scaffold with a
   stable identity.
2. The developer reads the artifact guidance; the system keeps one canonical
   policy per topic and points derived guidance at it.

## Extensions

- 1: the requested identity is invalid → the system rejects it and leaves the project unchanged; protects the minimal guarantee.

## Rules

- Change identities are stable kebab-case values.
- Generated guidance is synchronized from canonical source assets.
- One canonical policy exists per topic; derived guidance points at it.
- Every user-facing question follows the effective dialog policy, including questions outside a named skill and questions prepared in a fresh delegated context.
- Generated default guidance carries the policy obligation without copying the full policy; project-owned overrides remain authoritative.
- The policy serves project-familiar readers with decision-critical facts, concrete consequences, material revision costs, and proportional clarification.
- A change's proposal carries one release note; every revision the change records shares it as the message subject, with the scenario title in the message body.

## Examples

- A developer creates a multi-word change without manual scaffolding.
- An author entering directly through a generated skill loads the effective dialog policy before its first question, while the full worked examples remain in the canonical instructions.

## Minimal guarantee

The project remains unchanged when an invalid identity is rejected.

## Success guarantee

The project has the requested change, ready to carry from intent to
executable specification.

## Personas

- developer--primary-user
- agent--technical-role
- givn-maintainer--owner

## Capabilities

- change-release-note
- create-a-change
- prose-consolidation
- visual-design-contract

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| prose-consolidation | request canonical E2E guidance | Canonical E2E policy is emitted by specs instructions |
| prose-consolidation | inspect generated E2E guidance | Generated guidance points to the canonical E2E policy |
| visual-design-contract | request the design guidance for a visual interface | Author requests the visual design contract guidance |
| visual-design-contract | read the synchronized design and step guidance | Synchronized guidance defines the contract approval and capture mechanics |
| change-release-note | create a change | A new change's proposal carries the release-note field |
| change-release-note | read the proposal guidance | The proposal guidance requires a release note |
| change-release-note | read the generated implementation guidance | The generated implementation guidance shares the release note |

## Includes

- fragment: support

## Extends

- none

## Out of scope

Implementing the change's product behavior; archiving the change
(`archive-change`); managed upgrade (`maintain-project`).

## Diagram

```mermaid
flowchart LR
  Actor((Developer)) --> New[[Create change]]
  New --> Guidance[[Read canonical guidance]]
```
