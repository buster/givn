# Use case: author-change

## Level

!

## Actors

- Developer
- Authoring agent

## Goal

Create a valid change and the guidance needed to carry it from intent to executable specification.

## Trigger

The developer starts a new change or initializes a project.

## Preconditions

- The project is in a location where Givn can manage its workflow.
- The active SCM backend is selected when one exists; a project without a backend is still initializable.

## Main flow

1. Initialize the project and select agent targets: report the selected backend, install attribution plumbing only where the backend supports it, and name what was skipped and why.
2. Create a change with a valid identity.
3. Read the canonical artifact guidance: generated guidance names only commands that work on the active backend and states backend limitations explicitly.

## Extensions

- Optional addons are enabled or disabled by explicit user decision.
- No SCM backend exists: initialization scaffolds the corpus, generates guidance, and exits 0 with a named warning that archive and managed upgrade are unavailable.
- The active backend lacks a capability (for example transparent attribution hooks): the installation report and the generated guidance state the limitation instead of naming a command that would fail.

## Rules

- Change identities are stable kebab-case values.
- Generated guidance is synchronized from canonical source assets.
- Generated guidance may name only commands that work on the active backend.
- The installation report names what was installed, what was skipped, and why.
- Commit path: the SCM-neutral path is `givn commit`; Git keeps the chained `commit-msg` hook as a fallback for raw commits; Fossil attribution exists only through `givn commit`, and the guidance says so.
- Initialization without an SCM does not refuse; it warns.

## Examples

- Git: `givn init` writes `.githooks/commit-msg`, chains an existing hook, generates skills, and reports `commit attribution: .githooks/commit-msg`. The agent then commits with `givn commit -m "feat: parser"` or raw `git commit`; both carry attribution.
- Fossil: `givn init` scaffolds `givn/`, writes no hook, and reports `commit attribution: givn commit path only`. `givn commit -m "feat: parser"` stamps the check-in comment; raw `fossil commit` does not, and the generated skill states that.
- No SCM: `givn init` exits 0 with a warning that archive and managed upgrade are unavailable; skills are generated; a later `givn archive` refuses by name.
- A developer creates a multi-word change without manual scaffolding.

## Minimal guarantee

The project remains unchanged when an invalid request is rejected, and no generated guidance names a command that cannot work in the active project.

## Success guarantee

The project has the requested change scaffold and generated guidance that matches the active backend.

## Personas

- developer--primary-user
- agent--technical-role
- givn-maintainer--owner

## Capabilities

- prose-consolidation
- new-command
- init-agent-target-selection
- install-extras
- visual-design-contract

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| prose-consolidation | request canonical E2E guidance | Canonical E2E policy is emitted by specs instructions |
| prose-consolidation | inspect generated E2E guidance | Generated guidance points to the canonical E2E policy |
| visual-design-contract | request the design guidance for a visual interface | Author requests the visual design contract guidance |
| visual-design-contract | read the synchronized design and step guidance | Synchronized guidance defines the contract approval and capture mechanics |
| init-agent-target-selection | select agent targets | Consumer selects agent targets during interactive init |
| init-agent-target-selection | initialize non-interactively | Consumer runs init non-interactively and gets opencode skills |
| init-agent-target-selection | resynchronize selected targets | Consumer re-syncs skills after init and the selection is honoured |
| init-agent-target-selection | initialize a Fossil project | Consumer initializes a Fossil checkout and receives backend-aware guidance |
| init-agent-target-selection | initialize without SCM | Consumer initializes a plain directory and receives a named warning |

## Includes

- fragment: support

## Extends

- none

## Out of scope

Implementing the product behavior described by a change; archive publication (`archive-change`); managed upgrade (`maintain-project`).

## Diagram

```mermaid
flowchart LR
  Actor((Developer)) --> Init[[Initialize project]]
  Actor --> New[[Create change]]
  New --> Guidance[[Read guidance]]
```

<!-- givn:base-sha256:33827ac48f46b44a104e8ccc0d34f1b012bfb43cd4b53ed35108d7d6b3ead50d -->
