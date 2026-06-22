---
name: givn-tasks
description: Break down implementation into a scenario-by-scenario RED/GREEN/REFACTOR task list for a givn change.
---

# givn-tasks

Write the implementation task breakdown for change `<change-id>`.

## Context

- Tasks file: `givn/changes/<change-id>/tasks.md`
- Instructions: run `givn instructions tasks --change <change-id>`
- Design (HOW): `givn/changes/<change-id>/design.md`
- Test runner: read `verify.command` from `givn/config.yaml` at runtime (MUST
  be a Gherkin/Cucumber runner that executes the `.feature` files — see the
  executable-spec invariant in `design.md`; if it is still the `givn
  missing-testrunner` sentinel, running it prints setup guidance)

## The TDD loop (one scenario at a time)

```
For each scenario:
  RED:      Remove @wip from THIS scenario only.
            Write step definitions WITH real assertions.
            Run verify.command → MUST FAIL (no production code yet).
            If it passes, assertions are not real — fix the steps first.
  GREEN:    Write minimum production code.
            Run verify.command → this scenario PASSES.
  REFACTOR: Clean up without changing behaviour.
            Run verify.command → still PASSES.
Then move to the next scenario.
```

## Important

- `givn lint` is a STATIC Gherkin check only. It does NOT run tests.
- GREEN is confirmed by `verify.command` (the Gherkin runner) only, never by
  `givn lint`.
- Track progress: `givn apply --change <change-id>`

## Verify command

Read `verify.command` from `givn/config.yaml`. It MUST be a Gherkin/Cucumber
runner that executes the `.feature` files. If it is still the `givn
missing-testrunner` sentinel, running it prints setup guidance — follow that
guidance to configure a real runner before any RED phase.
