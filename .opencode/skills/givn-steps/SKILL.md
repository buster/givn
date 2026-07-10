---
name: givn-steps
description: Shared step-writing mechanics for Gherkin scenarios — the RED/GREEN/REFACTOR loop with step reuse. Referenced by givn-implement and givn-characterize.
---

# givn-steps

Shared step-writing mechanics for Gherkin `.feature` scenarios. This skill is
a **reference** — it is not a standalone workflow. It is read by
`/givn-implement` and `/givn-characterize` subagents before writing step
definitions.

## Context

- Test runner: `cargo test --test features_runner`
- Features path: `GIVN_FEATURES=givn/specs`
- Change: `givn/changes/<change-id>/`

## The `.feature` file is the executable spec

The `.feature` file is the sole source of scenario identity. Step text is the
contract; step definitions are shared glue that the Gherkin runner binds to
the scenarios. If a scenario's steps change, the test surface changes
automatically. Never maintain a parallel hand-written test file that
duplicates scenario logic.

## Step definition locations

Step definitions are the bindings the Gherkin runner uses to map `.feature`
steps to code. Their location is project-specific (documented in `design.md`
for normal changes, or in `tasks.md` for reverse-engineering changes). Common
patterns:

- Rust (`cucumber-rs`): `tests/steps/mod.rs`, auto-discovered by the runner.
- Python (`behave`): `features/steps/`, auto-discovered.
- Node (`cucumber-js`): `features/steps/`, configured in `cucumber.js`.

## E2E step definition locations

`@e2e` scenarios use **separate** step definitions from unit/integration
scenarios. They drive the real interaction surface (browser, CLI, API) and
require different infrastructure (browser driver, HTTP client, test harness).

Their location is project-specific (documented in `design.md`). Common
patterns:

- Rust: `tests/e2e_steps/mod.rs`, used by a separate `e2e_runner` test binary.
- Python: `features/e2e_steps/`, with `behave --tags=@e2e`.
- Node: `features/e2e_steps/`, with `cucumber-js --tags @e2e`.
- Java: `src/test/java/.../e2e/steps/`, with Cucumber + Playwright.

## Step reuse — always check first

Before writing any new step definition, check whether a matching step
definition already exists. Step reuse across scenarios and capabilities is
desirable — it reduces duplication and keeps the glue maintainable.

- **Reused step**: keep the existing definition as-is. It already has real
  assertions (proven by prior use). Do not modify or replace it.
- **New step**: write a new step definition.

## The RED/GREEN/REFACTOR loop (one scenario at a time)

Work through tasks, one scenario at a time. Non-@e2e scenarios use
`cargo test --test features_runner`; `@e2e` scenarios use `verify.e2e_command`.

### RED — confirm the test CAN fail

1. Remove `@wip` from the current scenario only (leave it on all others).
2. For each step in the scenario, check if a matching step definition already
   exists (in the appropriate step location — unit/integration or e2e):
   - **Reused step**: keep the existing definition as-is.
   - **New step**: write a step definition with an unimplemented assertion
     (e.g. `unimplemented!()` in Rust, `assert.fail("not implemented")` in
     JS, `raise NotImplementedError` in Python).
3. Run the appropriate runner:
   - Non-@e2e scenario: `cargo test --test features_runner`
   - @e2e scenario: `verify.e2e_command`
   - **Scenario FAILS** → RED confirmed (the test is real). Proceed to GREEN.
   - **Scenario PASSES (all steps reused)** → legitimate immediate GREEN.
     The pre-existing assertions already match this scenario's behaviour.
     Proceed to GREEN confirmation (no code changes needed, just verify).
   - **Scenario passes but has no real assertions** (all new steps are
     no-ops) → the assertions are fake. Fix them before continuing.

### GREEN — implement real assertions

4. Implement real assertions for new steps (Then steps must check concrete
   values, not just that code ran).
   - **For normal TDD** (`/givn-implement`): also write the minimum
     production code to make this scenario pass.
   - **For characterization** (`/givn-characterize`): probe the observed
     production behaviour. No production code changes. If it fails, run the
     mismatch diagnostic (see givn-characterize).
   - **For @e2e scenarios**: set up test infrastructure if needed (start
     server, launch browser, seed database). Write the e2e glue that drives
     the real interaction surface.
5. Run the appropriate runner → this scenario passes; others may still fail
   (expected, they still have `@wip`).

### REFACTOR — clean up without changing behaviour

6. Refactor per the workflow's scope:
   - **Normal TDD**: general cleanup (names, duplication, clarity). No
     behaviour change.
   - **Characterization**: testability seams only (extract interface, DI,
     make public). If red, revert the refactor. If no seam is needed, skip.
   - **E2E**: clean up infrastructure code, extract shared e2e helpers.
7. Run the appropriate runner → still passes.

### Done with scenario

8. Check off the task in `tasks.md`: `- [ ]` → `- [x]`
9. Move to the next scenario.

## Lint vs test — do not confuse them

- `givn lint` is a STATIC Gherkin check (syntax, `@wip`, delta tags). It
  runs NO tests.
- `cargo test --test features_runner` confirms GREEN for non-@e2e scenarios.
- `verify.e2e_command` confirms GREEN for @e2e scenarios.
- Never conflate the three.

## Verify command

Unit/integration:
```
cargo test --test features_runner
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
