---
description: Shared step-writing mechanics for Gherkin scenarios — RED/GREEN/REFACTOR with step reuse. Reference skill for givn-implement and givn-characterize.
---

Shared step-writing mechanics for Gherkin `.feature` scenarios. This is a
**reference skill**, not a standalone workflow. It is read by `/givn-implement`
and `/givn-characterize` subagents before writing step definitions.

**This command does not execute a workflow.** It prints the shared
step-writing reference. To actually write steps as part of a change, use:

- `/givn-implement` — RED/GREEN/REFACTOR TDD loop (normal changes).
- `/givn-characterize` — RED/GREEN/REFACTOR characterization loop (reverse
  changes).

---

## When this skill is read

The `/givn-implement` and `/givn-characterize` orchestrators instruct their
subagents to read this skill before writing any step definitions. It provides
the mechanics that are identical between both workflows:

- Step reuse (check existing definitions first).
- The RED/GREEN/REFACTOR loop structure.
- Lint vs test distinction.
- Step definition location conventions.

What differs between workflows (production code rules, refactor scope,
mismatch diagnostic) is overlaid by each orchestrator's prompt — not
duplicated here.

---

## The `.feature` file is the executable spec

The `.feature` file is the sole source of scenario identity. Step text is the
contract; step definitions are shared glue that the Gherkin runner binds to
the scenarios. If a scenario's steps change, the test surface changes
automatically. Never maintain a parallel hand-written test file that
duplicates scenario logic.

## Step reuse — always check first

Before writing any new step definition, check whether a matching step
definition already exists. Step reuse across scenarios and capabilities is
desirable — it reduces duplication and keeps the glue maintainable.

- **Reused step**: keep the existing definition as-is. It already has real
  assertions (proven by prior use). Do not modify or replace it.
- **New step**: write a new step definition.

## E2E step definitions

`@e2e` scenarios use **separate** step definitions from unit/integration
scenarios. They drive the real interaction surface (browser, CLI, API) and
require different infrastructure (browser driver, HTTP client, test harness).

Step reuse within e2e steps follows the same rules: check for existing e2e
step definitions first. E2e and non-e2e step definitions are separate — do
not mix them.

## The RED/GREEN/REFACTOR loop (one scenario at a time)

### RED — confirm the test CAN fail

1. Remove `@wip` from the current scenario only (leave it on all others).
2. For each step in the scenario, check if a matching step definition already
   exists:
   - **Reused step**: keep the existing definition as-is.
   - **New step**: write a step definition with an unimplemented assertion
     (e.g. `unimplemented!()` in Rust, `assert.fail("not implemented")` in
     JS, `raise NotImplementedError` in Python).
3. Run the appropriate test runner:
   - Non-@e2e scenario: `verify.command`
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
     mismatch diagnostic (see `/givn-characterize`).
   - **For @e2e scenarios**: set up test infrastructure if needed (start
     server, launch browser, seed database). Write the e2e glue that drives
     the real interaction surface.
5. Run the appropriate test runner → this scenario passes; others may still
   fail (expected, they still have `@wip`).

### REFACTOR — clean up without changing behaviour

6. Refactor per the workflow's scope:
   - **Normal TDD**: general cleanup (names, duplication, clarity). No
     behaviour change.
   - **Characterization**: testability seams only (extract interface, DI,
     make public). If red, revert the refactor. If no seam is needed, skip.
   - **E2E**: clean up infrastructure code, extract shared e2e helpers.
7. Run the appropriate test runner → still passes.

### Done with scenario

8. Check off the task in `tasks.md`: `- [ ]` → `- [x]`
9. Move to the next scenario.

## Lint vs test — do not confuse them

- `givn lint` is a STATIC Gherkin check (syntax, `@wip`, delta tags). It
  runs NO tests.
- `verify.command` confirms GREEN for non-@e2e scenarios.
- `verify.e2e_command` confirms GREEN for @e2e scenarios.
- Never conflate lint, `verify.command`, or `verify.e2e_command`.

## Guardrails

- One scenario at a time — remove `@wip` from one scenario only.
- Step definitions are shared glue — never a parallel hand-maintained test
  file.
- Then steps must check concrete values, not just that code ran.
- Reuse existing step definitions whenever they match — do not duplicate.
