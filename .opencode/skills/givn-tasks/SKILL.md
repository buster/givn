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
- Test runner: `cargo test --test features_runner`

## The TDD loop (one scenario at a time)

```
For each scenario:
  RED:      Remove @wip from THIS scenario only.
            Write step definitions WITH real assertions.
            Run: cargo test --test features_runner → MUST FAIL (no production code yet).
            If it passes, assertions are not real — fix the steps first.
  GREEN:    Write minimum production code.
            Run: cargo test --test features_runner → this scenario PASSES.
  REFACTOR: Clean up without changing behaviour.
            Run: cargo test --test features_runner → still PASSES.
Then move to the next scenario.
```

## Important

- `givn lint` is a STATIC Gherkin check only. It does NOT run tests.
- GREEN is confirmed by `cargo test --test features_runner` only, never by `givn lint`.
- Track progress: `givn apply --change <change-id>`

## Verify command

```
cargo test --test features_runner
```
