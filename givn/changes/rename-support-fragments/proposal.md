# Proposal: rename-support-fragments

## Use-Case Context

- Use-case ID: fragments under the `support` substrate
- Ideation topic: none
- Seed path: none
- Confirmed Personas: none — the rename repairs capability names; no
  user-visible surface changes.

## Problem / Opportunity

Fourteen support-fragment capabilities name a technical component,
mechanism, or channel, or are change-shaped rather than behavior-shaped:

| Current | New |
|---|---|
| `spec-explore-change` | `route-a-change` |
| `spec-explore-permanent` | `find-existing-scenarios` |
| `repository-ownership-fixtures` | `verify-repository-ownership` |
| `llm-attribution` | `attribute-commits` |
| `llm-attribution-installation` | `enable-commit-attribution` |
| `scm-backend-selection` | `detect-the-project-repository` |
| `config-overlay` | `customize-the-lifecycle-graph` |
| `template-conditional-rendering` | `match-guidance-to-enabled-addons` |
| `domain-skill-trigger` | `guide-domain-modeling` |
| `ideation-skill` | `generate-ideation-guidance` |
| `coverage-script-workflow` | `measure-coverage-per-project` |
| `readme-coverage-badge` | `show-coverage-in-the-readme` |
| `skill-rewrites` | `refresh-generated-guidance` |
| `arc42-review-alignment` | `review-architecture-documentation` |

`config-overlay` is also orphaned: `fragment.md` does not list it and no use
case owns it; the rename restores its ownership.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 1) records these
deviations and names this follow-up.

## Proposed Solution

Rename the fourteen feature files, update their titles, rewrite
`fragment.md`'s capability list, and update the one test path that names a
renamed file.

## Out of Scope

- Any behavior change; the scenarios keep their assertions.
- The use-case reshapes (all archived).

## Open Questions

None.
