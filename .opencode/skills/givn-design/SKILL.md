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
- Test runner: `givn missing-testrunner`

## This is the HOW layer

Everything technical belongs here — do not add implementation details to the spec.

- Technology decisions: language, framework, libraries, patterns.
- Architecture impact: which modules/components are affected, what is new.
- Data model changes: schema, structs, database changes.
- Step definition locations: where Cucumber/Gherkin step defs live.
- Runner command: confirm `verify.command` in `givn/config.yaml`.
- Justify non-obvious technical choices.

## Verify command

```
givn missing-testrunner
```
