# Proposal: migrate-usecases-and-ideation

## Problem / Opportunity

Projects created with Givn 0.5.0 may still represent domain narratives as
groups and `group.md`, while ideation topics and Personas are not connected to
permanent use-case identity. A filename rename would lose domain meaning,
misclassify reusable behavior, and make capability moves impossible to audit.

## Proposed Solution

Migrate the active project corpus to stable use-case IDs:

```text
givn/specs/<usecase-id>/usecase.md
givn/specs/<usecase-id>/<capability>.feature
givn/specs/fragments/fragment.md
givn/specs/fragments/<capability>.feature
```

The migration first inventories permanent specifications, active changes,
ideation topics, Personas, interactions, and coverage. It presents a confirmed
ledger of semantic operations before moving files. It preserves behavior hashes,
E2E evidence, interaction mappings, active-change state, ideation state, and
historical archives. Handoff and Persona promotion remain explicit user
decisions.

## Out of Scope

- Rewriting observable behavior during a structural migration.
- Rewriting `givn/archive/` history.
- Inventing, auto-promoting, or silently editing Personas.
- Choosing the release target version before release cutting.

## Open Questions

- This bundle is published for Givn 0.6.0.
