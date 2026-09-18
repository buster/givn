# Proposal: reshape-author-change

## Use-Case Context

- Use-case ID: `author-change`
- Ideation topic: `multi-scm-support`
- Seed path: `givn/ideation/multi-scm-support/changes/author-change.md`
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`

## Problem / Opportunity

The 0.8.0 use-case model falsifies the current `author-change` document:

- **R9 oversized leaf** — its Main flow names three actor goals: initialize a
  project and select agent targets; create a change; read the canonical
  artifact guidance.
- **R1 goal conjunction** — its Success guarantee names two outcomes: the
  change scaffold and generated guidance.
- **R5 extension discipline** — all three extensions lack the main-flow step
  reference and the protected guarantee.
- **R7 mechanism vocabulary** — "backend" and "attribution plumbing" appear
  in the Main flow and the Success guarantee.
- **R8 component- and channel-shaped names** — `new-command` and
  `init-agent-target-selection` name commands; `install-extras` names the
  addon bundle with the pre-glossary word.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 1) records these
deviations and names this follow-up.

## Proposed Solution

Promote `author-change` to a `+` summary and split it into two `!` leaves:

| Use case | Goal | Capabilities |
|---|---|---|
| `author-change` (`+`, summary) | Set up a Givn project and create changes with the guidance to carry them | none; Includes the two children |
| `initialize-project` (`!`) | Set up a Givn project and its agent integrations | `initialize-a-project`, `enable-optional-addons` |
| `create-change` (`!`) | Create a change ready to carry from intent to executable specification | `create-a-change`, `prose-consolidation`, `visual-design-contract` |

Renames (final names, per the migration's decision that the follow-up
decides them):

| Current | New | Home |
|---|---|---|
| `new-command` | `create-a-change` | create-change |
| `init-agent-target-selection` | `initialize-a-project` | initialize-project |
| `install-extras` | `enable-optional-addons` | initialize-project |

Each feature file moves with its capability; the narratives gain step-
referenced extensions and lose the mechanism vocabulary from Goal, Trigger,
Main flow, and Guarantees.

## Out of Scope

- The support-fragment renames (`rename-support-fragments`).
- Any behavior change; the scenarios move and keep their assertions.

## Open Questions

None.
