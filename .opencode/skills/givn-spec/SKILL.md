---
name: givn-spec
description: Write Gherkin .feature delta files for a givn change — observable behaviour in domain language, no implementation detail.
---

# givn-spec

Write the Gherkin spec delta for change `<change-id>`.

## Context

- Delta spec location: `givn/changes/<change-id>/specs/<usecase-id>/<capability>.feature`
- Instructions: run `givn instructions specs --change <change-id>`
- Proposal (WHY): `givn/changes/<change-id>/proposal.md`
- Permanent specs: `givn/specs/`

## Use-case context

Before writing a delta, read the owning `givn/specs/<usecase-id>/usecase.md`
or the ideation change seed that names it. Preserve its goal, guarantees,
rules, typed relationships, Persona references, and capability interaction
inventory. Gherkin names domain actors and observable actions; it never embeds
Persona biographies. If this is a new capability, create or update the
complete change-side `usecase.md` under the use-case ID selected by routing.

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

## Canonical interaction policy

Before writing any scenario, run:

```sh
givn instructions specs --change <change-id>
```

That instruction is the normative source for the user interaction inventory,
real-interface assertions, flag-variant classification, and one-E2E-per-action
scope. This skill only choreographs when to read it and how to write the delta;
it does not restate the policy.


rerun review; do not chunk or edit scenarios automatically.

## Verify command

Unit/integration:
```
./run-tests.sh
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
