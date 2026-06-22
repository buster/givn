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
- Test runner: read `verify.command` from `givn/config.yaml` at runtime (MUST
  be a Gherkin/Cucumber runner that executes the `.feature` files — see the
  executable-spec invariant in the design instruction; if it is still the
  `givn missing-testrunner` sentinel, running it prints setup guidance)

## This is the HOW layer

Everything technical belongs here — do not add implementation details to the spec.

- Technology decisions: language, framework, libraries, patterns.
- Architecture impact: which modules/components are affected, what is new.
- Data model changes: schema, structs, database changes.
- Step definition locations: where Cucumber/Gherkin step defs live.
- Runner command: confirm `verify.command` in `givn/config.yaml`.
- Justify non-obvious technical choices.

## Verify command

Read `verify.command` from `givn/config.yaml`. It MUST be a Gherkin/Cucumber
runner that executes the `.feature` files. If it is still the `givn
missing-testrunner` sentinel, running it prints setup guidance — follow that
guidance to configure a real runner.
