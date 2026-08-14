# Proposal: prose-consolidation

## Problem / Opportunity

The rules for writing and reviewing Gherkin are repeated in several generated
skills, slash commands, and artifact instructions. The wording differs between
those places, especially for E2E evidence, user-interaction inventories, and
the distinction between a real interface and a repository-only check. One
skill also contains contradictory guidance about whether flag combinations are
separate inventory entries. Authors therefore receive different answers
depending on which entry point they use, and a later rule update can drift
without anyone noticing.

## Proposed Solution

- The user receives one canonical explanation of the E2E policy: one E2E
  evidence scenario per distinct real user action, with variants covered by
  ordinary scenarios.
- `givn instructions` exposes that canonical policy in the normative
  instruction source.
- Generated skills and slash commands direct the user to that source and
  contain only the choreography needed to apply it; they do not restate or
  contradict the rule.
- A project-specific override continues to replace the relevant instruction
  content when a consuming project deliberately supplies one.
- The generated guidance for specification, design, and review remains
  consistent after a skills synchronization.

## Out of Scope

- Changes to the runtime behavior of givn commands or archive gates.
- Retrieval, semantic search, or embedding support.
- New E2E scenarios in existing capabilities solely to validate the prose.
- Changes to the substantive policy itself beyond choosing one consistent
  wording and removing contradictions.
- Changes to the watn repository.

## Open Questions

- None. The canonical policy is the repository-wide rule for E2E interaction
  inventory, real-interface assertions, and one-scenario-per-action scope.
