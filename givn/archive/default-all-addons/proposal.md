# Proposal: default-all-addons

## Release note

Givn enables every add-on by default and lets a project opt out explicitly, including the outline and architecture-principles add-ons.

## Use-Case Context

- Use-case ID: `initialize-project` (capability `enable-optional-addons`)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`

## Review Context

- Review kind: self-review
- Reason: cross-cutting default and CLI behavior; the default single-context review applies.

## Problem / Opportunity

`givn init` enables eight of the ten known add-ons, but `architecture_principles` and `outline` stay off until a project enables them explicitly. The result is an inconsistent default posture: most guidance and the outline report depend on add-ons a fresh project does not have, while the add-on list has to explain a mixed default-off set.

The embedded manifest also defaults every add-on to false. A project that writes a partial `addons:` block therefore silently disables every key it does not mention, and `givn init --no-addons` relies on that false default rather than writing an explicit opt-out. The default is not one posture; it is a mix of embedded defaults, a partial overlay, and init-time scaffolding.

## Proposed Solution

- Every known add-on is enabled by default.
- The embedded default manifest enables every add-on.
- An explicit `addons:` block may name only the add-ons it wants to change; a missing key resolves to enabled.
- `givn init` enables every add-on unless `--no-addons` is given.
- `givn init --no-addons` writes an explicit `false` for every known add-on, so the opt-out is visible in `givn/config.yaml` and does not depend on an embedded default.
- `givn addons list` reports the effective state: a missing key is shown as enabled, and an explicit `false` is shown as disabled.
- `givn addons disable <name>` and `givn addons enable <name>` remain the per-add-on opt-out and opt-in.
- The outline report is available in a fresh project; a project may still disable the outline add-on explicitly.
- Existing explicit `false` values stay false. The change does not rewrite a project's chosen configuration.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `enable-optional-addons` | `EXTEND givn-workflow-enforcement` | `EXTEND enable-optional-addons` | The add-on default posture belongs to project initialization and the add-on registry, not to the verification gate. The route matches shared `givn check` vocabulary, not the changed behavior. |
| `outline-report` | (no route signal) | `EXTEND outline-report` | The outline report's default availability changes with the outline add-on; its existing behavior and opt-out stay the same. |

## Out of Scope

- No add-on is removed or merged.
- No add-on's own behavior changes beyond its default enabled state.
- No migration rewrites an existing explicit `addons:` block.
- No harness, driver, model, or provider change.
- No change to the review-context or outline-identity policy.

## Open Questions

### D1 — What does a missing key in a partial `addons:` block mean?

- Question: should a project that writes `addons:\n  coverage: false` keep the other add-ons at their embedded default, or should every unnamed key become false?
- Options:
  1. **Missing means enabled** — the partial block changes only the named key; every unnamed add-on follows the all-enabled default; gain: one consistent opt-out posture and partial overrides stay small; cost: a project that intended to disable an add-on must name it.
  2. **Missing means disabled** — the block is replace-whole, so every unnamed add-on is false; gain: the block is self-contained; cost: a one-line opt-out silently disables the other nine add-ons, which is the inconsistency this change removes.
- Disposition: answered by this proposal — option 1. The design realizes it with an enabled-by-default serde default and an explicit `false` written by `givn init --no-addons`.
