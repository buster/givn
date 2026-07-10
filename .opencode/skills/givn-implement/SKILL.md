---
name: givn-implement
description: Execute the RED/GREEN/REFACTOR TDD loop for a givn change, one scenario at a time.
---

# givn-implement

Implement change `<change-id>` using scenario-by-scenario TDD.

## Context

- Tasks: `givn/changes/<change-id>/tasks.md`
- Spec: `givn/changes/<change-id>/specs/`
- Design: `givn/changes/<change-id>/design.md`
- Test runner: `cargo test --test features_runner`
- Features path: `GIVN_FEATURES=givn/specs`

## Before starting

1. Check your progress: `givn apply --change <change-id>`
2. `givn lint` to confirm the spec is well-formed (does NOT run tests).
3. Read the `givn-steps` skill for shared step-writing mechanics (RED/GREEN/REFACTOR
   loop, step reuse, lint-vs-test distinction).

## The loop

Work through tasks.md, one scenario at a time. Non-@e2e scenarios first,
then @e2e smoke tests. The `givn-steps` skill describes the shared
RED/GREEN/REFACTOR loop.

### Non-@e2e scenarios (unit/integration)

#### RED phase
1. Remove `@wip` from the current scenario only (keep it on others).
2. Write step definitions with **real assertions** (`Then` checks concrete values).
3. Run: `cargo test --test features_runner`
   - This scenario **must fail** — you have no production code yet.
   - If it passes, the assertions are fake — fix the steps.

#### GREEN phase
4. Write the **minimum** production code to make this scenario pass.
5. Run: `cargo test --test features_runner` — this scenario passes; others still fail (expected).

#### REFACTOR phase
6. Clean up code, names, duplication — without changing behaviour.
7. Run: `cargo test --test features_runner` — still passes.

#### Done with scenario
8. Check off the task in `tasks.md`: `- [ ]` → `- [x]`
9. Move to the next scenario.

### @e2e smoke tests (after all non-@e2e are GREEN)

These test the real interaction surface (browser, CLI, API). They use
`verify.e2e_command` and e2e-specific step definitions.

#### RED phase
1. Remove `@wip` from the current `@e2e` scenario.
2. Write e2e step definitions (in the e2e step location from design.md)
   with **real assertions** that drive the actual interface.
3. Run: `verify.e2e_command` — must fail.

#### GREEN phase
4. Set up test infrastructure if needed (start server, launch browser, seed DB).
   Write the minimum e2e glue to make the scenario pass.
5. Run: `verify.e2e_command` — this scenario passes.

#### REFACTOR phase
6. Clean up e2e infrastructure code.
7. Run: `verify.e2e_command` — still passes.

#### Done with scenario
8. Check off the task in `tasks.md`: `- [ ]` → `- [x]`
9. Move to the next `@e2e` scenario.

## When all tasks are checked

Run `givn apply --change <change-id>` to confirm progress.
Then proceed to the review step.

## Verify command

Unit/integration:
```
cargo test --test features_runner
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
