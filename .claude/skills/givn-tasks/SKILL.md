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
- Test runner: `givn missing-testrunner`

## The TDD loop (one scenario at a time)

```
For each scenario:
  RED:      Remove @wip from THIS scenario only.
            Write step definitions WITH real assertions.
            Run: givn missing-testrunner → MUST FAIL (no production code yet).
            If it passes, assertions are not real — fix the steps first.
  GREEN:    Write minimum production code.
            Run: givn missing-testrunner → this scenario PASSES.
  REFACTOR: Clean up without changing behaviour.
            Run: givn missing-testrunner → still PASSES.
Then move to the next scenario.
```

## Important

- `givn lint` is a STATIC Gherkin check only. It does NOT run tests.
- GREEN is confirmed by `givn missing-testrunner` only, never by `givn lint`.
- Track progress: `givn apply --change <change-id>`

## Verify command

```
givn missing-testrunner
```
