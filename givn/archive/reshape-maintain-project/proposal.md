# Proposal: reshape-maintain-project

## Use-Case Context

- Use-case ID: `maintain-project`
- Ideation topic: `multi-scm-support`
- Seed path: `givn/ideation/multi-scm-support/changes/maintain-project.md`
- Confirmed Personas: `developer--primary-user`, `givn-maintainer--owner`

## Problem / Opportunity

The 0.8.0 use-case model falsifies the current `maintain-project` document:

- **R1 goal conjunction** — the Success guarantee names two outcomes.
- **R9 oversized leaf** — the Main flow names three actor goals: inspect
  documentation, apply a managed upgrade, preserve a migration plan.
- **R3/R8 placement and naming** — `release-0-3-0` is a release container
  that advances no actor goal and names a release instead of behavior; it
  bundles managed-upgrade behavior, release metadata, and a retired-command
  notice.
- **R5 extension discipline** — all three extensions lack step references.
- **R7 mechanism vocabulary** — "active backend" appears in the Main flow
  and the Success guarantee.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 1) records these
deviations and names this follow-up.

## Proposed Solution

Promote `maintain-project` to a `+` summary and split it into three `!`
leaves (operator-approved 2026-09-18):

| Use case | Goal | Capability |
|---|---|---|
| `maintain-project` (`+`, summary) | Keep a project's documentation and managed files current across releases | none; Includes the three children |
| `keep-documentation-current` (`!`) | Keep the project's documentation complete and current | `readme-enforcement` |
| `upgrade-a-project` (`!`) | Cross a release gap with reviewable upgrade and migration revisions | `apply-a-managed-upgrade` (from `release-0-3-0`) |
| `publish-a-release` (`!`) | Publish a Givn release with truthful metadata and a migration route | `describe-the-release` (from the release metadata scenarios) |

`release-0-3-0.feature` splits into
`upgrade-a-project/apply-a-managed-upgrade.feature` (eleven scenarios) and
`publish-a-release/describe-the-release.feature` (five scenarios);
`readme-enforcement.feature` moves to `keep-documentation-current/`.

## Out of Scope

- Any behavior change; every scenario keeps its assertions.
- The other use-case reshapes and the support-fragment renames.

## Open Questions

None.
