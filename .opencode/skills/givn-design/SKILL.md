---
name: givn-design
description: Write the technical design for a givn change — the HOW layer with technology decisions, step definitions, and architecture impact.
---

# givn-design

Write the design for change `<change-id>`.

## Context

- Design file: `givn/changes/<change-id>/design.md`
- Instructions: run `givn instructions design --change <change-id>`
- Spec (WHAT): `givn/changes/<change-id>/specs/`
- Test runner: `cargo test --test features_runner`

## This is the HOW layer

Everything technical belongs here — do not add implementation details to the spec.

- Technology decisions: language, framework, libraries, patterns.
- Architecture impact: which modules/components are affected, what is new.
- Data model changes: schema, structs, database changes.
- Step definition locations: where Cucumber/Gherkin step defs live.
- Runner command: confirm `verify.command` in `givn/config.yaml`.
- Justify non-obvious technical choices.

## E2E smoke test infrastructure

The design MUST document the e2e test infrastructure for `@e2e` scenarios:

- **E2E runner command** → becomes `verify.e2e_command` in `givn/config.yaml`.
- **E2E step definition location** (separate from unit/integration steps).
- **Test infrastructure**: what is needed locally (browser driver, test server,
  test database, Testcontainers, fake services). How it starts/stops.
- **E2E framework choice** and justification.

## Verify command

Unit/integration:
```
cargo test --test features_runner
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
