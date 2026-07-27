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

## User Interaction Inventory (mandatory, before writing any scenario)

Enumerate every interaction a consumer performs against the system's black-box
boundary — before writing any scenario text. An interaction is a discrete
action that triggers an observable state change through the real interface.
What counts as "the real interface" depends on the system type:

| System type | Real interface | Inventory must include |
|---|---|---|
| **Web UI** | Rendered browser (Playwright, Selenium) | Every button click, form submission, navigation link, SSE/WS event that changes visible content. |
| **CLI / terminal** | Real subprocess invocation | Every subcommand, every flag combination that produces different output. |
| **HTTP/REST API** | Real HTTP client against deployed server | Every endpoint, every side-effect-permitting operation. |
| **Library / SDK** | Public API surface from real consumer code | Every public function or method a consumer invokes. |
| **Message/event bus** | Real producer/consumer against running services | Every message type consumed, every side-effect-producing message published. |

Record the inventory as a comment block at the very top of the `.feature`
file, before the Feature header. One entry per interaction:

```
# User Interaction Inventory:
# - <action description>
# - <action description>
```

Each inventory entry MUST have at least one matching `@e2e` scenario in the
same `.feature` file. The inventory is the exhaustive list — there is no
second-class interaction that can skip e2e coverage. If an inventory entry
has no corresponding `@e2e` scenario, the spec is incomplete — fill the gap
before moving to design.

## End-to-end smoke tests (`@e2e`)

Every capability MUST include one `@e2e` smoke-test scenario per User
Interaction Inventory entry, each exercising the real interaction surface
(browser, CLI, API, etc.). The exact number is determined by the inventory:
one `@e2e` scenario per entry, not fewer.

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
givn missing-testrunner
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
