---
name: givn-spec
description: Write Gherkin .feature delta files for a givn change — observable behaviour in domain language, no implementation detail.
---

# givn-spec

Write the Gherkin spec delta for change `<change-id>`.

## Context

- Delta spec location: `givn/changes/<change-id>/specs/<capability>/<capability>.feature`
- Instructions: run `givn instructions specs --change <change-id>`
- Proposal (WHY): `givn/changes/<change-id>/proposal.md`
- Permanent specs: `givn/specs/`

## Delta tags

| Tag | Scope | Meaning |
|---|---|---|
| `@givn.delta` | Feature | Marks this as a delta document |
| `@<capability>` | Feature | Capability identifier (kebab-case) |
| `@givn.added` | Scenario | Append to permanent spec (default) |
| `@givn.modified` | Scenario | Replace scenario by title |
| `@givn.removed` | Scenario | Delete scenario by title |
| `@wip` | Scenario | Step definitions not yet implemented |

## Rules

- Scenarios assert observable behaviour in domain language (Given/When/Then).
  No class names, function names, routes, DB schemas, or step mechanics.
- One scenario = one observable behaviour.
- `@givn.removed` scenarios include exactly one placeholder step.
- On archive, all `@givn.*` tags are stripped automatically.
  The `@e2e` tag is preserved (it is not a `@givn.*` tag).

## End-to-end smoke tests (`@e2e`)

Every capability MUST include at least one `@e2e` smoke-test scenario that
exercises the real interaction surface (browser, CLI, API, etc.).

- Tag `@e2e` alongside the delta tag: `@givn.added @e2e @wip`.
- `@e2e` scenarios run via `verify.e2e_command`, not `verify.command`.
- They use separate step definitions (location documented in `design.md`).
- They cover the critical happy path through the real external boundary.
- They are NOT unit tests wrapped in Gherkin, NOT in-process integration tests.
- **Scope: one `@e2e` scenario per distinct happy-path action, not per
  variant.** Do not add a second `@e2e` scenario for every input variant,
  error case, or enum/dropdown value of the same action — those are regular
  scenarios. Add another `@e2e` scenario only for a genuinely different
  user-facing action.

## Verify command

Unit/integration:
```
cargo test --test features_runner
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
