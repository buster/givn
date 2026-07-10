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

Non-@e2e scenarios first:

```
For each non-@e2e scenario:
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

Then @e2e smoke tests (after all non-@e2e scenarios are GREEN):

```
For each @e2e scenario:
  RED:      Remove @wip. Write e2e step definitions with real assertions
            that drive the actual interface (browser, CLI, API).
            Run: verify.e2e_command → MUST FAIL.
  GREEN:    Set up test infrastructure if needed. Write minimum e2e glue.
            Run: verify.e2e_command → scenario PASSES.
  REFACTOR: Clean up e2e code. Runner still PASSES.
Then move to the next @e2e scenario.
```

## Important

- `givn lint` is a STATIC Gherkin check only. It does NOT run tests.
- GREEN for non-@e2e is confirmed by `cargo test --test features_runner` only.
- GREEN for @e2e is confirmed by `verify.e2e_command` only.
- Never conflate lint, verify.command, or verify.e2e_command.
- Track progress: `givn apply --change <change-id>`

## Verify command

Unit/integration:
```
cargo test --test features_runner
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
